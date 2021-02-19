// modules
mod util;

// uses
use std::io::Read;
use util::*;
use util::tokenize::*;

/**
 * main method
 */
fn main() {

    // options
    let mut out_name: Option<String> = None;
    let mut in_method: Option<In> = None;

    // iterate over command line arguments to get values
    let args: Vec<String> = std::env::args().collect();
    for i in 1..args.len() {

        // match the argument
        match args[i].as_str() {

            // cli option
            "-c" => {
                if in_method.is_some() {
                    print_usage();
                    return;
                }
                in_method = Some(In::Cli(
                    if i >= args.len() {
                        print_usage();
                        return;
                    }
                    else {
                        args[i].clone()
                    }
                ));
            }

            // output option
            "-o" => {
                out_name = Some(
                    if i+1 >= args.len() {
                        print_usage();
                        return;
                    }
                    else {
                        args[i+1].clone()
                    }
                );
            },

            // filename option
            filename => {
                if !in_method.is_some() {
                    in_method = Some(In::File(filename.to_string()))
                }
                
            }
        }
    }

    // if no input options given, default to stdin
    if in_method.is_none() {
        in_method = Some(In::Stdin);
    }

    // if an output file is given, create an out object
    let mut output = match out_name {
        None => Output::stdout(),
        Some(filename) => match Output::file(filename) {
            Err(e) => {
                print_error(e);
                return;
            },
            Ok(file) => file
        }
    };

    // get the script from in_method
    let commented_script = match in_method {
        None => {
            let mut script = String::new();
            match std::io::stdin().read_to_string(&mut script) {
                Err(e) =>  {
                    print_error(e);
                    return;
                },
                Ok(_)=>(),
            };
            script
        },
        Some(in_method) => match in_method {
            In::Stdin => {
                let mut script = String::new();
                match std::io::stdin().read_to_string(&mut script) {
                    Err(e) =>  {
                        print_error(e);
                        return;
                    },
                    Ok(_)=>(),
                };
                script
            },
            In::Cli(script) => script,
            In::File(filename) => {
                match std::fs::read_to_string(filename) {
                    Err(e) => {
                        print_error(e);
                        return;
                    },
                    Ok(script) => script
                }
            }
        }
    };

    // remove the comment from the script
    let script = remove_comments(commented_script);

    // print out the top of the truth table
    let mut line_num: usize = 0;
    for line in script.split(',') {
        line_num+=1;

        // print the separator
        if line_num > 1 {
            output.write(",\t");
        }
        
        // parse the var
        let var = match get_token(line.to_string()) {
            (Some(Token::Var(var)),_) => var,
            _ => {
                println!("Error: Couldn't parse variable in line {}:\n\t\"{}\"",line_num,line);
                return;
            }
        };

        output.write(var);
    }
    output.writeln("");

    output.writeln("Done!");
}
