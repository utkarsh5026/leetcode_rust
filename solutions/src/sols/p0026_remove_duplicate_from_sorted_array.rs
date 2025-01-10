struct Solution;

impl Solution {
    /// Hey! This function removes duplicates from a sorted array in-place.
    /// It returns the length of the array with unique elements.
    ///
    /// Here's how it works:
    /// 1. We use two pointers:
    ///    - 'start': scans through the array looking for duplicates
    ///    - 'idx': keeps track of where to put the next unique number
    ///
    /// 2. For each position:
    ///    - Skip over any duplicates of the current number
    ///    - Put the unique number in its new position
    ///    - Move both pointers forward
    ///
    /// For example:
    ///
    /// nums = [1,1,2,2,3]
    ///
    /// Steps:
    /// - Find 1: put at index 0, skip duplicate
    /// - Find 2: put at index 1, skip duplicate
    /// - Find 3: put at index 2
    ///
    /// Result: [1,2,3,_,_] with length 3
    ///
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut start = 0;
        let mut idx = 0;

        while start < nums.len() {
            while start + 1 < nums.len() && nums[start] == nums[start + 1] {
                start += 1
            }

            if start == nums.len() {
                break;
            }

            nums[idx] = nums[start];
            idx += 1;
            start += 1;
        }
        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        // Test basic case
        let mut nums = vec![1, 1, 2];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 2);
        assert_eq!(&nums[0..result as usize], &[1, 2]);

        // Test with more duplicates
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
        assert_eq!(&nums[0..result as usize], &[0, 1, 2, 3, 4]);

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
        assert_eq!(result, 1);
        assert_eq!(&nums[0..result as usize], &[1]);
    }
}
