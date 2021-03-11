// modules
pub mod tokenize;

// uses
use std::fs::File;
use std::io::{self,Write};
use std::fmt::Display;
use std::collections::HashMap;
use tokenize::*;

/**
 * Prints usage notes for the applications
 */
pub fn print_usage() {
    println!("Usage: ttable <file script | pass directly option>")
}

/**
 * prints an error
 */
pub fn print_error(e: std::io::Error) {
    println!("Error: {}", e);
}

/**
 * helper enum for remove_comments method
 */
enum CommentState {
    None,
    Uniline,
    Multiline
}

/** 
 * removes comments from a script
 */
pub fn remove_comments(old_script: String) -> String {

    // init the new script and comment state
    let mut new_script = String::new();
    let mut comment_state = CommentState::None;

    //iterate over evey char in the old scrip
    for c in old_script.chars() {
        match comment_state {

            // if there isn't actively a comment
            CommentState::None => {
                match c {

                    // if the beginning of a uniline comment
                    '%' => comment_state = CommentState::Uniline,

                    // if the beginning of a multiline comment
                    '[' => comment_state = CommentState::Multiline,
                    
                    // if whitespace
                    ' ' => (),
                    '\t' =>(),
                    '\n' => (),

                    // if not the beginning of a comment
                    c => new_script.push(c)
                }
            }

            // if there is a uniline comment
            CommentState::Uniline => {
                
                // exit the comment state
                if c == '\n' {
                    comment_state = CommentState::None;
                }
            }

            //if there is a multiline comment
            CommentState::Multiline => {

                //exit the comment state
                if c == ']' {
                    comment_state = CommentState::None;
                }
            }
        }
    }
    return new_script;
}

/**
 * enum for the input method
 */
pub enum In {
    Cli(String),
    Stdin,
    File(String)
}

/**
 * struct used to handle output
 */
pub enum Output {
    File {
        file:File
    },
    Stdout
}

/**
 * impl block for output
 */
impl Output {

    // creates a new output object from a filename string
    pub fn file(filename:String)->io::Result<Output> {

        // create the file
        let file = match File::create(filename) {
            Err(e) => return Err(e),
            Ok(file) => file
        };
        return Ok(Output::File{file});
    }
    
    // creates a new output object for stdout
    pub fn stdout()->Output {
        return Output::Stdout;
    }

    /**
     * writes output to self
     */
    pub fn write<S:Display>(&mut self, output: S) {
        match self {

            // if self is a file writer
            Output::File{file} => {
                file.write_fmt(format_args!("{}",output)).unwrap();
            }

            // if self is stdin
            Output::Stdout => {
                print!("{}",output);
            }
        }
    }

    /**
     * writes output to self with a newline at the end
     */
    pub fn writeln<S:Display>(&mut self, output: S) {
        match self {

            // if self is a file writer
            Output::File{file} => {
                file.write_all(format!("{}",output).as_ref()).unwrap();
                file.write_all(&['\n' as u8]).unwrap();
            }

            // if self is stdin
            Output::Stdout => {
                println!("{}",output);
            }
        }
    }

    /**
     * writes the true value
     */
    pub fn write_true(&mut self) {
        self.write(1);
    }

    /**
     * writes the false value
     */
    pub fn write_false(&mut self) {
        self.write(0);
    }
}

fn operate(operator:&str,op_stack:&mut Vec<&str>,val_stack:&mut Vec<bool>,var_stack:&mut Vec<String>,var_map:&mut HashMap<String,bool>) -> io::Result<()> {
    match operator {

        // if assignment
        ASSIGNMENT => {

            var_map.insert(match var_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no variable for asssignment"));
                },
                Some(var) => var
            }, match val_stack.last() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for asssignment"));
                },
                Some(val) => *val
            });
        },

        // if disjunction
        DISJUNCTION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = a || b;
            val_stack.push(result);
        }

        // if neg disjunction
        NEG_DISJUNCTION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = a || b;
            val_stack.push(!result);
        }

        // if conjunction
        CONJUNCTION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = a && b;
            val_stack.push(result);
        }

        // if neg conjunction
        NEG_CONJUNCTION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = a && b;
            val_stack.push(!result);
        }

        // if exclusive disjunction
        EX_DISJUNCTION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = (!a && b) || (a && !b);
            val_stack.push(result);
        }

        // if neg exclusive disjunction
        NEG_EX_DISJUNCTION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = (!a && b) || (a && !b);
            val_stack.push(!result);
        }

        // if implication
        IMPLICATION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = !a || b;
            val_stack.push(result);
        }

        // if neg implication
        NEG_IMPLICATION => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = !a || b;
            val_stack.push(!result);
        }

        // if equivalence
        EQUIVALENCE => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = a==b;
            val_stack.push(result);
        }

        // if neg equivalence
        NEG_EQUIVALENCE => {

            // get values
            let b = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // get values
            let a = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for disjunction"));
                },
                Some(val) => val
            };

            // do calculations
            let result = a==b;
            val_stack.push(!result);
        }

        // if prenegation
        PRE_NEGATION => (),

        // if post var negation
        POST_VAL_NEGATION => {
            let val = match val_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no value for negation"));
                },
                Some(val) => val
            };

            val_stack.push(!val);
        }

        // if post op negation
        POST_OP_NEGATION => {
            let op = match op_stack.pop() {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "no operator for negation"));
                },
                Some(val) => val
            };

            op_stack.push(match flip(op) {
                Err(e) => {
                    return Err(e);
                },
                Ok(op) => op
            });
        }

        // if else
        _ => ()
    };

    return Ok(());
}

