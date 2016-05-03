use types;

pub fn tokenize(program: &String) -> Vec<String> { 
    program.replace("(", " ( ").replace(")", " ) ")
           .split_whitespace().map(|x| x.to_string())
           .collect()
}

fn read_from_tokens(tokens: &mut Vec<String>) -> Result<Vec<types::LispVal>, String> {
    let res: Result<Vec<types::LispVal>, String>;
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

            let l = types::LispVal::LispList { l: Box::new(vals) };
            res = Ok(vec![l]);
        }
        else if ")" == token {
            res = Err("Unexpected rparen found.".to_string());
        } else {
            let l = vec![types::LispVal::atom(token)];
            res = Ok(l);
        }
    }
    res
}

pub fn parse(program: &String) -> Result<Vec<types::LispVal>, String> {
    let mut tokens = tokenize(program);
    read_from_tokens(&mut tokens)
}
