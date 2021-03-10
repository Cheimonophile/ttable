
// operators
pub const ASSIGNMENT: &str = ":";
pub const PRE_NEGATION: &str = "~";
pub const POST_VAL_NEGATION: &str = "\'";
pub const POST_OP_NEGATION: &str = "!";
pub const DISJUNCTION: &str = "+";
pub const NEG_DISJUNCTION: &str = "+!";
pub const CONJUNCTION: &str = "*";
pub const NEG_CONJUNCTION: &str = "*!";
pub const EX_DISJUNCTION: &str = "@";
pub const NEG_EX_DISJUNCTION: &str = "@!";
pub const IMPLICATION: &str = ">";
pub const NEG_IMPLICATION: &str = ">!";
pub const EQUIVALENCE: &str = "=";
pub const NEG_EQUIVALENCE: &str = "=!";
pub const OPEN: &str = "(";
pub const CLOSE: &str = ")";




// uses
use std::io;

// constant variables
const WHITESPACE: [char;3] = [' ','\t','\n'];

/**
 * checks if the char is whitespace
 */
pub fn is_whitespace(c:char)->bool {
    WHITESPACE.iter().any(|ws|*ws==c)
}

/**
 * checks if the char is in the alphabet
 */
pub fn is_alpha(c:char)->bool {
    let val = c as u8;
    return (val>=65 && val<=90) || (val>=97 && val<=122);
}

/**
 * checks if the char is alphanumeric
 */
pub fn is_alphanum(c:char)->bool {
    let val = c as u8;
    return (val>=65 && val<=90) || (val>=97 && val<=122) || (val>=48 && val<=57) || val == 95;
}

/**
 * token enum for tokenizing
 */
pub enum Token {
    Val(bool),
    Var(String),
    Op(&'static str),
    EndLine,
    EndScript
}

/**
 * Returns the precidence of the operator.
 */
pub fn prec(op:&str) -> io::Result<i32> {
    match op {
        ASSIGNMENT=>Ok(0),
        PRE_NEGATION=>Ok(1),
        DISJUNCTION=>Ok(2),
        NEG_DISJUNCTION=>Ok(2),
        CONJUNCTION=>Ok(3),
        NEG_CONJUNCTION=>Ok(3),
        EX_DISJUNCTION=>Ok(4),
        NEG_EX_DISJUNCTION=>Ok(4),
        IMPLICATION=>Ok(5),
        NEG_IMPLICATION=>Ok(5),
        EQUIVALENCE=>Ok(6),
        NEG_EQUIVALENCE=>Ok(6),
        POST_VAL_NEGATION=>Ok(7),
        POST_OP_NEGATION=>Ok(7),
        OPEN=>Ok(8),
        CLOSE=>Ok(-1),
        _=> {
            return Err(io::Error::new(io::ErrorKind::Other, format!("unrecognized operator {}", op)));
        }
    }
}


/**
 * Flips the operator
 */
pub fn flip(op:&str) -> io::Result<&str> {
    Ok(match op {
        DISJUNCTION=>NEG_DISJUNCTION,
        NEG_DISJUNCTION=>DISJUNCTION,
        CONJUNCTION=>NEG_CONJUNCTION,
        NEG_CONJUNCTION=>CONJUNCTION,
        EX_DISJUNCTION=>NEG_EX_DISJUNCTION,
        NEG_EX_DISJUNCTION=>EX_DISJUNCTION,
        IMPLICATION=>NEG_IMPLICATION,
        NEG_IMPLICATION=>IMPLICATION,
        EQUIVALENCE=>NEG_EQUIVALENCE,
        NEG_EQUIVALENCE=>EQUIVALENCE,
        _=> {
            return Err(io::Error::new(io::ErrorKind::Other, "operator can't be negated"));
        }
    })
}


/**
 * gets a token from the string, shrinking the sring in the process
 */
pub fn get_token(script: &mut String)->Option<Token> {

    // iterate over the chars in the string
    for i in 0..script.len() {
        let c = script.as_bytes()[i] as char;

        // if whitespace
        if is_whitespace(c) {
            continue;
        }

        // if is the beginning of a variable
        if is_alpha(c) {

            // create the var string
            let mut var_string = String::new();
            let mut rest = String::new();
            let mut in_var = true;

            // get the variable and the rest of the script
            for j in i..script.len() {
                let c = script.as_bytes()[j] as char;

                // if not alphanum
                if !is_alphanum(c) {
                    in_var = false;
                }

                // if in the variable
                if in_var {
                    var_string.push(c)
                }

                // if not in the variable
                else {
                    rest.push(c);
                }
            }

            // return the values
            *script = rest;
            return Some(Token::Var(var_string))
        }

        // if equivalence
        if c == '=' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(EQUIVALENCE));
        }

        // if implication
        if c == '>' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(IMPLICATION));
        }

        // if exclusive disjunction
        if c == '@' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(EX_DISJUNCTION));
        }

        // if conjunction
        if c == '*' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(CONJUNCTION));
        }

        // if disjunction
        if c == '+' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(DISJUNCTION));
        }

        // if assignment
        if c == ':' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(ASSIGNMENT));
        }

        // if prefix negation
        if c == '~' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(PRE_NEGATION));
        }

        // if postfix variable negation
        if c == '\'' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(POST_VAL_NEGATION));
        }

        // if postfix operator negation
        if c == '!' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(POST_OP_NEGATION));
        }

        // if open parenthesis
        if c == '(' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(OPEN));
        }

        // if close parenthesis
        if c == ')' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(CLOSE));
        }

        // if end line
        if c == ',' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::EndLine);
        }

        // if end script
        if c == '.' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::EndScript);
        }

        // if c is a boolean value
        if c == '1' || c == '0' {

            let l = script.len();
            *script = script[1..l].to_string();
            if c== '1' {
                return Some(Token::Val(true));
            }
            if c=='0' {
                return Some(Token::Val(false));
            }
        }
        
    }

    return None;
}