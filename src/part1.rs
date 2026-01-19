/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.


/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    n * 2
}

pub fn double_v2(n: &i32) -> i32 {
    *n * 2
}

pub fn double_v3(n: &mut i32) {
    // double n in place
    *n = *n << 1;
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}
#[test]
fn test_double_v2() {
    assert_eq!(double_v2(&2), 4);
    assert_eq!(double_v2(&-3), -6);
}
#[test]
fn test_double_v3() {
    let mut x = 2;
    double_v3(&mut x);
    assert_eq!(x, 4);
    x = -3;
    double_v3(&mut x);
    assert_eq!(x, -6);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    let mut x = 1;
    loop {
        if (x + 1) * (x + 1) >= n{
            break x
        }
        x += 1;
    }
}

#[test]
fn test_sqrt() {
    assert_eq!(sqrt(2), 1);
    assert_eq!(sqrt(99), 9);
}
// Remember to write unit tests here (and on all future functions)

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut x = 0;
    for &v in slice {
        x += v;
    }
    x
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut x = 0;
    for v in slice {
        x += *v;
    }
    x
}

#[test]
fn test_sum_v1() {
    assert_eq!(sum_v1(&[1, 2, 3]), 6);
    assert_eq!(sum_v1(&[1, 2, -3]), 0);
}
#[test]
fn test_sum_v2() {
    assert_eq!(sum_v2(&[1, 2, 3]), 6);
    assert_eq!(sum_v2(&[1, 2, -3]), 0);
}
/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut v = Vec::new();
    for &i in slice{
        if !v.contains(&i){
            v.push(i);
        }
    }
    v
}
#[test]
fn test_unique(){
    assert_eq!(unique(&[1, 2, 3, 4, 5]),vec![1, 2, 3, 4, 5]);
    assert_eq!(unique(&[1, 2, 1, 3, 3, 3, 2, 4, 4, 5, 1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
}
/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut v = Vec::new();
    for &i in slice{
        if pred(i){
            v.push(i)
        }
    }
    v
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    fn is_odd (n: i32) -> bool {
        n % 2 != 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_odd), vec![1, 3, 5]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..out_size{
        if i == 0{
            v.push(n1)
        }else if i == 1{
            v.push(n2)
        }else{
            v.push(v[i-1] + v[i-2])
        }
    }
    v

}

#[test]
fn test_fibonacci(){
    assert_eq!(fibonacci(1, 1, 5), vec![1, 1, 2, 3, 5]);
    assert_eq!(fibonacci(0, 2, 6), vec![0, 2, 2, 4, 6, 10]);
}
/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    format!("{s1}{s2}")
}

pub fn string_concat(s1: String, s2: String) -> String {
    let mut owned = s1;
    owned.push_str(&s2);
    owned.to_string()
}
#[test]
fn test_str_concat(){
    assert_eq!(str_concat("amogus", "sus"), "amogussus");
    assert_eq!(str_concat("HOL8198PPP", ""), "HOL8198PPP");
}
#[test]
fn test_string_concat(){
    assert_eq!(string_concat("amogus".to_string(), "sus".to_string()), "amogussus".to_string());
    assert_eq!(string_concat("HOL8198PPP".to_string(), "".to_string()), "HOL8198PPP".to_string());
}
/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut r = "".to_string();
    for s in v{
        r = string_concat(r, s);
    }
    r
}

#[test]
fn test_concat_all(){
    assert_eq!(concat_all(vec!["h".to_string(), "e".to_string(), "l".to_string(), "l".to_string(), "o".to_string()]), "hello".to_string());
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut r = Vec::new();
    for i in v{
        r.push(i.parse().expect("ignoring error"));
    }
    r
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut r = Vec::new();
    for i in v{
        r.push(format!("{i}").to_string())
    }
    r
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    concat_all(print_all(filter(&fibonacci(1, 1, n), &is_even)))
    
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
