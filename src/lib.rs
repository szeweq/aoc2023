use std::{fs, fmt, string, time};

#[cfg(test)]
pub const TEXT_DIR: &str = "examples";

#[cfg(not(test))]
pub const TEXT_DIR: &str = "inputs";

pub fn read_file_string(name: &str) -> Box<str> {
    fs::read_to_string(format!("{TEXT_DIR}/{name}.txt"), ).map(string::String::into_boxed_str).unwrap()
}

#[allow(clippy::option_if_let_else)]
pub fn print_result<P, T: fmt::Display>(part: u32, func: fn(P) -> Option<T>, input: P) {
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
            let input = &aoc2023::read_file_string(env!("CARGO_BIN_NAME"));
            aoc2023::print_result(1, $solver1, input);
            aoc2023::print_result(2, $solver2, input);
        }
    };
    ($parser:ident, $solver1:ident, $solver2:ident) => {
        fn main() {
            let input = &aoc2023::read_file_string(env!("CARGO_BIN_NAME"));
            let pt = std::time::Instant::now();
            let parsed = $parser(input);
            println!("> Parsing time: {:.2?}", pt.elapsed());
            aoc2023::print_result(1, $solver1, &parsed);
            aoc2023::print_result(2, $solver2, &parsed);
        }
    }
}
#[macro_export]
macro_rules! assert_ex {
    ($solver:ident, $val:expr) => {
        let input = aoc2023::read_file_string(env!("CARGO_BIN_NAME"));
        assert_eq!($solver(&input), Some($val))
    };
    ($parser:ident, $solver:ident, $val:expr) => {
        let input = aoc2023::read_file_string(env!("CARGO_BIN_NAME"));
        let parsed = $parser(&input);
        assert_eq!($solver(&parsed), Some($val))
    }
}
#[macro_export]
macro_rules! assert_ex_part {
    ($part:expr, $solver:ident, $val:expr) => {
        let input = aoc2023::read_file_string(&format!("{}_{}", env!("CARGO_BIN_NAME"), $part));
        assert_eq!($solver(&input), Some($val))
    };
}