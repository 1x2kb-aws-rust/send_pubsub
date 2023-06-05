mod event;
mod vehicle;

use aws_sdk_sns::{
    self,
    config::Region,
    error::SdkError,
    operation::publish::{PublishError, PublishOutput},
    types::{builders::MessageAttributeValueBuilder, MessageAttributeValue},
    Client,
};
use base64::{self, engine::general_purpose, Engine};
use uuid::Uuid;
use vehicle::{Random, Vehicle};

async fn insert_vehicle_event(
    client: &Client,
    topic: &str,
    vehicle: Option<Vehicle>,
) -> Result<PublishOutput, SdkError<PublishError>> {
    let vehicle = vehicle.unwrap_or_else(|| Vehicle::random());
    let data = serde_json::to_string(&vehicle).expect("Failed to serialize vehicle");
    let mut base_64_encoded = String::new();
    general_purpose::STANDARD.encode_string(data, &mut base_64_encoded);

    client
        .publish()
        .topic_arn(topic)
        .message(base_64_encoded)
        .set_message_attributes(Some(
            event::MessageAttributes::new(
                Uuid::new_v4(),
                "insert_vehicle".to_string(),
                "vehicle".to_string(),
                None,
            )
            .to_key_values(),
        ))
        .send()
        .await
}

#[tokio::main]
async fn main() {
    let region = std::env::var("REGION")
        .map(Region::new)
        .unwrap_or(Region::new("us-east-2"));
    let topic = std::env::var("TOPIC").expect("No topic was provided!");

    let shared_config = aws_config::from_env().region(region).load().await;
    let client = Client::new(&shared_config);

    let response = insert_vehicle_event(&client, &topic, None)
        .await
        .expect("Failed to send send vehicle");

    println!("{:#?}", response);
}
