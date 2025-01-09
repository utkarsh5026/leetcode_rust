pub struct Solution;

impl Solution {
    /// Hey! This function calculates a score for a string in a pretty simple way:
    /// we just look at each pair of neighboring characters and add up their differences.
    ///
    /// Here's how it works:
    /// 1. First, we turn the string into ASCII numbers (like 'a' becomes 97)
    /// 2. Then we look at two characters at a time (like a sliding window)
    /// 3. For each pair, we:
    ///    - Find how far apart they are (like 'b' - 'a' = 1)
    ///    - Make sure it's positive (we don't care if it's going up or down)
    /// 4. Finally, we add up all these differences
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|w| (w[1] as i32 - w[0] as i32).abs())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_string() {
        assert_eq!(Solution::score_of_string("abcd".to_string()), 3);

        assert_eq!(Solution::score_of_string("aaa".to_string()), 0);

        assert_eq!(
            Solution::score_of_string("ababab".to_string()),
            1 + 1 + 1 + 1 + 1
        );

        assert_eq!(Solution::score_of_string("az".to_string()), 25);

        assert_eq!(Solution::score_of_string("aA".to_string()), 32);

        assert_eq!(Solution::score_of_string("a".to_string()), 0);
    }
}
