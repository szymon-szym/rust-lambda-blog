
use std::{ time::{SystemTime, UNIX_EPOCH}, error::Error};

use aws_sdk_dynamodb::Client;

use crate::models::{InputMessage, TemperatureMessage, MoistureMessage};



pub(crate) async fn handle_message(client: &Client, event: InputMessage, table_name: &String)->Result<String, Box<dyn Error>> {

    
    match event {
        InputMessage::TemperatureMessage(tm) => store_temp_msg(client, table_name, tm).await,
        InputMessage::MoistureMessage(mm) => store_moist_msg(client, table_name, mm).await,
    }

}


async fn store_temp_msg(client: &Client, table_name: &String, tm: TemperatureMessage) -> Result<String, Box<dyn Error>> {

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let req = client
        .put_item()
        .table_name(table_name)
        .item("sensor_id", aws_sdk_dynamodb::types::AttributeValue::S(tm.sensor_id))
        .item("message_type", aws_sdk_dynamodb::types::AttributeValue::S("TEMP_MESSAGE".to_string()))
        .item("timestamp", aws_sdk_dynamodb::types::AttributeValue::N(timestamp.to_string()))
        .item("temperature", aws_sdk_dynamodb::types::AttributeValue::N(tm.temperature.to_string()));

    req.send().await?;

    Ok("Item saved".to_string())
 
}

async fn store_moist_msg(client: &Client, table_name: &String, mm: MoistureMessage) -> Result<String, Box<dyn Error>> {

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let req = client
        .put_item()
        .table_name(table_name)
        .item("sensor_id", aws_sdk_dynamodb::types::AttributeValue::S(mm.sensor_id))
        .item("message_type", aws_sdk_dynamodb::types::AttributeValue::S("MOIST_MESSAGE".to_string()))
        .item("timestamp", aws_sdk_dynamodb::types::AttributeValue::N(timestamp.to_string()))
        .item("moisture", aws_sdk_dynamodb::types::AttributeValue::N(mm.moisture.to_string()));

    req.send().await?;

    Ok("Item saved".to_string())
 
}