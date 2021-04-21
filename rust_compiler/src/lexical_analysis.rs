use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// *************************************************************************************************************************************
// Author: Chinmay Deosthali                                                                                                           *                                                                                                                     
// Project: Designing a compiler with Rust                                                                                             *
// Code Snippet Description: This method(tokenize) reads each line of the input cpp file and creates a vector of tokens where each     *
//                           element of a vector represents a separate token. The vector of tokens generated will be used for syntax   *
//                           analysis in the next release.                                                                             *
// *************************************************************************************************************************************

pub fn tokenize(input: &str) -> Vec<String>{

    let mut tokens = Vec::new();
    
    if let Ok(lines) = read_lines(input) {
         for line in lines {
            if let Ok(ip) = line {

                let  iter = ip.split_whitespace();
                
                for intermediate in iter{
                    // println!("intermediate: {}", intermediate);
                    let mut indexes = Vec::new();
                    let mut _sc_flag: bool = false;
                    
                    if intermediate.find('(') != None {
                        let c = intermediate.find('(').unwrap();
                        // println!("  ( found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('(')!=None {
                            let s = subintermediate.find('(').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                        
                    }
                    if intermediate.find('=') != None {
                        let c = intermediate.find('=').unwrap();
                        // println!("  = found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('=')!=None {
                            let s = subintermediate.find('=').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                        
                    }
                    if intermediate.find('+') != None {
                        let c = intermediate.find('+').unwrap();
                        // println!("  + found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('+')!=None {
                            let s = subintermediate.find('+').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                        
                    }
                    if intermediate.find('-') != None {
                        let c = intermediate.find('-').unwrap();
                        // println!("  - found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('-')!=None {
                            let s = subintermediate.find('-').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                        
                    }
                    if intermediate.find('<') != None {
                        let c = intermediate.find('<').unwrap();
                        // println!("  < found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('<')!=None {
                            let s = subintermediate.find('<').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                        
                    }
                    if intermediate.find('>') != None {
                        let c = intermediate.find('>').unwrap();
                        // println!("  > found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('>')!=None {
                            let s = subintermediate.find('>').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                        
                    }
                    if intermediate.find('/') != None {
                        let c = intermediate.find('/').unwrap();
                        // println!("  / found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('/')!=None {
                            let s = subintermediate.find('/').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    if intermediate.find(')') != None {
                        let c = intermediate.find(')').unwrap();
                        // println!("  ) found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find(')')!=None {
                            let s = subintermediate.find(')').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    if intermediate.find(',') != None {
                        let c = intermediate.find(',').unwrap();
                        // println!("  , found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find(',')!=None {
                            let s = subintermediate.find(',').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    if intermediate.find(';') != None {
                        let c = intermediate.find(';').unwrap();
                        // println!("  ; found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find(';')!=None {
                            let s = subintermediate.find(';').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    if intermediate.find('[') != None {
                        let c = intermediate.find('[').unwrap();
                        // println!("  [ found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('[')!=None {
                            let s = subintermediate.find('[').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    if intermediate.find(']') != None {
                        let c = intermediate.find(']').unwrap();
                        // println!("  ] found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find(']')!=None {
                            let s = subintermediate.find(']').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                        
                    }
                    if intermediate.find('{') != None {
                        let c = intermediate.find('{').unwrap();
                        // println!("  opening parenthesis found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('{')!=None {
                            let s = subintermediate.find('{').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    if intermediate.find('}') != None {
                        let c = intermediate.find('}').unwrap();
                        // println!("  closing parenthesis found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('}')!=None {
                            let s = subintermediate.find('}').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    if intermediate.find('"') != None {
                        let c = intermediate.find('"').unwrap();
                        // println!("  \" found at position {}",c);
                        indexes.push(c);
                        _sc_flag = true;
                        let b = intermediate.to_string();
                        let mut subintermediate = &b[c+1..];
                        while subintermediate.find('"')!=None {
                            let s = subintermediate.find('"').unwrap();
                            indexes.push(c+1+s);
                            subintermediate = &subintermediate[s+1..];
                        }
                    }
                    
                   // println!("  indexes: {:?}",indexes);

                    if _sc_flag == false{
                        if intermediate.len() > 0{
                            tokens.push(intermediate.to_string());
                        }
                    }
                    else{
                        indexes.sort();
                        
                        if intermediate.len() == 1 {
                            tokens.push(intermediate.to_string());
                        }
                        else{
                            for i in 0..indexes.len(){
                                if i.clone()==0 {
                                    if indexes[i.clone()] != 0{
                                        let mut _s = String::new();
                                        _s = intermediate[0..indexes[i.clone()]].to_string();
                                        if _s.len() !=0{
                                            tokens.push(_s);
                                        }
                                        let mut _a = String::new();
                                        _a = intermediate.chars().nth(indexes[i.clone()]).unwrap().to_string();
                                        if _a.len() !=0{
                                            tokens.push(_a);
                                        }
                                    }
                                    else{
                                        let mut _s = String::new();
                                        _s = intermediate.chars().nth(0).unwrap().to_string();
                                        if _s.len() !=0{
                                            tokens.push(_s);
                                        }
                                    }
                                    if indexes.len() == 1 {
                                        if indexes[i.clone()] != (intermediate.len()-1){
 
                                            let mut _s = String::new();
                                            _s = intermediate[indexes[i.clone()]+1..].to_string();
                                            if _s.len() !=0{
                                                tokens.push(_s);
                                            }
                                        }
                                    }
                                }
                                else if i.clone() == (indexes.len()-1){
                                    let mut _a = String::new();
                                    _a = intermediate[indexes[(i.clone()-1)]+1..indexes[i.clone()]].to_string();
                                    let mut _b = String::new();
                                    _b = intermediate[indexes[i.clone()]+1..].to_string();
                                    if (indexes[i.clone()-1]+1) != indexes[i.clone()]{
                                        if _a.len() !=0{
                                            tokens.push(_a);
                                        }
                                    }
                                    let mut _s = String::new();
                                    _s = intermediate.chars().nth(indexes[i]).unwrap().to_string();
                                    if _s.len() !=0{
                                        tokens.push(_s);
                                    }
                                    if _b.len() !=0{
                                        tokens.push(_b);
                                    }

                                }
                                else{
                                    let mut _b = String::new();
                                    _b = intermediate[(indexes[i.clone()-1]+1)..indexes[i.clone()]].to_string();
                                    if _b.len() !=0{
                                        tokens.push(_b);
                                    }
                                    let mut _s = String::new();
                                    _s = intermediate.chars().nth(indexes[i]).unwrap().to_string();
                                    if _s.len() !=0{
                                        tokens.push(_s);
                                    }
                                }
                            }
                        }


                    }
                }
            }
            tokens.push("EndOfLine".to_string());
        }
        
        //println!("Tokens: {:?}",tokens);
        // Release 0.3 changes start.
        // println!("\n\nLEXICAL ANALYSIS OUTPUT:");
        // println!();
        // for token in tokens.iter(){
        //     println!{"{}",token};
        // }
        // Release 0.3 changes start.
    }    
    tokens
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}