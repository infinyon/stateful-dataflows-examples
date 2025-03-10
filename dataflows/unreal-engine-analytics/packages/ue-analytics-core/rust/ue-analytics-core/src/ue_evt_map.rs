use sdfg::Result;
use sdfg::sdf;
use crate::bindings::fluvio::ue_analytics_core_types::types::Bytes;
#[allow(unused_imports)]
use crate::bindings::fluvio::ue_analytics_core_types::types::*;
#[sdf(fn_name = "ue-evt-map")]
pub(crate) fn ue_evt_map(msg: Bytes) -> Result<String> {
    use unreal_analytics_core::ue_aevt_map;
    match ue_aevt_map(&msg) {
        Ok(event) => {
            let string_event = serde_json::to_string(&event)?;
            Ok(string_event)
        },
        Err(e) => Err(e),
    }
}
