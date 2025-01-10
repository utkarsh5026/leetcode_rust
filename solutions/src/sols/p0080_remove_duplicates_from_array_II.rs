struct Solution;

impl Solution {
    /// Hey! This function removes duplicates from a sorted array, but with a twist:
    /// we allow each number to appear up to twice! It modifies the array in-place
    /// and returns the length of the modified array.
    ///
    /// Here's how it works:
    /// 1. We use two pointers:
    ///    - 'start': scans through the array looking for duplicates
    ///    - 'idx': keeps track of where to put the next number
    ///
    /// 2. For each number we find:
    ///    - Always keep the first occurrence
    ///    - If there's a second occurrence, keep that too
    ///    - Skip any additional occurrences
    ///
    /// For example:
    ///
    /// nums = [1,1,1,2,2,3]
    ///
    /// Steps:
    /// - For 1: keep first two (1,1), skip third
    /// - For 2: keep both (2,2)
    /// - For 3: keep it (3)
    ///
    /// Result: [1,1,2,2,3,_] with length 5
    ///
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut start = 0;
        let mut idx = 0;

        while start < nums.len() {
            let curr = nums[start];
            // Keep first occurrence
            nums[idx] = curr;
            idx += 1;
            start += 1;

            // Check for and handle second occurrence
            if start < nums.len() && nums[start] == curr {
                nums[idx] = curr;
                idx += 1;
                start += 1;

                // Skip any additional occurrences
                while start < nums.len() && nums[start] == curr {
                    start += 1;
                }
            }
        }
        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        // Test case from the example
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(&nums[0..result as usize], &[1, 1, 2, 2, 3]);

        // Test with more duplicates
        let mut nums = vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 7);
        assert_eq!(&nums[0..result as usize], &[0, 0, 1, 1, 2, 2, 3]);

        // Test with no duplicates
        let mut nums = vec![1, 2, 3, 4, 5];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(&nums[0..result as usize], &[1, 2, 3, 4, 5]);

        // Test with empty array
        let mut nums = vec![];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 0);

        // Test with single element
        let mut nums = vec![1];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 1);
        assert_eq!(&nums[0..result as usize], &[1]);

        // Test with all same elements
        let mut nums = vec![1, 1, 1, 1, 1];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 2);
        assert_eq!(&nums[0..result as usize], &[1, 1]);

        // Test with exactly two duplicates
        let mut nums = vec![1, 1, 2, 2, 3, 3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 6);
        assert_eq!(&nums[0..result as usize], &[1, 1, 2, 2, 3, 3]);
    }
}