/**
 * Executes the script
 */
pub fn execute(output: &mut Output,  mut vals: Vec<bool>, mut var_map: HashMap<String,bool>, lines: &[&str]) -> io::Result<()> {

    // breaks if the slice is empty
    if lines.len() < 1 {

        // print every value in vals
        let mut i = 0;
        while i < vals.len() {
            match vals[i] {
                true => output.write_true(),
                false => output.write_false()
            }

            // add comma and tab
            if i != vals.len()-1 {
                output.write(",\t")
            }

            // increment
            i+=1;
        }

        // write endl
        output.writeln("");

        return Ok(());
    }

    // solve expression
    match evaluate(lines[0].to_string(), &mut var_map)? {
        
        // if variable assigned
        Some(val) =>  {
            // append result to vals
            vals.push(val);
    
            // recurse
            execute(output, vals.clone(), var_map.clone(),&lines[1..lines.len()])?;
        },

        // if variable declared
        None => {

            // append false to new vals
            vals.push(false);
            var_map.insert(lines[0].to_string(),false);

            // recurse once
            execute(output, vals.clone(), var_map.clone(),&lines[1..lines.len()])?;

            // change last in vals to true
            let i = vals.len()-1;
            vals[i] = true;
            var_map.insert(lines[0].to_string(),true);

            // recurse once
            execute(output, vals.clone(), var_map.clone(),&lines[1..lines.len()])?;
        }
    };
    
    return Ok(());
}


fn evaluate(mut expression:String,var_map:&mut HashMap<String,bool>)->io::Result<Option<bool>> {

    // create stacks
    let mut val_stack: Vec<bool> = Vec::new();
    let mut op_stack: Vec<&str> = Vec::new();
    let mut var_stack: Vec<String> = Vec::new();

    // loop over expr
    let mut token = get_token(&mut expression);
    while token.is_some() {

        // perform an action depending on the token
        match token {

            // if none
            None => (),

            // if endline
            Some(Token::EndLine) => (),

            // if endscript
            Some(Token::EndScript) => (),

            // if the token is a value
            Some(Token::Val(val)) => {
                if !op_stack.is_empty() && *op_stack.last().unwrap() == PRE_NEGATION {
                    val_stack.push(!val);
                    op_stack.pop();
                }
                else {
                    val_stack.push(val);
                }
            },

            // if token is a variable
            Some(Token::Var(var)) => {
                match var_map.get(&var) {

                    // if the variable has been assigned.
                    Some(val) => {

                        // if the previous operator is is pre_neg, flip the operator
                        if !op_stack.is_empty() && *op_stack.last().unwrap() == PRE_NEGATION {
                            val_stack.push(!*val);
                            op_stack.pop();
                        }

                        // push the value to the stack
                        else {
                            val_stack.push(*val);
                        }
                    },

                    // if the variable has not been assigned
                    None => {
                        
                        // if there is room for an operator
                        if op_stack.is_empty() && var_stack.is_empty() {
                            var_stack.push(var);
                        }

                        // if there isn't room for an operator.
                        else {
                            return Err(io::Error::new(io::ErrorKind::Other, format!("variable {} not assigned", var)))
                        }
                    }

                }
            },

            // if the token is an operator
            Some(Token::Op(mut op)) => {

                // if the previous operator is is pre_neg, flip the operator
                if !op_stack.is_empty() && *op_stack.last().unwrap() == PRE_NEGATION {
                    match flip(op) {
                        Err(_)=>(),
                        Ok(operator)=>{
                            op = operator;
                            op_stack.pop();
                        }
                    }
                }

                // if the previous operators precidence is lower
                while !op_stack.is_empty() && *op_stack.last().unwrap() != OPEN && prec(op_stack.last().unwrap()).unwrap() > prec(op).unwrap() {

                    // get operator
                    let operator = op_stack.pop().unwrap();

                    // choose operation
                    operate(operator,&mut op_stack,&mut val_stack,&mut var_stack,var_map)?;
                }

                // handle if op is close
                if op == CLOSE {
                    match op_stack.pop() {
                        None=> {
                            return Err(io::Error::new(io::ErrorKind::Other, "unexpected close parenthesis"));
                        }
                        Some(OPEN)=>(),
                        Some(_)=>{
                            return Err(io::Error::new(io::ErrorKind::Other, "unexpected close parenthesis"));
                        }
                    }
                }

                // add the operator to the operator stack
                else {
                    op_stack.push(op);
                }
            }
        }

        // get the next token
        token = get_token(&mut expression);
    }

    // perform the rest of the operations
    while !op_stack.is_empty() {
        let operator = op_stack.pop().unwrap();
        operate(operator,&mut op_stack,&mut val_stack,&mut var_stack,var_map)?;
    }

    // return the result
    if val_stack.is_empty() {
        return Ok(None);
    }
    return Ok(Some(val_stack.pop().unwrap()))
    
}