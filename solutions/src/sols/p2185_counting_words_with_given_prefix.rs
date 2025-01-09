struct Solution;

impl Solution {
    /// Hey! This function counts how many words in a list start with a given prefix.
    /// It's super straightforward!
    ///
    /// Here's how it works:
    /// 1. We look at each word in our list
    /// 2. Check if it starts with our prefix
    /// 3. Count up all the matches
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|x| x.starts_with(&pref)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_count() {
        // Test basic case
        assert_eq!(
            Solution::prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            ),
            2 // "attention" and "attend"
        );

        // Test with no matches
        assert_eq!(
            Solution::prefix_count(
                vec!["hello".to_string(), "world".to_string()],
                "xyz".to_string()
            ),
            0
        );

        // Test with empty prefix
        assert_eq!(
            Solution::prefix_count(
                vec!["test".to_string(), "testing".to_string()],
                "".to_string()
            ),
            2 // Empty prefix matches everything
        );

        // Test with empty word list
        assert_eq!(Solution::prefix_count(vec![], "test".to_string()), 0);

        // Test with prefix same length as word
        assert_eq!(
            Solution::prefix_count(
                vec!["cat".to_string(), "dog".to_string(), "cat".to_string()],
                "cat".to_string()
            ),
            2 // Both "cat" words match
        );
    }
}
