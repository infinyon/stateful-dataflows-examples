use anyhow::Result;
use sdfg::sdf;

#[sdf(fn_name = "sentence-to-words")]
pub(crate) fn sentence_to_words(sentence: String) -> Result<Vec<String>> {
    Ok(sentence.split_whitespace().map(String::from).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sentence_to_words() {
        let sentence = "This is a test.".to_string();
        let words = sentence_to_words(sentence);
        assert_eq!(words.unwrap(), vec!["This", "is", "a", "test."])
    }
}