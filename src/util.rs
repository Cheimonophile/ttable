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
                    
                    // if not the beginning of a comment
                    c => new_script.push(c)
                }
            }

            // if there is a uniline comment
            CommentState::Uniline => {
                
                // exit the comment state
                if c == '\n' {
                    comment_state = CommentState::None;
                    new_script.push('\n');
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

// Err(io::Error::new(io::ErrorKind::Other, ""))
pub fn execute(output: &mut Output,  mut vals: Vec<bool>, mut var_map: HashMap<String,bool>, lines: &[&str]) -> io::Result<()> {

    // create stacks
    let val_stack: Vec<bool> = Vec::new();
    let op_stack: Vec<Op> = Vec::new();

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
    let expression = lines[0].to_string();
    
    // if variable assigned
    if val_stack.len()>0 {
        let result = val_stack[0];

        // append result to vals
        vals.push(result);

        // recurse
        execute(output, vals.clone(), var_map.clone(),&lines[1..lines.len()])?;
    }

    // if variable declared
    else {

        // append false to new vals
        vals.push(false);

        // recurse once
        execute(output, vals.clone(), var_map.clone(),&lines[1..lines.len()])?;

        // change last in vals to true
        let i = vals.len()-1;
        vals[i] = true;

        // recurse once
        execute(output, vals.clone(), var_map.clone(),&lines[1..lines.len()])?;
    }
    
    

    


    return Ok(());
}