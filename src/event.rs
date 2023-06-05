use std::collections::HashMap;

use aws_sdk_sns::types::MessageAttributeValue;
use uuid::Uuid;

pub struct MessageAttributes {
    event_id: Uuid,
    event_type: String,
    domain: String,
    source_event_id: Option<String>,
}

impl MessageAttributes {
    pub fn new(
        event_id: Uuid,
        event_type: String,
        domain: String,
        source_event_id: Option<String>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            domain,
            source_event_id,
        }
    }

    pub fn to_key_values(self) -> HashMap<String, MessageAttributeValue> {
        let mut map = HashMap::new();

        map.insert(
            "eventId".to_string(),
            MessageAttributeValue::builder()
                .data_type("String".to_string())
                .string_value(self.event_id.to_string())
                .build(),
        );
        map.insert(
            "eventType".to_string(),
            MessageAttributeValue::builder()
                .data_type("String".to_string())
                .string_value(self.event_type)
                .build(),
        );
        map.insert(
            "domain".to_string(),
            MessageAttributeValue::builder()
                .data_type("String".to_string())
                .string_value(self.domain)
                .build(),
        );
        map.insert(
            "source_event_id".to_string(),
            MessageAttributeValue::builder()
                .data_type("String".to_string())
                .string_value(self.source_event_id.unwrap_or("".to_string()))
                .build(),
        );

        map
    }
}
