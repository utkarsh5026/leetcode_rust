struct Solution;

impl Solution {
    /// Hey! This function removes all instances of a specific value from an array.
    /// The cool part is we do it in-place and keep track of the remaining elements!
    ///
    /// Here's how it works:
    /// 1. First, we do a quick check - if the array is empty, we're done!
    /// 2. We use two pointers:
    ///    - 'start': moves from left to right looking for values to remove
    ///    - 'end': moves from right to left looking for values to keep
    /// 3. When we find a value to remove:
    ///    - We look at the end of the array for a value we want to keep
    ///    - Swap it with the value we want to remove
    ///    - Keep track of how many valid numbers we have
    ///
    /// For example:
    ///
    /// nums = [3,2,2,3], val = 3
    ///
    /// Steps:
    /// - Start finds 3 at index 0
    /// - End finds 2 at index 2
    /// - Swap them: [2,2,2,3]
    /// - Start moves right, End moves left
    /// - Final result: [2,2,_,_] with length 2
    ///
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut end = nums.len();
        let mut start = 0;
        let mut remaining = 0;

        while start < end {
            if nums[start] == val {
                // Skip values we want to remove at the end
                while start < end && end > 0 && nums[end - 1] == val {
                    end -= 1;
                }

                if start >= end || end == 0 {
                    break;
                }

                // Swap with a good value from the end
                nums[start] = nums[end - 1];
                end -= 1;
                remaining += 1;
                start += 1;
            } else {
                remaining += 1;
                start += 1;
            }
        }

        remaining
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        // Test case from the example
        let mut nums = vec![3, 2, 2, 3];
        let result = Solution::remove_element(&mut nums, 3);
        assert_eq!(result, 2);
        assert_eq!(&nums[0..result as usize], &[2, 2]);

        // Test with empty array
        let mut nums = vec![];
        let result = Solution::remove_element(&mut nums, 1);
        assert_eq!(result, 0);

        // Test with all elements being the target
        let mut nums = vec![1, 1, 1, 1];
        let result = Solution::remove_element(&mut nums, 1);
        assert_eq!(result, 0);

        // Test with no elements being the target
        let mut nums = vec![4, 5, 6, 7];
        let result = Solution::remove_element(&mut nums, 1);
        assert_eq!(result, 4);
        assert_eq!(&nums[0..result as usize], &[4, 5, 6, 7]);

        // Test with single element
        let mut nums = vec![1];
        let result = Solution::remove_element(&mut nums, 1);
        assert_eq!(result, 0);
    }
}
