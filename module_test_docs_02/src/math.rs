//! # module engine
//! A collection of modules for engine

/// Returns a random integer
pub fn rand_int() -> u32 {
    49
}

/// Returns a random float
pub fn rand_float() -> f32 {
    49.0
}

pub mod a {
    pub mod b {
        pub fn foo() {}   
        pub fn bar() {}   
    }
}