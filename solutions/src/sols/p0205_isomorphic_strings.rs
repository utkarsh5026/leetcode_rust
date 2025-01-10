use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Hey! This function checks if two strings are isomorphic - meaning we can map
    /// characters from one string to another while keeping the pattern the same.
    /// Each character must map to a unique character (no two characters can map to the same one)!
    ///
    /// Here's how it works:
    /// 1. First, check if the strings are the same length (if not, can't be isomorphic)
    /// 2. Then we use two maps to track character mappings:
    ///    - One map for s → t mappings
    ///    - One map for t → s mappings (to ensure one-to-one mapping)
    ///
    ///         
    ///
    /// s = "egg", t = "add"
    ///
    /// Steps:
    /// 1. 'e' → 'a' (new mapping)
    /// 2. 'g' → 'd' (new mapping)
    /// 3. 'g' → 'd' (matches existing mapping) ✅
    ///
    /// Result: true (they're isomorphic!)
    ///
    /// But for s = "foo", t = "bar":
    /// 1. 'f' → 'b' (new mapping)
    /// 2. 'o' → 'a' (new mapping)
    /// 3. 'o' → 'r' (conflicts with 'o' → 'a') ❌
    ///
    /// Result: false
    ///
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        Self::check_isomorphism(s.chars().collect(), t.chars().collect())
    }

    /// Checks if two character vectors are isomorphic by maintaining
    /// bidirectional mappings between characters
    fn check_isomorphism(s: Vec<char>, t: Vec<char>) -> bool {
        let vec_size = s.len();
        let mut s_to_t: HashMap<char, char> = HashMap::new();
        let mut t_to_s: HashMap<char, char> = HashMap::new();

        for i in 0..vec_size {
            let s_ch = s[i];
            let t_ch = t[i];

            match (s_to_t.get(&s_ch), t_to_s.get(&t_ch)) {
                (None, None) => {
                    s_to_t.insert(s_ch, t_ch);
                    t_to_s.insert(t_ch, s_ch);
                }
                (Some(&mapped_t), Some(&mapped_s)) => {
                    if mapped_t != t_ch || mapped_s != s_ch {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        // Test case from the example
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));

        // Test case where strings are not isomorphic
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));

        // Test with empty strings
        assert!(Solution::is_isomorphic("".to_string(), "".to_string()));

        // Test with single characters
        assert!(Solution::is_isomorphic("a".to_string(), "b".to_string()));

        // Test with same characters mapping to different ones
        assert!(!Solution::is_isomorphic(
            "badc".to_string(),
            "baba".to_string()
        ));

        // Test with different characters mapping to same one
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));

        // Test with repeated characters
        assert!(Solution::is_isomorphic(
            "abba".to_string(),
            "cddc".to_string()
        ));
    }
}
