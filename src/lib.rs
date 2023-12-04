use std::{fs, fmt, string, time};

pub const EXAMPLE_DIR: &str = "examples";
pub const INPUT_DIR: &str = "inputs";

pub fn read_file_string(dir: &str, name: &str) -> Box<str> {
    fs::read_to_string(format!("{dir}/{name}.txt")).map(string::String::into_boxed_str).unwrap()
}

#[allow(clippy::option_if_let_else)]
pub fn print_result<T: fmt::Display>(part: u32, func: impl FnOnce(&str) -> Option<T>, input: &str) {
    let tim = time::Instant::now();
    let result = func(input);
    let el = tim.elapsed();
    println!("> Part {part} ({el:.2?})");
    if let Some(r) = result {
        println!("{r}");
    } else {
        println!("[!] It cannot be solved");
    }
}

#[macro_export]
macro_rules! solve {
    ($solver1:ident, $solver2:ident) => {
        fn main() {
            let input = &aoc2023::read_file_string(aoc2023::INPUT_DIR, env!("CARGO_BIN_NAME"));
            aoc2023::print_result(1, $solver1, input);
            aoc2023::print_result(2, $solver2, input);
        }
    };
}
#[macro_export]
macro_rules! assert_ex {
    ($solver:ident, $val:expr) => {
        let input = aoc2023::read_file_string(aoc2023::EXAMPLE_DIR, env!("CARGO_BIN_NAME"));
        assert_eq!($solver(&input), Some($val))
    };
}
#[macro_export]
macro_rules! assert_ex_part {
    ($part:expr, $solver:ident, $val:expr) => {
        let input = aoc2023::read_file_string(aoc2023::EXAMPLE_DIR, &format!("{}_{}", env!("CARGO_BIN_NAME"), $part));
        assert_eq!($solver(&input), Some($val))
    };
}