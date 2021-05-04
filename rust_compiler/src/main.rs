// Release 0.2 changes start
mod lexical_analysis;
// Release 0.2 changes end

// Release 0.3 changes start
mod syntax_analysis;
// Release 0.3 changes end

// Release 1.0 Changes start
mod parse_tree;
// Release 1.0 Changes end


// Release 0.1 changes start
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::io::ErrorKind;
use std::collections::HashMap;

// *************************************************************************************************************************************
// Author: Elle Zeeman                                                                                                                 *               
// Sub-author: Chinmay Kulkarni                                                                                                        *
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: This section of code will accept input from the command line (which will be the path to the input file    *
//                           which we want to check with our compiler). The code will try to open the file, if the file is opened then *
//                           read the contents of the file. If for some reason the file cannot be read, then close the program with    * 
//                           appropriate message. If the number of arguments is more than expected (i.e., more than path to the file)  *
//                           then exit the program with appropriate message.                                                           *
// Reference: https://www.youtube.com/watch?v=nQqraiMymcU                                                                              *
// *************************************************************************************************************************************

fn main() {
    let args: Vec<String> = env::args().collect();
 
    if args.len() > 1 {
        if args.len() == 2 {
            let mut file = match File::open(&args[1]) {
                Ok(file) => file,

                 //changes for github issue #4 start- replace panic!()
                Err(why) => match why.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("File not found");
                        process::exit(0);
                    }
                    ErrorKind::PermissionDenied => {
                        eprintln!("File permission denied");
                        process::exit(0);
                    }
                    _ => {
                        eprintln!("other errors {}", why);
                        process::exit(0);
                    }
                },
            };
 
        //changes for github issue #4 end
             let mut contents = String::new();
            
            

            match file.read_to_string(&mut contents) {
                Ok(_) => println!("\n\nFile Contents\n\n{}", contents),

            //changes for github issue #4 start - replace panic!()
                Err(why) => match why.kind() {
                    _ => {
                        eprintln!("other errors {}", why);
                        process::exit(0);
                    }
                },
                //changes for github issue #4 end
            };

            // Release 0.2 changes start.
            // Call the function "tokenize" for Lexical analysis.
            let tokens = lexical_analysis::tokenize(&args[1]);
            // Release 0.2 changes end.

            // Release 0.3 changes start.   
            let (check_parentheses_flag, parentheses_map) = syntax_analysis::check_if_parentheses_are_balanced(tokens.clone());

            if !check_parentheses_flag{
                process::exit(0x0100);
            }

            let check_function_flag = syntax_analysis::slice_the_tokens(tokens.clone());
            
            if !check_function_flag {
                println!("\nFunction_Declaration:Compilation Error.Exiting the program...\n");
                process::exit(0x0100);
            }
            
            let check_semi_colon_flag = syntax_analysis::check_semi_colon(tokens.clone());
            
            if !check_semi_colon_flag {
                process::exit(0x0100);
            }
         
            // Resease 0.3 changes end.

            // Release 1.0 changes start
            println!("\nParse Tree :\n");
            let var_type_map = HashMap::new();
            parse_tree::parse_tree(0,tokens.len(),tokens.clone(), parentheses_map, var_type_map, "-".to_string());
            // Release 1.0 changes end
            
        } else {
            println!("More than required arguments provided:{:?}",args);
        }
 
    } else {
        println!("Please provide a file name!");
    }
}
// Release 0.1 changes end