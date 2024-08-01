use std::collections::HashMap;

/// Finds the maximum number of fruits that can be placed in the given 2 baskets.
/// - Each basket can have only one type of fruit. There is no limit to how many fruit a basket can hold.
/// - You can start with any tree, but you can’t skip a tree once you have started.
/// - You will pick exactly one fruit from every tree until you cannot, i.e., you will stop when you have to pick from a third fruit type.
///
/// # Arguments
///
/// * `fruits` - A vector of characters where each character represents a fruit tree.
///
/// # Returns
///
/// * The maximum number of fruits that can be placed in the given 2 baskets.
///
/// # Complexity
///
/// * Time Complexity: O(N)
///   * O(N) as each element in the vector is processed at most twice, once when traversing and another when shrinking the window.
/// * Space Complexity: O(1)
///   * Constant space as the hash map stores at most 2 + 1 elements.
pub fn find_length(fruits: Vec<char>) -> u32 {
    let mut window_start: usize = 0;
    let mut max_len: u32 = 0;
    let mut fruit_map = HashMap::new();

    for (idx, curr_fruit) in fruits.iter().enumerate() {
        fruit_map
            .entry(*curr_fruit)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        while fruit_map.len() > 2 {
            // shrink until window is valid
            let fruit_to_remove = fruits[window_start];
            fruit_map
                .entry(fruit_to_remove)
                .and_modify(|count| *count -= 1);

            if let Some(count) = fruit_map.get(&fruit_to_remove) {
                if *count == 0 {
                    fruit_map.remove(&fruit_to_remove);
                }
            }

            window_start += 1;
        }

        let curr_len = (idx - window_start + 1) as u32;

        if curr_len > max_len {
            max_len = curr_len;
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruits_into_basket_1() {
        assert_eq!(find_length(vec!['A', 'B', 'C', 'A', 'C']), 3);
    }

    #[test]
    fn test_fruits_into_basket_2() {
        assert_eq!(find_length(vec!['A', 'B', 'C', 'B', 'B', 'C']), 5);
    }
}
