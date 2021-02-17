// uses
use std::fs::File;

// constant variables
const whitespace: [char;3] = [' ','\t','\n'];

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
 * checks if the char is whitespace
 */
pub fn is_whitespace(c:&char)->bool {
    whitespace.iter().any(|ws|ws==c)
}

/**
 * checks if the char is in the alphabet
 */
pub fn alpha(c:char)->bool {
    let val = c as u8;
    return (val>=65 && val<=90) || (val>=97 && val<=122);
}

/**
 * checks if the char is alphanumeric
 */
pub fn alphanum(c:char)->bool {
    let val = c as u8;
    return (val>=65 && val<=90) || (val>=97 && val<=122) || (val>=48 && val<=57) || val == 95;
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