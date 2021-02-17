// modules
mod util;

// uses
use std::io::Read;
use util::*;

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
                    if i >= args.len() {
                        print_usage();
                        return;
                    }
                    else {
                        args[i].clone()
                    }
                );
            },

            // filename option
            filename => {
                if in_method.is_some() {
                    print_usage();
                    return;
                }
                in_method = Some(In::File(filename.to_string()))
            }
        }
    }

    // get the script from in_method
    let script = match in_method {
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
}
