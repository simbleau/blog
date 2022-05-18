+++
title = "Blind 50 in Rust"
date = 2022-05-17T00:00:00+00:00
description = "The most famous 50 problems commonly used in interviews, in Rust."

[taxonomies]
categories = ["Rust"]
tags = ["rust"]
[extra]
toc = true
+++

# What are the Blind 50?
The [Blind 75](https://leetcode.com/list/xi4ci4ig/) is the most important list of questions used in coding interviews to judge candidates. Years later, the author later revised this list to 50 questions in 5 weeks to prepare in an even quicker timeframe[^author_rev]. Preparing for these interviews usually means hitting a heavy paywall, and I am against money benefitting someone's education. Hence, I wanted to open-source the author's Blind 50 questions, implemented in Rust. This is dually helpful because the answers are uniquely different than the common Python or Java implementations you may find online, given the uniqueness of Rust itself.

# The Blind 50 List
## Sequences
| Question | Difficulty | LeetCode | Rust Playground |
| -------- | ---------- | -------- | ---- |
| Two Sum | Easy | [Link](https://leetcode.com/problems/two-sum/) | [Link]() |
| Contains Duplicate | Easy | [Link](https://leetcode.com/problems/contains-duplicate/) | [Link]() |
| Best Time to Buy and Sell Stock | Easy | [Link](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/) | [Link]() |
| Valid Anagram | Easy | [Link](https://leetcode.com/problems/valid-anagram/) | [Link]() |
| Valid Parentheses | Easy | [Link](https://leetcode.com/problems/valid-parentheses/) | [Link]() |
| Maximum Subarray | Easy | [Link](https://leetcode.com/problems/maximum-subarray/) | [Link]() |
| Product of Array Except Self | Medium | [Link](https://leetcode.com/problems/product-of-array-except-self/) | [Link]() |
| 3Sum | Medium | [Link](https://leetcode.com/problems/3sum/) | [Link]() |
| Merge Intervals | Medium | [Link](https://leetcode.com/problems/merge-intervals/) | [Link]() |
| Group Anagrams | Medium | [Link](https://leetcode.com/problems/group-anagrams/) | [Link]() |

## Data Structures
| Question | Difficulty | LeetCode | Rust Playground |
| -------- | ---------- | -------- | ---- |
| Reverse a Linked List | Easy | [Link](https://leetcode.com/problems/reverse-linked-list/) | [Link]() |
| Detect Cycle in a Linked List | Easy | [Link](https://leetcode.com/problems/linked-list-cycle/) | [Link]() |
| Container With Most Water | Medium | [Link](https://leetcode.com/problems/container-with-most-water/) | [Link]() |
| Find Minimum in Rotated Sorted Array | Medium | [Link](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/) | [Link]() |
| Longest Repeating Character Replacement | Medium | [Link](https://leetcode.com/problems/longest-repeating-character-replacement/) | [Link]() |
| Longest Substring Without Repeating Characters | Medium | [Link](https://leetcode.com/problems/longest-substring-without-repeating-characters/) | [Link]() |
| Number of Islands | Medium | [Link](https://leetcode.com/problems/number-of-islands/) | [Link]() |
| Remove Nth Node From End Of List | Medium | [Link](https://leetcode.com/problems/remove-nth-node-from-end-of-list/) | [Link]() |
| Palindromic Substrings | Medium | [Link](https://leetcode.com/problems/palindromic-substrings/) | [Link]() |
| Pacific Atlantic Water Flow | Medium | [Link](https://leetcode.com/problems/pacific-atlantic-water-flow/) | [Link]() |
| Minimum Window Substring | Hard | [Link](https://leetcode.com/problems/minimum-window-substring/) | [Link]() |

## Non-linear Data Structures
| Question | Difficulty | LeetCode | Rust Playground |
| -------- | ---------- | -------- | ---- |
| Invert/Flip Binary Tree | Easy | [Link](https://leetcode.com/problems/invert-binary-tree/) | [Link]() |
| Validate Binary Search Tree | Medium | [Link](https://leetcode.com/problems/validate-binary-search-tree/) | [Link]() |
| Non-overlapping Intervals | Medium | [Link](https://leetcode.com/problems/non-overlapping-intervals/) | [Link]() |
| Construct Binary Tree from Preorder and Inorder Traversal | Medium | [Link](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) | [Link]() |
| Top K Frequent Elements | Medium | [Link](https://leetcode.com/problems/top-k-frequent-elements/) | [Link]() |
| Clone Graph | Medium | [Link](https://leetcode.com/problems/clone-graph/) | [Link]() |
| Course Schedule | Medium | [Link](https://leetcode.com/problems/course-schedule/) | [Link]() |
| Serialize and Deserialize Binary Tree | Hard | [Link](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/) | [Link]() |
| Binary Tree Maximum Path Sum | Hard | [Link](https://leetcode.com/problems/binary-tree-maximum-path-sum/) | [Link]() |

## More Data Structures
| Question | Difficulty | LeetCode | Rust Playground |
| -------- | ---------- | -------- | ---- |
| Subtree of Another Tree | Easy | [Link](https://leetcode.com/problems/subtree-of-another-tree/) | [Link]() |
| Lowest Common Ancestor of BST | Easy | [Link](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/) | [Link]() |
| Implement Trie (Prefix Tree) | Medium | [Link](https://leetcode.com/problems/implement-trie-prefix-tree/) | [Link]() |
| Add and Search Word | Medium | [Link](https://leetcode.com/problems/add-and-search-word-data-structure-design/) | [Link]() |
| Kth Smallest Element in a BST | Medium | [Link](https://leetcode.com/problems/kth-smallest-element-in-a-bst/) | [Link]() |
| Merge K Sorted Lists | Hard | [Link](https://leetcode.com/problems/merge-k-sorted-lists/) | [Link]() |
| Find Median from Data Stream | Hard | [Link](https://leetcode.com/problems/find-median-from-data-stream/) | [Link]() |
| Insert Interval | Medium | [Link](https://leetcode.com/problems/insert-interval/) | [Link]() |
| Longest Consecutive Sequence | Medium | [Link](https://leetcode.com/problems/longest-consecutive-sequence/) | [Link]() |
| Word Search II | Hard | [Link](https://leetcode.com/problems/word-search-ii/) | [Link]() |

## Dynamic Programming
| Question | Difficulty | LeetCode | Rust Playground |
| -------- | ---------- | -------- | ---- |
| Climbing Stairs | Easy | [Link](https://leetcode.com/problems/climbing-stairs/) | [Link]() |
| Coin Change | Medium | [Link](https://leetcode.com/problems/coin-change/) | [Link]() |
| Longest Increasing Subsequence | Medium | [Link](https://leetcode.com/problems/longest-increasing-subsequence/) | [Link]() |
| Combination Sum IV | Medium | [Link](https://leetcode.com/problems/combination-sum-iv/) | [Link]() |
| House Robber | Medium | [Link](https://leetcode.com/problems/house-robber/) | [Link]() |
| House Robber II | Medium | [Link](https://leetcode.com/problems/house-robber-ii/) | [Link]() |
| Decode Ways | Medium | [Link](https://leetcode.com/problems/decode-ways/) | [Link]() |
| Unique Paths | Medium | [Link](https://leetcode.com/problems/unique-paths/) | [Link]() |
| Jump Game | Medium | [Link](https://leetcode.com/problems/jump-game/) | [Link]() |
| Word Break | Medium | [Link](https://leetcode.com/problems/word-break/) | [Link]() |


Farvel! Until next time.

---
[^author_rev]: [Best practice questions by the author of Blind 75](https://www.techinterviewhandbook.org/best-practice-questions/)