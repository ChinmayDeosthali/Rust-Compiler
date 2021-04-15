 mod lexical_analysis;

use std::env;
use std::fs::File;
use std::io::prelude::*;

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
                Err(why) => panic!("\n\n{}:{}\n\n", why, &args[1]),
            };
 
             let mut contents = String::new();
            
            

            match file.read_to_string(&mut contents) {
                Ok(_) => println!("\n\nFile Contents\n\n{}", contents),
                Err(why) => panic!("\n\nCould not read the contents of the file at path {} due to {}\n\n", &args[1], why),
            };

            // Release 0.2 changes start.
            // Call the function "tokenize" for Lexical analysis.
            lexical_analysis::tokenize(&args[1]);
            // Release 0.2 changes end.
 
        } else {
            println!("More than required arguments provided:{:?}",args);
        }
 
    } else {
        println!("Please provide a file name!");
    }
}
