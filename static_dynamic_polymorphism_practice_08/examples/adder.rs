// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=18c44055b7a7ae39ca194dc7e307d6b2
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::ops::Add;

// fn adder<T: Add<Output = T> + Clone>(y: T) -> impl Fn(T) -> T {
//     move |x| y.clone() + x
// }

// fn apply<T, F>(f: F, x: T) -> T
// where
//     F: Fn(T) -> T,
// {
//     f(x)
// }

fn adder<T: Add<Output = T> + Copy>(y: T) -> impl FnOnce(T) -> T + Copy {
    move |x| y + x
}

fn apply<T, F>(f: F, x: T) -> T
where
    F: FnOnce(T) -> T,
{
    f(x)
}

#[test]
fn test_adder() {
    assert_eq!(apply(adder(5), 1), 6);
    assert_eq!(apply(adder(3.14), 8.0), 11.14);
    let adder = adder(1);
    assert_eq!(apply(adder, 5), 6);
    assert_eq!(apply(adder, 6), 7);
}
