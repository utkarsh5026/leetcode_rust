# 🦀 Leetcode Solutions in Rust

Hey there! 👋 This is my personal playground where I'm learning Rust by solving Leetcode problems. I'm documenting my journey from "what the heck is ownership?" to "look ma, I'm fighting the borrow checker and winning (sometimes)!"

## What's This All About?

I'm working through Leetcode problems using Rust because:
- It's a fantastic way to learn Rust's unique features
- The problems are fun brain teasers
- Who doesn't love that feeling when the tests finally pass? 🎉

## Structure

Each solution lives in its own file under `solutions/src/sols/`. They're all thoroughly commented because future-me will definitely forget how they work!

## Some Cool Solutions So Far

- [Remove Duplicates](solutions/src/sols/p0026_remove_duplicate_from_sorted_array.rs) - Playing with pointers and in-place array manipulation
- [Merge Sorted Arrays](solutions/src/sols/p0088_merge_sorted_array.rs) - A neat backwards iteration approach
- [String Score Calculator](solutions/src/sols/p3110_score_of_string.rs) - Using Rust's awesome iterator methods
- [Array Concatenation](solutions/src/sols/p1929_concatenation_of_array.rs) - Simple but sweet vector operations

## Running the Code

```
bash
cd solutions
cargo test # Run all the tests
```