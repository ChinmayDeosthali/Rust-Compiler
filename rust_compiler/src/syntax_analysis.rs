// *************************************************************************************************************************************
// Author: Chinmay Kulkarni                                                                                                            *               
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: function check_semi_colon will accept tokenized vector from the syntax analysis and check if the function *
//                           pattern follows following syntax:                                                                         *
//                           return-type function-name (parameter type parameter variable, ...) {}                                     *
// *************************************************************************************************************************************
pub fn check_function(tokens: Vec<std::string::String>) -> bool {
    return true;
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