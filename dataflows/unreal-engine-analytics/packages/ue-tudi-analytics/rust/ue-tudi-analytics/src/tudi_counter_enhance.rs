use sdfg::Result;
use sdfg::sdf;
use crate::bindings::fluvio::ue_tudi_analytics_types::types::AnalyticsEvent;
#[allow(unused_imports)]
use crate::bindings::fluvio::ue_tudi_analytics_types::types::*;

#[sdf(fn_name = "tudi-counter-enhance")]
pub(crate) fn tudi_counter_enhance(
    key: Option<String>,
    evt: AnalyticsEvent,
) -> Result<Vec<(Option<String>, AnalyticsEvent)>> {

    let sevt = serde_json::to_string(&evt)
        .inspect_err(|e| println!("failed to parse event 1 : {} {:?}", e, evt))?;
    let out = unreal_tudi_analytics::ue_tudi_enrich(key, sevt)
        .inspect_err(|e| println!("failed to parse event 2: {} {:?}", e, evt))?;
    let outvals = out.into_iter().filter_map(|(k, v)| {
        let evt = serde_json::from_str::<AnalyticsEvent>(&v)
            .inspect_err(|e| println!("failed to parse event 3: {} {:?}", e, evt));
        match evt {
            Ok(evt) => Some((k, evt)),
            Err(_) => None,
        }
    }).collect();
    Ok(outvals)
}
