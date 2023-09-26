use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureMessage {
    pub sensor_id: String,
    pub temperature: f32
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoistureMessage {
    pub sensor_id: String,
    pub moisture: i32
}
#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InputMessage {
    TemperatureMessage(TemperatureMessage),
    MoistureMessage(MoistureMessage)
}

#[derive(Serialize)]
pub struct Response {
    pub statusCode: i32,
    pub body: String,
}
