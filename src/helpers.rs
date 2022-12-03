/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */
pub fn split_lines<'a>(input: &'a str) -> Box<dyn Iterator<Item = &str> + 'a> {
    let split = input
        .lines() // Split by line
        .filter(|line| !line.is_empty()); // Remove empty lines
    Box::new(split)
}

pub fn str_as_chars(input: &str) -> Vec<char> {
    input.chars()
        .into_iter()
        .collect()
}
