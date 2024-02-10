pub fn get_definition(word: &str) -> String {
    match webster::dictionary(word) {
        Some(dfn) => dfn.to_string(),
        None      => format!("Sorry, no definition found for {word}."),
    }
}