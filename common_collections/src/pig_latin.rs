fn pig_latin(word: &str) -> String {
    let mut iterator = word.chars();
    let first = if let Some(item) = iterator.next() {
        match item.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => 'h',
            _ => item,
        }
    } else {
        '\0'
    };
    let word = iterator.as_str();
    format!("{word}-{first}ay")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pig_latin_test() {
        let word = "first";
        assert_eq!(pig_latin(word), "irst-fay");
    }

    #[test]
    fn pig_latin_test_2() {
        let word = "apple";
        assert_eq!(pig_latin(word), "pple-hay");
    }
}
