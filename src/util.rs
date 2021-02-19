// modules
pub mod tokenize;

// uses
use std::fs::File;
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