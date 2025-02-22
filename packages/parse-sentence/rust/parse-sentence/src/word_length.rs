use sdfg::anyhow::Result;
use sdfg::sdf;

#[sdf(fn_name = "word-length")]
pub(crate) fn word_length(word: String) -> Result<u32> {
    Ok(word.chars().count() as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_length() {
        let input = "Hello".to_string();
        let output = word_length(input);
        assert_eq!(output.unwrap(), 5);
    }
}
