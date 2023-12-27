use serde;
use super::input::Input;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Platform {
    input_devices: Vec<InputDevice>
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InputDevice {
    inputs: Vec<Input>
}

