use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Hey! This function finds all words from words1 that are "universal" words for words2.
    /// A word is "universal" if it contains all the characters from each word in words2
    /// (with at least the same frequency).
    ///
    /// Here's how it works:
    /// 1. First, we create a maximum frequency map from words2
    ///    - For each character, we keep track of the highest count needed
    ///    - This combines all words2 requirements into one map!
    ///
    /// 2. Then for each word in words1:
    ///    - Create a frequency map for that word
    ///    - Check if it meets all the requirements from our max frequency map
    ///    - If it does, add it to our result list
    ///
    /// For example:
    ///
    /// words1 = ["amazon", "apple", "facebook"]
    /// words2 = ["e", "o"]
    ///
    /// Steps:
    /// 1. Max freq map from words2: {'e': 1, 'o': 1}
    /// 2. Check each word:
    ///    - "amazon": has 'o', no 'e' ❌
    ///    - "apple": has 'e', no 'o' ❌
    ///    - "facebook": has both 'e' and 'o' ✅
    ///
    /// Result: ["facebook"]
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let max_freq_map = Solution::create_max_freq_map(&words2);
        let mut subsets = Vec::new();

        for word in words1 {
            let word_map = Solution::create_hmap(&word);
            if Solution::is_subset(&max_freq_map, &word_map) {
                subsets.push(word);
            }
        }
        subsets
    }

    /// Creates a map showing the maximum frequency needed for each character
    /// across all words in words2
    fn create_max_freq_map(words: &Vec<String>) -> HashMap<char, i32> {
        let mut max_freq: HashMap<char, i32> = HashMap::new();

        for word in words {
            let curr_freq = Solution::create_hmap(word);
            for (ch, count) in curr_freq {
                let max_count = max_freq.entry(ch).or_insert(0);
                *max_count = (*max_count).max(count);
            }
        }
        max_freq
    }

    /// Creates a frequency map for a single word
    fn create_hmap(word: &String) -> HashMap<char, i32> {
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for c in word.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }
        char_map
    }

    /// Checks if one frequency map (big) contains at least the frequencies
    /// required by another map (small)
    fn is_subset(small: &HashMap<char, i32>, big: &HashMap<char, i32>) -> bool {
        for (k, v) in small {
            if big.get(k).unwrap_or(&0) < v {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_subsets() {
        // Test case from the example
        assert_eq!(
            Solution::word_subsets(
                vec![
                    "amazon".to_string(),
                    "apple".to_string(),
                    "facebook".to_string()
                ],
                vec!["e".to_string(), "o".to_string()]
            ),
            vec!["facebook"]
        );

        // Test with empty words2
        assert_eq!(
            Solution::word_subsets(vec!["hello".to_string(), "world".to_string()], vec![]),
            vec!["hello", "world"] // All words should match when words2 is empty
        );

        // Test with multiple character requirements
        assert_eq!(
            Solution::word_subsets(
                vec![
                    "warrior".to_string(),
                    "world".to_string(),
                    "water".to_string()
                ],
                vec!["wa".to_string(), "r".to_string()]
            ),
            vec!["warrior", "water"]
        );

        // Test with frequency requirements
        assert_eq!(
            Solution::word_subsets(
                vec![
                    "letter".to_string(),
                    "better".to_string(),
                    "setter".to_string()
                ],
                vec!["tt".to_string()] // Requires two 't's
            ),
            vec!["letter", "better", "setter"]
        );

        // Test with no matches
        assert_eq!(
            Solution::word_subsets(
                vec!["abc".to_string(), "def".to_string()],
                vec!["xyz".to_string()]
            ),
            Vec::<String>::new() // No words should match
        );
    }
}
