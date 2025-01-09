pub struct Solution;

impl Solution {
    /// Hey! This function does something pretty straightforward - it takes an array
    /// and creates a new one that's double the size by repeating the original array.
    ///
    /// Here's how it works:
    /// 1. First, we see how long the input array is (let's call it n)
    /// 2. We make a new array that's twice as big (2 * n) and fill it with zeros
    /// 3. Then we copy the numbers over in a clever way:
    ///    - Put each number in its original position (0 to n-1)
    ///    - AND also put it in the second half (n to 2n-1)
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut v = vec![0; 2 * n];

        for i in 0..n {
            v[i] = nums[i];
            v[i + n] = nums[i];
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_concatenation() {
        // Test basic case
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );

        // Test single element
        assert_eq!(Solution::get_concatenation(vec![1]), vec![1, 1]);

        // Test empty array
        assert_eq!(Solution::get_concatenation(vec![]), vec![]);

        // Test with negative numbers
        assert_eq!(
            Solution::get_concatenation(vec![-1, 0, 1]),
            vec![-1, 0, 1, -1, 0, 1]
        );

        // Test larger array
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5]
        );
    }
}
