fn tokenize(program: String) -> Vec<String> { 
    program.replace("(", " ( ").replace(")", " ) ")
           .split_whitespace().map(|x| x.to_string())
           .collect()
}

#[derive(Debug)]
enum LispVal {
    LispInt { x: i32 },
    LispFlt { x: f64 },
    LispStr { s: String },
    LispList { l: Box<Vec<LispVal>> },
}

fn main() {
    println!("Hello, world!");
    let s = "(+ 1 1)".to_string();
    println!("Let's tokenize {}", s);
    println!("{:?}", tokenize(s));
}
