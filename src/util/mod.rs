mod string_util;
pub use string_util::*;

pub fn distinct<T: PartialEq>(vector: &mut Vec<T>) -> &mut Vec<T> {
    let mut i = 1; // Skip first element, as each element will only be compared against previous elements

    'outer:
    while i < vector.len() {
        let value = &vector[i];
        let mut j = 0;

        while j < i {
            let previous = &vector[j];

            if value == previous {
                vector.remove(i);
                continue 'outer; // Skips the i += 1, so it looks at the value at the same position, which is now the one that was previously next
            }

            j += 1;
        }

        i += 1;
    }

    vector
}
