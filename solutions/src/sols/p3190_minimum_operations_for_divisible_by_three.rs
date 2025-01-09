struct Solution;

impl Solution {
    /// Hey! This function figures out how many numbers we need to change to make
    /// everything divisible by 3. It's pretty simple actually!
    ///
    /// Here's how it works:
    /// 1. We look at each number in the input array
    /// 2. For each number, we check:
    ///    - If it's already divisible by 3 (like 0, 3, 6, 9...), we don't need to change it
    ///    - If it's not divisible by 3, we count it as 1 operation needed
    /// 3. Finally, we add up all the operations needed
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter().map(|x| if x % 3 == 0 { 0 } else { 1 }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_operations() {
        // Test case from the example
        assert_eq!(
            Solution::minimum_operations(vec![1, 2, 3, 4, 6]),
            3 // 1, 2, and 4 need to be changed
        );

        // Test all numbers divisible by 3
        assert_eq!(
            Solution::minimum_operations(vec![3, 6, 9, 12]),
            0 // No changes needed
        );

        // Test no numbers divisible by 3
        assert_eq!(
            Solution::minimum_operations(vec![1, 2, 4, 5]),
            4 // All numbers need to be changed
        );

        // Test empty array
        assert_eq!(
            Solution::minimum_operations(vec![]),
            0 // Empty array requires no changes
        );

        // Test with negative numbers
        assert_eq!(
            Solution::minimum_operations(vec![-3, -2, -1, 0, 1, 2, 3]),
            4 // -2, -1, 1, 2 need to be changed
        );
    }
}
