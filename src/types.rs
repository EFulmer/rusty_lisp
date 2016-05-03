use std::collections::HashMap;

/// `Env`, short for "environment", is a mapping of names to values, currently implemented as a
/// HashTable. Exported as a type synonym here.
pub type Env = HashMap<String, LispVal>;

/// The set of types this Lisp supports.
#[derive(Debug)]
pub enum LispVal {
    /// An integral numeric value. Currently 32-bit precision.
    LispInt   { x: i32                        },
    /// A floating-point numeric value. Currently 64-bit precision; precision 
    /// unlikely to change.
    LispFloat { x: f64                        },
    /// Symbol, used for representing both language constructs like `if`, `define`, 
    /// `lambda`, and user-defined symbols of the form `'sym-name`.
    LispSym   { s: String                     },
    /// Lisp list: a reference to a list containing more Lisp values.
    LispList  { l: Box<Vec<LispVal>>          },
    // Reference to a function on Lisp values. Will include function metadata 
    // (captured values, etc.) in future version.
    // LispFn      { f: Box<Fn(LispVal) -> LispVal>  },
}

impl LispVal {
    /// number, symbol, or string
    pub fn atom(s: String) -> LispVal {
        match s.parse::<i32>() {
            Ok(x) => LispVal::LispInt { x: x } ,
            Err(_) => match s.parse::<f64>() {
                Ok(x) => LispVal::LispFloat { x: x},
                Err(_) => LispVal::LispSym { s: s },
            }
        }
    }
}
