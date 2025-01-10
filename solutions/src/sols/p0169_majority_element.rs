use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Hey! This function finds the majority element in an array - that's the number
    /// that appears more than n/2 times (where n is the array length).
    ///
    /// Here's how it works:
    /// 1. Create a HashMap to count how many times each number appears
    /// 2. Calculate what counts as a majority (length/2)
    /// 3. Count each number's occurrences
    /// 4. Find the number that appears more than length/2 times
    ///
    /// For example:
    ///     
    /// nums = [2,2,1,1,1,2,2]
    ///
    /// Steps:
    /// - Length is 7, so majority needs > 7/2 = 3 occurrences
    /// - Count occurrences: {2: 4, 1: 3}
    /// - 2 appears 4 times (> 3), so it's the majority element
    ///
    /// Result: 2
    ///
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let major_cnt: i32 = (nums.len() / 2).try_into().unwrap();

        // Count occurrences of each number
        for num in nums {
            *cnt.entry(num).or_insert(0) += 1;
        }

        // Find the number that appears more than n/2 times
        for (num, cnt) in cnt {
            if cnt > major_cnt {
                return num;
            }
        }
        unreachable!() // Problem guarantees a majority element exists
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        // Test case from the example
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);

        // Test with minimal array (single element)
        assert_eq!(Solution::majority_element(vec![1]), 1);

        // Test with all same elements
        assert_eq!(Solution::majority_element(vec![5, 5, 5, 5]), 5);

        // Test with negative numbers
        assert_eq!(Solution::majority_element(vec![-1, -1, -1, 2, 2]), -1);

        // Test with exactly n/2 + 1 occurrences
        assert_eq!(Solution::majority_element(vec![1, 1, 1, 2, 2]), 1);
    }
}
