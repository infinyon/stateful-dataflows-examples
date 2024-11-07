use anyhow::Result;
use sdfg::sdf;

use std::sync::LazyLock;
use regex::Regex;

static SSN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d{3}-\d{2}-\d{4}").unwrap());

#[sdf(fn_name = "mask-ssn")]
pub(crate) fn mask_ssn(text: String) -> Result<String> {
    let output = SSN_RE.replace_all(text.as_str(), "***-**-****").to_string();
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mask_ssn() {
        let input = "123-45-6789".to_string();
        let output = mask_ssn(input);
        assert_eq!(output.unwrap(), "***-**-****");
    }
}