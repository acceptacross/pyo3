#![allow(dead_code, unused_variables)]
#![feature(proc_macro, specialization, const_fn, const_align_of, const_size_of)]

extern crate docmatic;

#[test]
fn test_overview() {
    docmatic::assert_file("guide/src/overview.md");
}

#[test]
fn test_conversions() {
    docmatic::assert_file("guide/src/conversions.md")
}

#[test]
fn test_exception() {
    docmatic::assert_file("guide/src/exception.md")
}
