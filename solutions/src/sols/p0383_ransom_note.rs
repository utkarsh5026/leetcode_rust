use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Hey! This function checks if you can create a ransom note using letters
    /// from a magazine. Each letter in the magazine can only be used once!
    ///
    /// Here's how it works:
    /// 1. Create frequency maps for both strings:
    ///    - One for the magazine (how many letters we have)
    ///    - One for the ransom note (how many letters we need)
    ///
    /// 2. For each letter in the ransom note:
    ///    - Check if we have enough of that letter in the magazine
    ///    - If we don't have enough of any letter, return false
    ///
    /// For example:
    ///
    /// ransom_note = "aa", magazine = "aab"
    ///
    /// Steps:
    /// 1. Magazine map: {'a': 2, 'b': 1}
    /// 2. Ransom map: {'a': 2}
    /// 3. Check:
    ///    - Need 2 'a's, have 2 'a's ✅
    ///
    /// Result: true (we can make the note!)
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mag_map = Solution::create_hmap(&magazine);
        let ransom_map = Solution::create_hmap(&ransom_note);

        for (ch, count) in ransom_map {
            if mag_map.get(&ch).unwrap_or(&0) < &count {
                return false;
            }
        }
        true
    }

    /// Creates a frequency map counting how many times each character appears
    fn create_hmap(s: &str) -> HashMap<char, i32> {
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for ch in s.chars() {
            let cnt = char_map.entry(ch).or_insert(0);
            *cnt += 1;
        }
        char_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct() {
        // Test case from the example
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));

        // Test when ransom note can't be constructed
        assert!(!Solution::can_construct(
            "abc".to_string(),
            "ab".to_string()
        ));

        assert!(Solution::can_construct("".to_string(), "abc".to_string()));

        assert!(Solution::can_construct(
            "abc".to_string(),
            "abc".to_string()
        ));

        assert!(Solution::can_construct(
            "aa".to_string(),
            "aaaa".to_string()
        ));

        assert!(!Solution::can_construct(
            "abc".to_string(),
            "def".to_string()
        ));
    }
}
