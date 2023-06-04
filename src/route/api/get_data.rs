use crate::state::CompassHeading;
use crate::state::GeolocationData;
use crate::state::OrientationData;
use crate::state::VerySimpleXYZ;
use crate::State;
use actix_web::get;
use actix_web::web;
use serde::Serialize;

#[derive(Serialize)]
pub struct GetDataResp {
    acceleration: VerySimpleXYZ,
    gyroscope: VerySimpleXYZ,
    orientation: OrientationData,
    geolocation: GeolocationData,
    compass: CompassHeading,
}

#[get("/get_data")]
pub async fn get_data(state: State) -> web::Json<GetDataResp> {
    let resp = GetDataResp {
        acceleration: {
            state.acceleration.lock().to_very_simple() - state.gravity.lock().to_very_simple()
        },
        gyroscope: { state.gyroscope.lock().to_very_simple() },
        orientation: { state.orientation.lock().clone() },
        geolocation: { state.geolocation.lock().clone() },
        compass: { state.compass.lock().clone() },
    };

    web::Json(resp)
}
