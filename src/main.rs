fn tokenize(program: String) -> Vec<String> { 
    program.replace("(", " ( ").replace(")", " ) ")
           .split_whitespace().map(|x| x.to_string())
           .collect()
}

fn main() {
    println!("Hello, world!");
    let s = "(+ 1 1)".to_string();
    println!("Let's tokenize {}", s);
    println!("{:?}", tokenize(s));
}
