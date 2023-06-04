use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use anyhow::{anyhow, bail, Context};
use flexi_logger::{DeferredNow, Duplicate, FileSpec, Logger, TS_DASHES_BLANK_COLONS_DOT_BLANK};
use log::{info, Record};
use parking_lot::Mutex;
use state::{AppState, CompassHeading, GeolocationData, OrientationData, SimpleXYZ};

use crate::config::Config;

pub type State = web::Data<AppState>;

pub mod config;
pub mod route;
pub mod state;
pub fn my_log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] {} [{}:{}] {}",
        now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args()
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if !std::path::Path::new("config.yaml").exists() {
        tokio::fs::write(
            "config.yaml",
            serde_yaml::to_string(&Config::default())?.as_bytes(),
        )
        .await?;
        bail!("已创建默认配置文件，请修改后重启。");
    }

    let config = Arc::new(
        serde_yaml::from_str::<Config>(
            &tokio::fs::read_to_string("config.yaml")
                .await
                .with_context(|| anyhow!("读取配置文件错误"))?,
        )
        .with_context(|| anyhow!("反序列化错误"))?,
    );

    Logger::try_with_env_or_str(&config.log_level)
        .map_err(|_| anyhow!("Invalid logging level: {}", config.log_level))?
        .format(my_log_format)
        .log_to_file(FileSpec::default().directory("logs").basename("iot-web"))
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .map_err(|e| anyhow!("Failed to start logger!\n{}", e))?;
    let base_dir = std::env::current_dir().unwrap();
    let app_state = web::Data::new(AppState {
        base_dir: base_dir.clone(),
        web_dir: base_dir.join("web"),
        config: config.clone(),
        gyroscope: Arc::new(Mutex::new(SimpleXYZ::default())),
        acceleration: Arc::new(Mutex::new(SimpleXYZ::default())),
        orientation: Arc::new(Mutex::new(OrientationData::default())),
        gravity: Arc::new(Mutex::new(SimpleXYZ::default())),
        geolocation: Arc::new(Mutex::new(GeolocationData::default())),
        compass: Arc::new(Mutex::new(CompassHeading::default())),
    });
    {
        let web_dir = app_state.web_dir.as_path();
        if !web_dir.exists() {
            info!("Missing web directory, creating..");
            std::fs::create_dir(web_dir)?;
        }
        let index_html = web_dir.join("index.html");
        if !index_html.exists() {
            let index_bytes = include_bytes!("index.html");
            tokio::fs::write(index_html, index_bytes).await?;
        }
    }

    let bind = (config.bind_host.clone(), config.bind_port);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(actix_web::middleware::Logger::new(
                r#"%a,%{r}a "%r" %s %b %T"#,
            ))
            .service(route::index)
            .service(route::compass_png)
            .service(route::api::make_scope())
    })
    .bind(bind)?
    .run()
    .await?;
    Ok(())
}
