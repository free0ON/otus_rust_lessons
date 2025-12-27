// Agenda
// 1. Features of Rust
// 2. Multitreads
// 3. Arhiceture
// 4. Ecosystem
// 5. Final project
// 6. Homeworks
// to send hw at chat as
// - url to repo,
// - pull request,
// - playground,
// - zip,
// - main.rs

pub fn get_src_path() -> &'static str {
    let parts: Vec<&str> = file!().split('\\').collect();
    parts[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_src_path(), "solve_problems_with_rust_01");
    }
}
