// lib.rs

extern crate libc;
use libc::{c_int, c_double, c_char};
use std::ffi::{c_str_to_bytes_with_nul, CString};
use std::str;


#[link(name = "exampleC", kind = "static")]
extern {
    fn addTwoInt32(a: c_int, b: c_int) -> c_int;
    fn addTwoDoubles(a: c_double, b: c_double) -> c_double;
    fn sendTenToFunction(func: fn(i32) -> i32) -> c_int;
    fn getAString() -> *const c_char;
    fn takeAString(s: *const c_char) -> i32;
    fn totalNumbers(length: i32, numbers: &[i32]) -> i32;
}


fn add_two_i32(a: i32, b: i32) -> i32 {
    unsafe {
        let result: i32 = addTwoInt32(a, b);
        return result;
    }
}


fn add_two_doubles(a: f64, b: f64) -> f64 {
    unsafe {
        let result: f64 = addTwoDoubles(a, b);
        return result;
    }
}


fn send_ten_to_function(func: fn(i32) -> i32) -> i32 {
    unsafe {
        let result: i32 = sendTenToFunction(func);
        return result;
    }
}


fn get_a_string() -> String {
    unsafe {
        let orig: *const c_char = getAString();
        let bytes = c_str_to_bytes_with_nul(&orig);
        let converted: &str = str::from_utf8(bytes).unwrap();
        return String::from_str(converted);
    }
}


fn total_numbers(numbers: Vec<i32>) -> i32 {
    let length: i32 = std::num::from_uint(numbers.len()).unwrap();
    let numList = numbers.as_slice();
    unsafe {
        let result = totalNumbers(length, numList);
        return result;
    }
}


fn take_a_string(s: String) -> i32 {
    unsafe {
        let cVersion = CString::from_slice(s.as_bytes());
        let result = takeAString(cVersion.as_ptr());
        return result;
    }
}


#[test]
fn test_add_two_i32() {
    assert!(add_two_i32(4, 5) == 9);
}


#[test]
fn test_add_two_doubles() {
    let result: f64 = add_two_doubles(3.9, 5.1);
    assert!(result > 8.9 && result < 9.1);
}


fn add5(n: i32) -> i32 {
    return n + 5;
}


#[test]
fn test_send_ten_to_function() {
    let result: i32 = send_ten_to_function(add5);
    assert!(result == 15);
}


#[test]
fn test_get_a_string() {
    let result: String = get_a_string();
    assert!(result.len() > 0);
}


#[test]
fn test_take_a_string() {
    let inputStr: String = String::from_str("Here is a string");
    let result: i32 = take_a_string(inputStr);
    assert!(result > 0);
}
