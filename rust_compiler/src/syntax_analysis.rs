// *************************************************************************************************************************************
// Author: Chinmay Kulkarni                                                                                                            *               
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: function check_semi_colon will accept tokenized vector from the syntax analysis and check_function will   *
//                           if the function pattern follows following syntax:                                                         *
//                           return-type function-name (parameter type parameter variable, ...) {}                                     *
// *************************************************************************************************************************************
pub fn slice_the_tokens(tokens: Vec<std::string::String>) -> bool {

    // This code snippet will take only the line which contains function declaration.
    let mut function_declaration_index: Vec<usize> = Vec::new();
    for (index, element) in tokens.iter().enumerate() {
        if element == "int" || element == "float" || element == "char" || element == "double" || element == "void" {
            if tokens[index + 2] == "(" {
                function_declaration_index.push(index);
                let mut var = index + 3;
                while tokens[var]!="EndOfLine" {
                    var = var + 1;
                }
                let sliced_tokens = &tokens[index..var];
                // send the sliced_tokens to actual checking of the function
                let check_function_flag = check_function(sliced_tokens.clone().to_vec());
                if check_function_flag {}
                // else {
                //     return false;
                // }
            }
        }
    }
    return true;
}

fn check_function(sliced_tokens: Vec<std::string::String>) -> bool {
    println!("{:?}", sliced_tokens);
    let mut return_type_flag = false;
    let mut function_name_flag = false;
    // for (index, element) in tokens.iter().enumerate() {
        
    //     // First here we are checking the return-type of the function name consist of following data types: int,float, 
    //     // char, double, void. If this is satisfied then the returnType flag will be set to true.                      
    //     if element == "int" || element == "float" || element == "char" || element == "double" || element == "void" {
    //         return_type_flag = true;
    //     }

    //     if index < tokens.len() - 1 {
    //         // Here we will check for the function name follows aschii values of [a-zA-Z]. If it does then make function_name_flag true else false.element.
    //         if element != "int" && element != "float" && element != "char" && element != "double" && element != "void" && element != "if" && element != "else" && element != "EndOfLine" && element != "else if" || tokens[index + 1] == "(" {
    //             let new_element = element.chars();
    //             // iterate through characters in element
    //             for each_character in new_element {
    //                 if each_character.is_ascii_lowercase() {
    //                     function_name_flag = true;
    //                 }
    //                 else if each_character.is_ascii_uppercase() {
    //                     function_name_flag = true;
    //                 }
    //                 else {
    //                     function_name_flag = false;
    //                 }
    //             }
    //         }
    //     }
    // }

    // // *************************************************************************************************************    
    // // At the end we will check if all the flags (return_type_flag, function_name_flag, function_parameter_flag)   *
    // // are true or not. If they are true then the whole function will return value as true otherwise false.        *
    // // *************************************************************************************************************
    // if return_type_flag && function_name_flag {
    //     return true;
    // }
    println!("\nfunction name should be of pattern [a-zA-Z]\n");
    return false;
}

// *************************************************************************************************************************************
// Author: Chinmay Kulkarni                                                                                                            *               
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: function check_semi_colon will accept tokenized vector from the syntax analysis and check if the function *
//                           file has proper semicolons after each statements or not.(excluding the function declaration line.)        *
// *************************************************************************************************************************************

pub fn check_semi_colon(tokens: Vec<std::string::String>) -> bool {
    let mut count = 0;
    for (index, element) in tokens.iter().enumerate() {
        if element == "EndOfLine" {
            count = count + 1;
            if tokens[index - 1] == "{" || tokens[index - 1] == "}" || tokens[index - 1] == "EndOfLine" {}
            else if tokens[index - 1] == ")" {
                if tokens[index + 1] == "{" {}
                else {
                    println!("\nSemi-colon missing on line number {}\n", count);
                    return false;
                }
            }
            else if tokens[index - 1] != ";" {
                println!("\nSemi-colon missing on line number {}\n", count);
                return false;
            }
        }
    }
    return true;
}

 // println!("tokens:{:?}", tokens);

    // let mut end_of_line_index: Vec<usize> = Vec::new();
    // for (index, element) in tokens.iter().enumerate() {
    //     if element == "EndOfLine" {
    //         end_of_line_index.push(index);
    //     }
    // } 
    // println!("\nend_of_line_index:{:?}\n",end_of_line_index);