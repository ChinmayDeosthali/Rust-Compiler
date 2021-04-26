use std::collections::HashMap;

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

    // check parameters starts from here
    let mut index = 3;
    let mut parameter_string = String::new();
    if brackets == "{" {
        while index != sliced_tokens.len() - 2 {
            if !parameter_string.is_empty() {
                parameter_string.push_str(" ");
            }
            parameter_string.push_str(&sliced_tokens[index]);
            index = index + 1;
        }
    }
    else if brackets == ")"{
        while index != sliced_tokens.len() - 1 {
            if !parameter_string.is_empty() {
                parameter_string.push_str(" ");
            }
            parameter_string.push_str(&sliced_tokens[index]);
            index = index + 1;
        }
    }
    
    // Splitting the parameters based on "," and then doing the parameters check on them.
    let split_parameters_vector: Vec<&str> = parameter_string.split(",").collect();
    let mut function_parameter_flag = false;
    // println!("\nlen:{},{:?}\n", split_parameters_vector.len(), split_parameters_vector);

    // if there are no arguments passed in function then make the function_parameter_flag as true. Otherwise make the following checking
    if split_parameters_vector.len() > 1 {
        for (_, each_param) in split_parameters_vector.iter().enumerate() {
            // split the parameters based on space
            let split_parameter: Vec<&str> = each_param.trim().split(" ").collect();
            // println!("split_parameter:{:?}", split_parameter);
            let len_of_split_parameter = split_parameter.len();
            match len_of_split_parameter {
                // for all the other conditions do the normal checking that the arguments have followed the correct way of parameter initialization
                2 => if split_parameter[0] == "int" || split_parameter[0] == "char" || split_parameter[0] == "float" || split_parameter[0] == "double" || split_parameter[0] == "string" {
                        let next_parameter_to_check = split_parameter[1];
                        if next_parameter_to_check.chars().all(|x| x.is_ascii_lowercase()) || next_parameter_to_check.chars().all(|x| x.is_ascii_uppercase()) {
                            function_parameter_flag = true;
                        } else {
                            println!("\nParameter {} is not following the pattern [a-zA-Z]. (Note: Keywords int, float, char, double, string are not allowed.)\n", next_parameter_to_check);
                            function_parameter_flag = false;
                            break;
                        }
                    }
                // default case
                _ => if len_of_split_parameter < 2 || len_of_split_parameter > 2 {
                    println!("\n{:?}:Incorrect way of parameter initialization. (Note:Parameters should be declared in following way for arguments: parameter-type parameter-var)\n",split_parameter);
                    function_parameter_flag = false;
                    break;
                }
                // 1 => if index == 0 {
                //         println!("\nIncorrect initialization of the arguments in function.\n");
                //         function_parameter_flag = false;
                //         break;
                //     } else {
                //         let param_to_check = split_parameter[0];
                //         println!("{}", param_to_check);
                //         // check if the split_parameter is not any of the keywords: int, char, float, double, string. also, check if it follows pattern [a-zA-Z]
                //         if param_to_check != "int" || param_to_check != "char" || param_to_check != "float" || param_to_check != "double" || param_to_check != "string" {
                //             if param_to_check.chars().all(|x| x.is_ascii_lowercase()) || param_to_check.chars().all(|x| x.is_ascii_uppercase()) {
                //                 println!("\nsatisfied!\n");
                //                 function_parameter_flag = true;
                //             } else {
                //                 println!("\nParameter is not following the pattern [a-zA-Z]. (Note: Keywords int, float, char, double, string are not allowed.)\n");
                //                 function_parameter_flag = false;
                //                 break;
                //             }
                //         }
                //     }
            }
        }
    }
    // when function declaration do not have any arguments
    else {
        function_parameter_flag = true;
    }

    // if (check_brackets_flag, function_name_flag, function_parameter_flag) are true then the whole function will return value as true otherwise false.       
    if check_brackets_flag && function_name_flag && function_parameter_flag {
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

// *************************************************************************************************************************************
// Author: Chinmay Deosthali                                                                                                           *               
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: function check_if_parentheses_are_balanced will accept tokenized vector from the syntax analysis and check*
//                           if the input cpp file has syntactically balanced parentheses. The parentheses include                     *
//                           ("{","}","(",")","[","]"). It returns a boolean value indicating if it is balanced or not. Also it        *
//                           returns a hashmap which maps each opening parenthesis index to a closing parenthesis index in the         *
//                           tokenized vector.                                                                                         *                                                                                 * 
// *************************************************************************************************************************************
pub fn check_if_parentheses_are_balanced(tokens: Vec<String>) -> (bool,HashMap<usize,usize>){
    let mut parentheses_map = HashMap::new();
    let mut t = Vec::new();
    let mut s = Vec::new();
    let mut temp = Vec::new();
    let mut flag : bool = true;


    for i in 0..tokens.len(){
        if tokens[i] == "(" || tokens[i] == "[" || tokens[i] == "{" || tokens[i] == ")" || tokens[i] == "]" || tokens[i] == "}"
        {
            if tokens[i] == "(" || tokens[i] == "[" || tokens[i] == "{"
            {
                s.push(tokens[i].clone());
                t.push(i);
                temp.push(tokens[i].clone());
                continue;
            }

            if s.len()==0 {
                flag = false;
                println!("Syntax error: Parenthesis are not balanced.");
                return (flag,parentheses_map);
            }

            if tokens[i]==")"{
                let t1 = t.pop();
                parentheses_map.insert(t1.unwrap(), i);
                let x = s.pop();
                temp.push(tokens[i].clone());
                if x.clone().unwrap() == "{" || x.clone().unwrap() == "[" {
                    flag = false;
                    println!("Syntax error: Parenthesis are not balanced, missing opening (");
                    return (flag,parentheses_map);
                }
            }
            else if tokens[i]=="}"{
                let t1 = t.pop();
                parentheses_map.insert(t1.unwrap(), i);
                let x = s.pop();
                temp.push(tokens[i].clone());
                if x.clone().unwrap() == "(" || x.clone().unwrap() == "[" {
                    flag = false;
                    println!("Syntax error: Parenthesis are not balanced, missing opening Parenthesis");
                    return (flag,parentheses_map);
                }
            }
            else if tokens[i]=="]"{
                let t1 = t.pop();
                parentheses_map.insert(t1.unwrap(), i);
                let x = s.pop();
                temp.push(tokens[i].clone());
                if x.clone().unwrap() == "{" || x.clone().unwrap() == "(" {
                    flag = false;
                    println!("Syntax error: Parenthesis are not balanced, missing opening [");
                    return (flag,parentheses_map);
                }
            }

        }

    }
     (flag,parentheses_map)
}