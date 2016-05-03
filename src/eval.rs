use types::*;
use types::LispVal::*;

pub fn eval(l: LispVal, e: Env) -> LispResult {
    match l {
        LispSym   { s }  => match e.get(&s) {
            Some(v) => Ok((*v).clone()),
            _       => Err("symbol not defined".to_string()),
        },
        LispInt   { x }  => Ok(l),
        LispFloat { x }  => Ok(l),
        LispBool  { b }  => Ok(l),
        LispList  { l } => handle_list(*l, e),
    }
}

fn handle_list(l: Vec<LispVal>, e: Env) -> LispResult {
    panic!("not yet implemented")
}
