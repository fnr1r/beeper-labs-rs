#![allow(irrefutable_let_patterns)]

use alloc::boxed::Box;
use core::convert::Infallible;

use super::*;

#[test]
fn branch_forward_some_test() {
    let v = Box::new("aspp");
    let v2 = branch_forward(Some(v.clone()));
    let Continue(v2) = v2 else {
        panic!();
    };
    assert_eq!(v2, v);
    let v3 = branch_forward::<Infallible>(None);
    let Break(v3) = v3 else {
        panic!();
    };
    assert_eq!(v3, ());
}
#[test]
fn branch_reverse_some_test() {
    let v = Box::new("aspp");
    let v2 = branch_reverse(Some(v.clone()));
    let Break(v2) = v2 else {
        panic!();
    };
    assert_eq!(v2, v);
    let v3 = branch_reverse::<Infallible>(None);
    let Continue(v3) = v3 else {
        panic!();
    };
    assert_eq!(v3, ());
}
