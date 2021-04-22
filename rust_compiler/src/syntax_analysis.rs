// *************************************************************************************************************************************
// Author: Chinmay Kulkarni                                                                                                            *               
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: Function slice_the_tokens will slice the tokens if following condition occurs:                            *
//                           1. if the current element in tokens vector is any one of the following: int, char, float, void, double    *
//                           2. if current element index + 2 is ( --> Which means this is the function declaration line                *
//                           if above 2 conditions are satisfied then we will get the index of the immediate EndOfLine element in      *
//                           tokens
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
                else {
                    return false;
                }
            }
        }
    }
    return true;
}

// *************************************************************************************************************************************
// Author: Chinmay Kulkarni                                                                                                            *               
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: function check_semi_colon will accept tokenized vector from the syntax analysis and check_function will   *
//                           if the function pattern follows following syntax:                                                         *
//                           return-type function-name (parameter type parameter variable, ...) {}                                     *
// *************************************************************************************************************************************
fn check_function(sliced_tokens: Vec<std::string::String>) -> bool {
    println!("\n{:?}\n", sliced_tokens);
    // check if the last element contains "{". If it does then check if second last element contains ")". If it does the make check_brackets_flag as true.
    // if the last element does not contain "{", then check if it contains ")". If it does then make check_brackets_flag as true.
    let mut check_brackets_flag = false;
    let brackets = &sliced_tokens[sliced_tokens.len() - 1];
    
    if brackets == "{" {
        if &sliced_tokens[sliced_tokens.len() - 2] == ")" {
            check_brackets_flag = true;
        }
    }
    else if brackets == ")" {
        check_brackets_flag = true;
    }

    // checking the function name is of the pattern [a-zA-Z]. if it is satisfied then function_name_flag will be set to true.
    let mut function_name_flag = false;
    let fn_name_in_chars = &sliced_tokens[1].to_string();

    for each_character in fn_name_in_chars.chars() {
        if each_character.is_ascii_lowercase() {
            function_name_flag = true;
        }
        else if each_character.is_ascii_uppercase() {
            function_name_flag = true;
        }
        else {
            function_name_flag = false;
            break;
        }
    }

    // At the end we will check if all the flags (return_type_flag, function_name_flag, function_parameter_flag) are true or not. If they are true then the
    // whole function will return value as true otherwise false.       
    if check_brackets_flag && function_name_flag {
        return true;
    }
    println!("\nFunction name should be of pattern: return-type function-name (parameter-type parameter-name, ...) \n");
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