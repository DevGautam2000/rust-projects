#![allow(unused)]


mod ferr{
    use ferris_says::say; // from the previous step
    use std::io::{stdout, BufWriter};

    pub fn print_ferris() {
        let stdout = stdout();
        let message = String::from("Hello fellow Rustaceans!");
        let width = message.chars().count();
        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }
}

mod print;
mod format;
mod datatypes;
mod vars;
mod shadowing;
mod constants;
mod types;

fn main() { 
    // ferr::print_ferris();
    // print::run();
    // format::run();
    // datatypes::run();
    // vars::run();
    // shadowing::run();
    // constants::run();
    types::run();
}
