use std::env;
use std::io::{self, Write};

pub fn out_term(object: &Vec<String>) {
    writeln!(
        &mut io::stdout(),
        "{:?}",
        object
    ).ok();
}

fn main() {
    let mut args = env::args();
    let mut response:Vec<String> = Vec::new();
    args.next();
    for argument in args {
        response.push(argument);
    }
    out_term(&response)
}
