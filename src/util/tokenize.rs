
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
    Op(Op),
    EndLine,
    EndScript
}

/** 
 * Operation enum for operationss
 */
#[derive(PartialEq)]
pub enum Op {
    Assignment,
    Disjunction,
    NegDisjunction,
    Conjunction,
    NegConjunction,
    ExDisjunction,
    NegExDisjunction,
    Implication,
    NegImplication,
    Equivalence,
    NegEquivalence,
    PreNegation,
    PostValNegation,
    PostOpNegation,
    Open,
    Close
}

/**
 * impl block for op
 */
impl Op {

    /**
     * returns the precidence of self
     */
    pub fn prec(&self) -> i32 {
        match self {
            Op::Assignment=>0,
            Op::PreNegation=>1,
            Op::PostValNegation=>1,
            Op::PostOpNegation=>1,
            Op::Disjunction=>2,
            Op::NegDisjunction=>2,
            Op::Conjunction=>3,
            Op::NegConjunction=>3,
            Op::ExDisjunction=>4,
            Op::NegExDisjunction=>4,
            Op::Implication=>5,
            Op::NegImplication=>5,
            Op::Equivalence=>6,
            Op::NegEquivalence=>6,
            Op::Open=>7,
            Op::Close=>-1
        }
    }

    /**
     * 
     */
    pub fn flip(&self) -> io::Result<Op> {
        Ok(match self {
            Op::Disjunction=>Op::NegDisjunction,
            Op::NegDisjunction=>Op::Disjunction,
            Op::Conjunction=>Op::NegConjunction,
            Op::NegConjunction=>Op::Conjunction,
            Op::ExDisjunction=>Op::NegExDisjunction,
            Op::NegExDisjunction=>Op::ExDisjunction,
            Op::Implication=>Op::NegImplication,
            Op::NegImplication=>Op::Implication,
            Op::Equivalence=>Op::NegEquivalence,
            Op::NegEquivalence=>Op::Equivalence,
            _=> {
                return Err(io::Error::new(io::ErrorKind::Other, "operator can't be negated"));
            }
        })
    }
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
        if script.len() > i+1 && [c,script.as_bytes()[i+1] as char] == ['=','='] {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+2..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::Equivalence));
        }

        // if implication
        if script.len() > i+1 && [c,script.as_bytes()[i+1] as char] == ['-','>'] {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+2..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::Implication));
        }

        // if exclusive disjunction
        if c == '@' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::ExDisjunction));
        }

        // if conjunction
        if c == '*' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::Conjunction));
        }

        // if disjunction
        if c == '+' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::Disjunction));
        }

        // if assignment
        if c == '=' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::Assignment));
        }

        // if prefix negation
        if c == '~' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::PreNegation));
        }

        // if postfix variable negation
        if c == '\'' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::PostValNegation));
        }

        // if postfix operator negation
        if c == '!' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::PostOpNegation));
        }

        // if open parenthesis
        if c == '(' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::Open));
        }

        // if close parenthesis
        if c == ')' {

            // create a new string without the token
            let mut rest = String::new();
            for j in i+1..script.len() {
                rest.push(script.as_bytes()[j] as char)
            }

            *script = rest;
            return Some(Token::Op(Op::Close));
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