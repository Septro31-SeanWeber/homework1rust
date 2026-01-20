/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    The Rust borrow checker may help avoid some possible bugs.

    Then answer this question:
    Q: A common source of error in swap implementations is failing to work if
       the two references are the same. Why don't you need to worry about this
       case in Rust?

    (Try writing a unit test where they are both
    the same, i.e. swap_ints(&mut x, &mut x).)
*/
pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    let x = *x1;
    *x1 = *x2;
    *x2 = x;
}

#[test]
fn test_swap_ints(){
    let mut x1 = 2;
    let mut x2 = 4;
    swap_ints(&mut x1, &mut x2);
    assert_eq!(x1, 4);
    assert_eq!(x2, 2);
}

/*
    Problem 2: String duplication
*/
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?

// Q2. How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..times{
        v.push(s.to_string().clone());
    }
    v
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: &String) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
// fn new_ref_string<'a>() -> &'a String {
//     let s : &'a str = "lifetime";
//     s
// }

// fn new_ref_str() -> &str {
//     unimplemented!();
// }

// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    match s1.len() > s2.len(){
        true => s1,
        false => s2
    }
}


/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    If the vector is empty, return "".

    Q1. In pick_longest_in_v2, if you were to explicitly specify the lifetime
        of the input and output, what should it be?

    Q2. What are the pros and cons of v1 and v2?
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    v.iter().fold((String::from("")), |acc, x| pick_longest2(acc.as_str(),x.as_str()).to_string())
}

fn pick_longest_in_v2(v: Vec<&str>) -> &str {
    v.into_iter().fold("", pick_longest2)
}

#[test]
fn test_pick_longest_in_v1() {
    let v = vec![String::from("H"), String::from("He"), String::from("Hello"), String::from("Hell"), String::from("Hel")];
    assert_eq!(pick_longest_in_v1(v), String::from("Hello"));
}

#[test]
fn test_pick_longest_in_v2() {
    let v = vec!["H", "He", "Hello", "Hell", "Hel"];
    assert_eq!(pick_longest_in_v2(v), "Hello");
}
/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() if necessary to make any additional unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    let mut r = vec![];
    if desired_len > v.len(){
        for i in 0..desired_len{
            if i < v.len(){
                r.push(v[i]);
            }else{
                r.push(0);
            }
        }
    }
    r
    // debug_assert_eq!(result.len(), desired_len);
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    let mut r = slice.to_vec();
    if desired_len > slice.len(){
        for i in slice.len()..desired_len{
            r.push(0);
        }
    }
    r
    // debug_assert_eq!(result.len(), desired_len);
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    if desired_len > v.len(){
        for i in v.len()..desired_len{
            v.push(0)
        }
    }
    debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    // Check if row is the first row in grid
    // Remember to handle the case when grid is empty
    match grid.len(){
        0 => row.len() < 1,
        _ => row == grid[0]
    }
}

#[test]
fn test_append_row() {
    let mut grid = vec![vec![true, true, false], vec![true, true, false], vec![true, true, false]];
    let row = vec![true, false, true];
    append_row(&mut grid, row);
    assert_eq!(grid, vec![vec![true, true, false], vec![true, true, false], vec![true, true, false], vec![true, false, true]])
}

#[test]
fn test_is_first_row() {
    let grid = vec![vec![true, true, false], vec![true, true, false], vec![true, true, false]];
    let row = vec![true, true, false];
    let wrong_row_1 = vec![true, false, true];
    let wrong_row_2 = vec![true, true, false, true];
    assert_eq!(is_first_row(&grid, &row), true);
    assert_eq!(is_first_row(&grid, &wrong_row_1), false);
    assert_eq!(is_first_row(&grid, &wrong_row_2), false);
}


/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::{collections::HashMap, hash::Hash};

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    let mut h_m = HashMap::new();
    for i in v{
        h_m.insert(i.0, i.1.to_string());
    }
    h_m
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    h.retain(|&k, _| k >= 0);
}

#[test]
fn test_delete_negative_keys(){
    let mut h = HashMap::new();
    h.insert(1, 2);
    h.insert(0, 3);
    h.insert(2, 4);
    h.insert(-1, 5);
    h.insert(3, 6);
    h.insert(-2, 7);
    h.insert(4, 8);
    let mut correct = HashMap::new();
    correct.insert(1, 2);
    correct.insert(0, 3);
    correct.insert(2, 4);
    correct.insert(3, 6);
    correct.insert(4, 8);
    delete_negative_keys(&mut h);
    assert_eq!(h, correct);
}
/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    for (k, v) in add{
        merged.entry(k).and_modify(|e| e.push_str(v.as_str())).or_insert(v);
    }
}

#[test]
fn test_(){
    let mut merged = HashMap::new();
    merged.insert("1".to_string(), "he".to_string());
    merged.insert("3".to_string(), "sup".to_string());
    merged.insert("4".to_string(), "yo".to_string());
    let mut add = HashMap::new();
    add.insert("3".to_string(), "erb".to_string());
    add.insert("2".to_string(), "hello".to_string());
    add.insert("4".to_string(), "lo".to_string());
    let mut correct = HashMap::new();
    correct.insert("1".to_string(), "he".to_string());
    correct.insert("2".to_string(), "hello".to_string());
    correct.insert("3".to_string(), "superb".to_string());
    correct.insert("4".to_string(), "yolo".to_string());
    merge_maps(&mut merged, add);
    assert_eq!(merged, correct);
}
