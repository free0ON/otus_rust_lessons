pub fn roll20() -> u32 {
    rand_int() % 20 + 1
}

#[cfg(feature = "real_rand")]
fn rand_int() -> u32 {
    rand::random()
}

#[cfg(not(feature = "real_rand"))]
fn rand_int() -> u32 {
    42
}
