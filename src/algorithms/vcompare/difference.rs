use super::edit::Edit;

/// Computes the differences between two sequences of strings using a simplified Myers diff algorithm.
///
/// # Arguments
/// * `old` - The original sequence of strings.
/// * `new` - The new sequence of strings.
///
/// # Returns
/// A vector of `Edit` operations that transform `old` into `new`.
pub fn myers(old: Vec<String>, new: Vec<String>) -> Vec<Edit> {
    let mut edits = Vec::new();
    let mut old_index = 0;
    let mut new_index = 0;

    while old_index < old.len() && new_index < new.len() {
        if old[old_index] == new[new_index] {
            // Lines are the same, move to the next pair
            old_index += 1;
            new_index += 1;
        } else {
            // Lines are different, push a Replace edit
            edits.push(Edit::Replace(new_index, new[new_index].clone()));
            old_index += 1;
            new_index += 1;
        }
    }

    // Handle remaining lines in the old sequence (deletions)
    while old_index < old.len() {
        edits.push(Edit::Delete(old_index));
        old_index += 1;
    }

    // Handle remaining lines in the new sequence (insertions)
    while new_index < new.len() {
        edits.push(Edit::Insert(new_index, new[new_index].clone()));
        new_index += 1;
    }

    edits
}