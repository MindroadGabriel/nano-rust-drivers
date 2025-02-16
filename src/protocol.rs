use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum ControllerEvent {
    Connected,
    HardwareFailure,
    NewData {
        x: f32,
        y: f32,
        z: f32,
        temperature: f32,
    },
    CalibrationStarted,
    CalibrationEnded,
    ButtonOne,
    ButtonTwo,
}
