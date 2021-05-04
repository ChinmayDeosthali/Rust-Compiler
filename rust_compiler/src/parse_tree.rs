use std::collections::HashMap;
use std::collections::HashSet;

// *************************************************************************************************************************************
// Author: Chinmay Deosthali                                                                                                           *                                                                                                                     
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: This method(parse_tree) takes in Vector Tokens as input and generates a parse tree as the output.         *
// *************************************************************************************************************************************


pub fn parse_tree(
    start: usize,
    end: usize,
    tokens: Vec<String>,
    parentheses_map: HashMap<usize, usize>,
    new_var_type_map: HashMap<String, String>,
    input_indent: String,
) {
    let mut special_char_set = HashSet::new();
    special_char_set.insert("main");
    special_char_set.insert("int");
    special_char_set.insert("char");
    special_char_set.insert("string");
    special_char_set.insert("float");
    special_char_set.insert("return");
    special_char_set.insert("void");
    special_char_set.insert("if");
    special_char_set.insert("else");
    special_char_set.insert(",");
    special_char_set.insert("\"");
    special_char_set.insert(";");
    special_char_set.insert("{");
    special_char_set.insert("}");
    special_char_set.insert("(");
    special_char_set.insert(")");
    special_char_set.insert("[");
    special_char_set.insert("]");
    special_char_set.insert("=");
    special_char_set.insert("+");
    special_char_set.insert("-");
    special_char_set.insert("/");
    special_char_set.insert("EndOfLine");

    let mut var_type_map = new_var_type_map;
    let mut indent = input_indent.clone();

    let mut i = start;
    while i < end {
 
        let token_val_str: &str = &tokens[i].clone()[..];

        if (tokens[i] == "int")
            || (tokens[i] == "string")
            || (tokens[i] == "char")
            || (tokens[i] == "float")
            || (tokens[i] == "void")
        {
            if tokens[i + 2] == "(" {

                let close_brac_num = parentheses_map.get(&(i + 2)).unwrap().clone();
                // println!("close_brac_num {}",close_brac_num);
                println!("function {}", tokens[i + 1]);
                println!("{} return {}", indent, tokens[i]);
                if close_brac_num != (i + 3) {
                    // println!("here {} {}",i+3,close_brac_num);
                    for j in i + 3..close_brac_num {
                        // println!("here1");
                        if j == i + 3 {
                            // println!("here2");
                            var_type_map.insert(tokens[j + 1].clone(), tokens[j].clone());
                            println!("{} param {} {}", indent, tokens[j], tokens[j + 1]);
                        } else if tokens[j] == "," {
                            // println!("here3");
                            var_type_map.insert(tokens[j + 2].clone(), tokens[j + 1].clone());
                            // println!("{:?}",var_type_map);
                            println!("{} param {} {}", indent, tokens[j + 1], tokens[j + 2]);
                        }
                    }
                }
                let mut open_par_num = close_brac_num + 1;
                if tokens[close_brac_num + 1] == "EndOfLine" {
                    open_par_num = close_brac_num + 2;
                }

                let close_par_num = parentheses_map.get(&(open_par_num)).unwrap().clone();
                indent.push('-');
                // println!("open_par_num: {}, close_par_num: {}, indent: {}",open_par_num,close_par_num, indent);
                parse_tree(
                    (open_par_num + 1).clone(),
                    (close_par_num).clone(),
                    tokens.clone(),
                    parentheses_map.clone(),
                    var_type_map.clone(),
                    indent.clone(),
                );
                i = close_par_num;
                indent.pop();
            } else if (tokens[i + 2] == ",") || (tokens[i + 2] == ";") {
                if tokens[i + 2] == "," {
                    let mut k = i + 2;
                    while tokens[k] != ";" {
                        var_type_map.insert(tokens[k - 1].clone(), tokens[i].clone());
                        println!("{} local {} {}", indent, tokens[i], tokens[k - 1]);
                        k = k + 2;
                    }
                    var_type_map.insert(tokens[k - 1].clone(), tokens[i].clone());
                    println!("{} local {} {}", indent, tokens[i], tokens[k - 1]);
                }
            }
        } else if tokens[i] == "if" {
            let open_brac_num = i + 1;
            let close_brac_num = parentheses_map.get(&(open_brac_num)).unwrap().clone();
            let mut open_par_num = close_brac_num + 1;
            if tokens[close_brac_num + 1] == "EndOfLine" {
                open_par_num = close_brac_num + 2;
            }
            if (close_brac_num - open_brac_num) == 4 {
                let mut op = "";
                if tokens[open_brac_num + 2] == "<" {
                    op = "lt";
                } else if tokens[open_brac_num + 2] == ">" {
                    op = "gt";
                }
                println!(
                    "{} if {} {} {}",
                    indent,
                    tokens[open_brac_num + 1],
                    op,
                    tokens[open_brac_num + 3]
                )
            } else if (close_brac_num - open_brac_num) == 5 {
                let mut op = "";
                if tokens[open_brac_num + 2] == "<" {
                    op = "lte";
                } else if tokens[open_brac_num + 2] == ">" {
                    op = "gte";
                } else if tokens[open_brac_num + 2] == "=" {
                    op = "eq";
                }
                println!(
                    "{} if {} {} {}",
                    indent,
                    tokens[open_brac_num + 1],
                    op,
                    tokens[open_brac_num + 3]
                )
            }
            let close_par_num = parentheses_map.get(&(open_par_num)).unwrap().clone();
            indent.push('-');
            // println!("open_par_num: {}, close_par_num: {}, indent: {}",open_par_num,close_par_num, indent);
            parse_tree(
                (open_par_num + 1).clone(),
                (close_par_num).clone(),
                tokens.clone(),
                parentheses_map.clone(),
                var_type_map.clone(),
                indent.clone(),
            );
            i = close_par_num;
            indent.pop();
        } else if tokens[i] == "else" {
            if tokens[i + 1] == "if"
            {
                let  open_brac_num =i + 2;
                let  close_brac_num = parentheses_map.get(&(open_brac_num)).unwrap().clone();
                let mut open_par_num = close_brac_num + 1;
                if tokens[close_brac_num + 1] == "EndOfLine" {
                    open_par_num = close_brac_num + 2;
                }
                if (close_brac_num - open_brac_num) == 4
                {
                    let mut op ="";
                    if tokens[open_brac_num + 2] == "<"
                    {
                        op = "lt";
                    }
                    else if tokens[open_brac_num + 2] == ">"
                    {
                        op = "gt";
                    }
                    println!("{} else if {} {} {}", indent, tokens[open_brac_num + 1], op, tokens[open_brac_num + 3])
                }
                else if (close_brac_num - open_brac_num) == 5
                {
                    let mut op ="";
                    if tokens[open_brac_num + 2] == "<"
                    {
                        op = "lte";
                    }
                    else if tokens[open_brac_num + 2] == ">"
                    {
                        op = "gte";
                    }
                    else if tokens[open_brac_num + 2] == "="
                    {
                        op = "eq";
                    }
                    println!("{} else if {} {} {}", indent, tokens[open_brac_num + 1], op, tokens[open_brac_num + 3])
                }
                let  close_par_num = parentheses_map.get(&(open_par_num)).unwrap().clone();
                indent.push('-');
                // println!("open_par_num: {}, close_par_num: {}, indent: {}",open_par_num,close_par_num, indent);
                parse_tree((open_par_num + 1).clone(), (close_par_num).clone(), tokens.clone(), parentheses_map.clone(), var_type_map.clone(), indent.clone());
                i = close_par_num;
                indent.pop();
            }
        } else if !special_char_set.contains(&token_val_str) {
            let s1 = String::from(token_val_str);
            let test = s1.parse::<f64>();
            let mut number_flag = false;
            let mut _fn_name_flag = false;
            match test {
                Ok(_ok) => number_flag = true,
                Err(_e) => _fn_name_flag = true,
            }

            if number_flag == false {
                if tokens[i + 1] == "(" {
                    let fname = tokens[i].clone();
                    let open_brac_num = i + 1;
                    let close_brac_num = parentheses_map.get(&(open_brac_num)).unwrap().clone();
                    println!("{} call fn {}", indent, tokens[i]);
                    indent.push('-');
                    let mut j = open_brac_num + 1;
                    // for mut j in open_brac_num + 1 .. close_brac_num {
                    while j < close_brac_num {
                        if tokens[j] != "," {
                            if tokens[j] == "\"" {
                                let mut close_quotes = j;
                                let mut string_val = "\"".to_string();
                                for k in j + 1..end {
                                    string_val.push_str(&tokens[k][..]);
                                    if tokens[k] == "\"" {
                                        close_quotes = k;
                                        break;
                                    }
                                }
                                println!("{} arg String {}", indent, string_val);
                                j = close_quotes + 1;
                                i = j;
                                continue;
                            } else {
                                let mut key = "";
                                if fname == "scanf" {
                                    if &tokens[j][0..1] == "&" {
                                        key = &tokens[j][1..2];
                                    }
                                } else {
                                    key = &tokens[j][..];
                                }
                                let key_string = key.to_string();
                                // println!("keystring {}",key_string);
                                // println!("var_type_map : {:?}",var_type_map);
                                let var_type = var_type_map.get(&(key_string)).unwrap().clone();
                                println!("{} arg {} {}", indent, var_type, key_string);
                            }
                        }
                        j = j + 1;
                    }
                    indent.pop();
                }
            }
        }

        i = i + 1;
    }
}
