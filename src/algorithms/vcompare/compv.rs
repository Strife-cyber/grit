use super::edit::Edit;
use super::difference::myers;
use super::utils::split_lines;

pub fn compare(old: &str, new: &str) -> Vec<Edit> {
    let old_lines = split_lines(old);
    let new_lines = split_lines(new);
    myers(old_lines, new_lines)
}