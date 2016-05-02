use std::collections::HashMap;
use std::iter::Iterator;
use LispVal::*;

fn tokenize(program: &String) -> Vec<String> { 
    program.replace("(", " ( ").replace(")", " ) ")
           .split_whitespace().map(|x| x.to_string())
           .collect()
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Vec<LispVal>, String> {
    let res: Result<Vec<LispVal>, String>;
    if tokens.len() == 0 {
        res = Err("Unexpected EOF while reading.".to_string());
    } else {

        let token = tokens.remove(0); // ok because ^ 

        if "(" == token {
            let mut vals = Vec::new();
 
            while tokens.get(0).unwrap() != ")" { // this line is probably Bad -- yeah see below
                vals.append(&mut read_from_tokens(tokens).unwrap()); // TODO this line is probably Bad too.
            }
            // TODO 
            // in lis.py, this is the line that catches missing rparens. 
            // (techincally, it's "tokens.pop(0) # pop off ')'")
            // Try tokens.remove(0) wrapped in something that'll return a Result?
            // tokens.pop(); // ALTERED LINE
            tokens.remove(0);

            let l = LispVal::LispList { l: Box::new(vals) };
            res = Ok(vec![l]);
        }
        else if ")" == token {
            res = Err("Unexpected rparen found.".to_string());
        } else {
            let l = vec![LispVal::atom(token)];
            res = Ok(l);
        }
    }
    res
}

fn parse(program: &String) -> Result<Vec<LispVal>, String> {
    let mut tokens = tokenize(program);
    read_from_tokens(&mut tokens)
}

// #[derive(Debug)]
enum LispVal {
    LispInt     { x: i32                        },
    LispFlt     { x: f64                        },
    LispSym     { s: String                     },
    LispList    { l: Box<Vec<LispVal>>          },
    LispFn      { f: Box<Fn(LispVal) -> LispVal>  },
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

fn standard_env() -> HashMap<String, LispVal> {
    let mut env = HashMap::new();

    env.insert("+", Box::new(lisp_add));
    env;
}

fn only_lisp_numbers(v: &Vec<LispVal>) -> bool {
    (*v).into_iter().all(|x| match x { 
        LispInt { .. } => true,
        LispFlt { .. } => true, 
        _              => false,
    })
}

fn contains_floats(v: &Vec<LispVal>) -> bool {
    (*v).into_iter().any(|x| match x {
        LispFlt { .. }  => true,
        _               => false,
    })
}

fn convert_to_floats(v: &Vec<LispVal>) -> Vec<LispVal> {
    (*v).into_iter()
     .map(|x| match x {
         LispInt { x }   => LispFlt { x: x as f64 },
         LispFlt { ..  } => x,
         _               => panic!("shouldn't happen -- called in wrong order")})
     .collect()
}

// TODO -- should I implement Add?
fn sum_list_int(v: &Vec<LispVal>) -> LispVal { 
    (*v).into_iter()
        .fold(LispInt { x : 0 }, 
              |LispInt { x : sum }, LispInt { x }| LispInt { x: sum + x})
}

fn sum_list_float(v: &Vec<LispVal>) -> LispVal { 
    (*v).into_iter()
        .fold(LispFlt { x : 0.0 }, 
              |LispFlt { x : sum }, LispFlt { x }| LispFlt { x: sum + x})
}

fn lisp_add(v: LispVal) -> Result<LispVal, String> {
    match v {
        LispInt     { .. } => Ok(v),
        LispFlt     { .. } => Ok(v),
        LispList    { l }  => { if only_lisp_numbers(&l) {
                                    if contains_floats(&l) {
                                        let v = convert_to_floats(&l);
                                        Ok(sum_list_float(&v))
                                    } else {
                                        Ok(sum_list_int(&l))
                                    }
                                } else { 
                                    Err("found non-numerics in arg list".to_string())
                                }
                            }
                            // panic!("not implemented yet lol"),
        _                  => Err("Expected list of numeric values.".to_string()),
    }
}

fn main() {
    println!("Hello, world!");

    let s = "(+ 1 1)".to_string();
    println!("Let's tokenize {}", s);
    println!("{:?}", tokenize(&s));
    println!("Let's parse {} into a Lisp-y form.", s);
    println!("{:?}", parse(&s));

    let s2 = "(if (< x 0) (* x -1) x)".to_string();
    println!("Let's try something more complex: {}", s2);
    println!("Tokenized: {:?}", tokenize(&s2));
    println!("Let's parse it now.");
    println!("Parsed: {:?}", parse(&s2));

    let s3 = "(if (< x 0) (* x -1) x".to_string();
    println!("This test should fail-- uneven parentheses! {:?}", s3);
    println!("Parsed: {:?}", parse(&s3));
    println!("It doesn't. Gotta fix this later.");
}
