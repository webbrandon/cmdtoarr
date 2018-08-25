use std::env;
use std::str;
use std::io::{self, Write, BufRead};

pub fn out_term(object: &Vec<&str>) {
    writeln!(
        &mut io::stdout(),
        "{:?}",
        object
    ).ok();
}

fn convert(cmd: &String ) {
    let v: Vec<&str> = cmd.trim().split(' ').collect();
    out_term(&v);
}

fn main() {
    let mut args = env::args();
    
    match &args.nth(1) {
        Some(x) => {
            x.trim();
            convert(x);
        },
        None => {
            let stdin = io::stdin();
            for x in stdin.lock().lines() {
                convert(&x.unwrap());
            }
        }
    }
}