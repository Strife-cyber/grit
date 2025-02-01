/// Takes text read from a text file and returns each line as a vector.
///
/// # Arguments
/// * `text` - A string slice that holds the content of the text file.
///
/// # Returns
/// A vector of `String`s, each representing a line from the input text.
///
/// # Examples
/// ```
/// let text = "line one\n line two\n line three";
/// let lines = split_lines(text);
/// assert_eq!(lines, vec!["line one", "line two", "line three"]);
/// ```
pub fn split_lines(text: &str) -> Vec<String> {
    text.lines().map(|line| line.trim().to_string()).collect()
}
