use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Structure matching the JSON format of the analytics events,
#[derive(Debug, Deserialize, Serialize)]
pub struct InAnalyticsEvent {
    #[serde(rename = "eventName")]
    pub event_name: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attribute {
    pub key: String,
    pub val: String,
}

/// Structure matching the JSON format of the analytics events,
/// compatible w/ sdf (sdf-beta8)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnalyticsEvent {
    pub event_name: String,
    pub attributes: Vec<Attribute>, // compat w/ wit
}

impl AnalyticsEvent {
    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attributes.iter().find(|attr| attr.key == key).map(|attr| attr.val.as_str())
    }
}

impl From<InAnalyticsEvent> for AnalyticsEvent {
    fn from(event: InAnalyticsEvent) -> Self {
        let attributes = event
            .attributes
            .into_iter()
            .map(|(key, val)| Attribute { key, val })
            .collect();
        AnalyticsEvent {
            event_name: event.event_name,
            attributes,
        }
    }
}
