extern crate ferris_says;
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello world!!!!!!!!!!!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    loop {
        say(&message, width, &mut writer).unwrap();
    }
}
