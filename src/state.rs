use crate::config::Config;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{ops::Sub, path::PathBuf, sync::Arc};

#[derive(Serialize)]
pub struct VerySimpleXYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl Sub for VerySimpleXYZ {
    type Output = VerySimpleXYZ;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SimpleXYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub timestamp: i64,
}

impl SimpleXYZ {
    pub fn to_very_simple(&self) -> VerySimpleXYZ {
        VerySimpleXYZ {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
    pub fn update_with_delta(&mut self, new: &SimpleXYZ) {
        let SimpleXYZ { x, y, z, timestamp } = new;

        if self.timestamp == 0 {
            self.timestamp = *timestamp;
        } else {
            // Needs updating
            let time_delta = (*timestamp - self.timestamp) as f64 / 1000.0;
            self.x += time_delta * *x;
            self.y += time_delta * *y;
            self.z += time_delta * *z;
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone, Serialize)]
pub struct OrientationData {
    qx: f64,
    qy: f64,
    qz: f64,
    qw: f64,
    pitch: f64,
    roll: f64,
    yaw: f64,
    timestamp: f64,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct GeolocationCoords {
    latitude: f64,
    longitude: f64,
    accuracy: f64,
    altitude: Option<f64>,
    heading: Option<f64>,
    speed: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct GeolocationData {
    coords: GeolocationCoords,
    timestamp: i64,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct CompassHeading {
    pub heading: f64,
    pub accuracy: f64,
}

pub struct AppState {
    pub config: Arc<Config>,
    pub base_dir: PathBuf,
    pub web_dir: PathBuf,
    pub gyroscope: Arc<Mutex<SimpleXYZ>>,
    pub acceleration: Arc<Mutex<SimpleXYZ>>,
    pub orientation: Arc<Mutex<OrientationData>>,
    pub gravity: Arc<Mutex<SimpleXYZ>>,
    pub geolocation: Arc<Mutex<GeolocationData>>,
    pub compass: Arc<Mutex<CompassHeading>>,
}

impl AppState {
    pub fn set_acceleration(&self, acc: SimpleXYZ) {
        *self.acceleration.lock() = acc;
    }
    pub fn add_gyroscope(&self, gs: &SimpleXYZ) {
        let mut guard = self.gyroscope.lock();
        guard.update_with_delta(gs);
    }
    pub fn set_orientation(&self, gs: OrientationData) {
        let mut guard = self.orientation.lock();
        *guard = gs;
    }
    pub fn set_gravity(&self, g: SimpleXYZ) {
        *self.gravity.lock() = g;
    }
    pub fn set_geolocation(&self, g: GeolocationData) {
        *self.geolocation.lock() = g;
    }
    pub fn set_compass(&self, c: CompassHeading) {
        *self.compass.lock() = c;
    }
}
