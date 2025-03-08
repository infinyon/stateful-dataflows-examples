use anyhow::Result;

mod types;
use types::InAnalyticsEvent;
pub use types::AnalyticsEvent;
pub use types::Attribute;


/// parse unreal engine analytics event
pub fn ue_aevt_map(bytes: &[u8]) -> Result<AnalyticsEvent> {
    let in_event: InAnalyticsEvent = match serde_json::from_slice(bytes) {
        Ok(event) => event,
        Err(e) => {
            let msg = format!("Failed to parse JSON: {:?}", e);
            println!("ue_aevt_map: error: {}", msg);
            return Err(anyhow::anyhow!(msg));
        }
    };
    let out_event = types::AnalyticsEvent::from(in_event);
    Ok(out_event)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str; // Add this import to convert bytes to string

    #[test]
    fn test_parse() {
        const SAMPLE_JSONL: &str = "../../samples/analysis-events-proc.jsonl";
        // read SAMPLE_JSONL and split into lines
        let lines = std::fs::read_to_string(SAMPLE_JSONL).expect("Failed to read sample file");
        let lines: Vec<&str> = lines.lines().collect();

        let mut outerrs = Vec::new();
        for line in lines.iter() {
            let bytes = line.as_bytes();
            let res = ue_aevt_map(bytes);
            if let Err(e) = res {
                let msg = str::from_utf8(bytes).expect("Failed to convert bytes to string");
                let errmsg = format!("Error {:?} parsing {msg}", e);
                outerrs.push(errmsg);
            }
        }
        eprintln!("{}", outerrs.join("\n"));
        assert_eq!(outerrs.len(), 0);
    }
}
