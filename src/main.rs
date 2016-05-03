use std::collections::HashMap;
use std::iter::Iterator;

mod env;
mod parse;
mod types;

fn main() {
    println!("Hello, world!");

    let s = "(+ 1 1)".to_string();
    println!("Let's tokenize {}", s);
    println!("{:?}", parse::tokenize(&s));
    println!("Let's parse {} into a Lisp-y form.", s);
    println!("{:?}", parse::parse(&s));

    let s2 = "(if (< x 0) (* x -1) x)".to_string();
    println!("Let's try something more complex: {}", s2);
    println!("Tokenized: {:?}", parse::tokenize(&s2));
    println!("Let's parse it now.");
    println!("Parsed: {:?}", parse::parse(&s2));

    let s3 = "(if (< x 0) (* x -1) x".to_string();
    println!("This test should fail-- uneven parentheses! {:?}", s3);
    println!("Parsed: {:?}", parse::parse(&s3));
    println!("It doesn't. Gotta fix this later.");
}
