fn tokenize(program: String) -> Vec<String> { 
    program.replace("(", " ( ").replace(")", " ) ")
           .split_whitespace().map(|x| x.to_string())
           .collect()
}

#[derive(Debug)]
enum LispVal {
    LispInt { x: i32 },
    LispFlt { x: f64 },
    LispSym { s: String },
    LispList { l: Box<Vec<LispVal>> },
}

impl LispVal {
    /// number, symbol, or string
    fn atom(s: String) -> LispVal {
        match s.parse::<i32>() {
            Ok(x) => LispVal::LispInt { x: x } ,
            Err(_) => match s.parse::<f64>() {
                Ok(x) => LispVal::LispFlt { x: x},
                Err(_) => LispVal::LispSym { s: s },
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let s = "(+ 1 1)".to_string();
    println!("Let's tokenize {}", s);
    println!("{:?}", tokenize(s));
}
