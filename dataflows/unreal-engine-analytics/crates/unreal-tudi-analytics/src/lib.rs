use anyhow::Result;
pub use unreal_analytics_core::{AnalyticsEvent, Attribute};

const MAP_TOO_LOW: f64 = -1500.0;


// enhance the stream by adding additional counter based events by analyzing
// incoming events
pub fn ue_tudi_enrich(
    key: Option<String>,
    analytics_event_string: String,
) -> Result<Vec<(Option<String>, String)>> {
    let evt = serde_json::from_str::<AnalyticsEvent>(&analytics_event_string)?;
    let mut out = vec![(key.clone(), analytics_event_string.clone())];

    // conditionally add enhanced events
    match evt.event_name.as_str() {
        "sampleLocation" => {
            if let Ok(mapy) = get_attr_f64(&evt, "mapy") {
                if mapy < MAP_TOO_LOW {
                    let fault_event = make_map_fault("off map, sample map y too low", &evt);
                    let fe_str = serde_json::to_string(&fault_event)?;
                    out.push((key, fe_str));
                }
            }
        }
        "jump" => {
            if let Ok(mapy) = get_attr_f64(&evt, "mapY") {
                if mapy < MAP_TOO_LOW {
                    let fault_event = make_map_fault("off map, jump y too low", &evt);
                    let fe_str = serde_json::to_string(&fault_event)?;
                    out.push((key, fe_str));
                }
            }
        }
        _ => {}
    }
    Ok(out)
}

fn get_attr_f64(evt: &AnalyticsEvent, key: &str) -> Result<f64, String> {
    let Some(mapy_attr) = evt.get_attr(key) else {
        return Err(format!("missing attribute: {}", key));
    };
    let mapy = mapy_attr
        .parse::<f64>()
        .map_err(|e| format!("failed to parse mapy: {}", e))?;
    Ok(mapy)
}

/// create a new map fault event with the given reason as an attribute
fn make_map_fault(reason: &str, evt_parent: &AnalyticsEvent) -> AnalyticsEvent {
    const FAULT_EVENT_NAME: &str = "mapFault";
    const FAULT_REASON_ATTR_KEY: &str = "fault_reason";
    const ORIGINAL_EVENT_ATTR_KEY: &str = "original_event";

    let mut attributes = evt_parent.attributes.clone();

    let attr = Attribute {
        key: FAULT_REASON_ATTR_KEY.to_owned(),
        val: reason.to_owned(),
    };
    attributes.push(attr);

    let attr = Attribute {
        key: ORIGINAL_EVENT_ATTR_KEY.to_owned(),
        val: evt_parent.event_name.clone(),
    };
    attributes.push(attr);

    AnalyticsEvent {
        event_name: FAULT_EVENT_NAME.to_owned(),
        attributes,
    }
}
