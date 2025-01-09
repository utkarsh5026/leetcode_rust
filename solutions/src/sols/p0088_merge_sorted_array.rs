struct Solution;

impl Solution {
    /// Hey! This function merges two sorted arrays into one. The trick is that we're
    /// doing it in-place, working backwards from the end to avoid overwriting values!
    ///
    /// Here's how it works:
    /// 1. First, we check if nums2 is empty - if it is, we're already done!
    /// 2. Then we set up three pointers:
    ///    - n1_end: points to the last actual number in nums1
    ///    - n2_end: points to the last number in nums2
    ///    - final_end: points to the last position in the final array
    ///
    /// 3. Working backwards, we:
    ///    - Compare the last numbers from both arrays
    ///    - Take the bigger one and put it at the end
    ///    - Move the pointers accordingly
    ///
    /// For example:
    ///
    /// nums1 = [1,3,5,0,0,0], m = 3
    /// nums2 = [2,4,6], n = 3
    ///
    /// Steps:
    /// - Compare 5 and 6: put 6 at the end
    /// - Compare 5 and 4: put 5 at the end
    /// - Compare 3 and 4: put 4 at the end
    /// And so on...
    ///
    /// Result: [1,2,3,4,5,6]
    ///
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let mut n1_end = m as usize;
        let mut n2_end = n as usize;
        let mut final_end = (m + n) as usize;

        while n2_end > 0 {
            if n1_end > 0 && nums1[n1_end - 1] >= nums2[n2_end - 1] {
                nums1[final_end - 1] = nums1[n1_end - 1];
                n1_end -= 1;
            } else {
                nums1[final_end - 1] = nums2[n2_end - 1];
                n2_end -= 1;
            }
            final_end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        // Test case from the example
        let mut nums1 = vec![1, 3, 5, 0, 0, 0];
        let mut nums2 = vec![2, 4, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);

        // Test when nums2 is empty
        let mut nums1 = vec![1, 2, 3];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 3, &mut nums2, 0);
        assert_eq!(nums1, vec![1, 2, 3]);

        // Test when nums1 is effectively empty
        let mut nums1 = vec![0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        Solution::merge(&mut nums1, 0, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3]);

        // Test with negative numbers
        let mut nums1 = vec![-1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![-3, -2, 0];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![-3, -2, -1, 0, 2, 3]);

        // Test with duplicate numbers
        let mut nums1 = vec![1, 1, 2, 0, 0];
        let mut nums2 = vec![1, 2];
        Solution::merge(&mut nums1, 3, &mut nums2, 2);
        assert_eq!(nums1, vec![1, 1, 1, 2, 2]);
    }
}
