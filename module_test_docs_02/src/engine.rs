//! # module engine
//! A collection of modules for engine

// use super::math::rand_int;
use crate::math::a::b::{bar, foo};
use crate::math::rand_float as math_rand_float;
use crate::math::rand_int;

// use crate::math;
// math::rand_int()

mod inner;

/// Returns a random roll_20
///
/// # Example
/// ```
/// use module_test_docs_02::engine::{roll8, roll_20};
///
/// let roll = roll20();
/// let roll_8 = roll8();
/// assert!(roll < 21);
/// assert!(roll_8 < 9);
/// println!("Your roll_20 is {roll}");
/// ```
///
/// # Panics
/// Never panics or panics in case of:
/// - some error
/// - another errori
/// "cargo test --all-targets" will ignore doc tests
/// "cargo test --doc" run only doc tests
pub fn roll_20() -> u32 {
    // inner::use_private_ulit(); // error
    rand_int() % 20 + 1
}

/// Returns a random roll8
pub fn roll8() -> u32 {
    private_ulit();
    rand_int() % 8 + 1
}

fn private_ulit() {
    let rand_float_val = math_rand_float();
    println!("Private function: {rand_float_val}");
}

// fn rand_float() -> f32 {
//     32.5
// }

#[cfg(test)]
mod tests {
    #[test]
    fn roll20_less_then_21() {
        let roll = super::roll_20();

        // panic!("Roll is {roll}");

        assert!(roll < 21);
    }

    #[test]
    fn roll8_less_then_9() {
        some_test_util();
        let roll = super::roll8();
        assert!(roll < 9);
    }

    fn some_test_util() {}

    #[test]
    #[ignore = "too slow"]
    fn roll20_less_then_21_multiple() {
        for _ in 0..1_000_000 {
            let roll = super::roll_20();
            assert!(roll < 21);
        }
    }

    #[test]
    #[should_panic]
    fn always_fail() {
        assert!(false);
    }
}
