use crate::{interpreter::StackType, token::Token, types::*};
use std::fs::File;
use std::io::prelude::*;

fn is_string_numeric(str: String) -> bool {
    let mut result = false;
    for c in str.chars() {
        if !c.is_numeric() {
            result = false;
        }
    }

    if str.parse::<f64>().is_ok() {
        result = true;
    }

    return result;
}

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut result: Vec<Token> = vec![];
        let aschar: Vec<&str> = self.source.split_whitespace().collect();
        let mut iter = aschar.iter();
        let mut index = 0;
        let mut open_scope = 0;

        while let Some(word) = iter.next() {
            match word {
                //&"0" | &"1" | &"2" | &"3" | &"4" | &"5" | &"6" | &"7" | &"8" | &"9" => {}
                &"add" => result.push(Token::Add),
                &"minus" => result.push(Token::Minus),
                &"div" => result.push(Token::Div),
                &"mul" => result.push(Token::Mul),
                &"swap" => result.push(Token::Swap),
                &"rot" => result.push(Token::Rot),
                &"mod" => result.push(Token::Mod),
                &"put" => result.push(Token::Put),

                &"eq" => {
                    result.push(Token::Eq);
                }

                &"noteq" => {
                    // Pop items from stack
                    result.push(Token::Noteq)
                }

                &"bigger" => result.push(Token::Bigger),

                &"smaller" => result.push(Token::Smaller),

                &"then" => result.push(Token::Then),

                &"dup" => result.push(Token::Dup),

                &"true" => result.push(Token::True),

                &"false" => result.push(Token::False),

                &"drop" => result.push(Token::Drop),

                &"str" => {
                    // Next element in word will be a string
                    let content = aschar[index + 1];

                    Self::next(&mut iter, &mut index);
                    result.push(Token::Str(content.to_string()))
                }

                &"times" => result.push(Token::Times),

                &"import" => {
                    let file_name = aschar[index + 1];

                    Self::next(&mut iter, &mut index);

                    // Check filename for std
                    let file_data = match file_name {
                        //"math" => include_str!("../std/math.jsl").to_string(),
                        //"std" => include_str!("../std/std.jsl").to_string(),
                        //"memory" => include_str!("../std/memory.jsl").to_string(),
                        _ => {
                            // read file
                            let mut file = File::open(file_name).unwrap();
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();

                            contents
                        }
                    };

                    result.push(Token::Import(Self::new(file_data).lex()))
                }

                &"let" => {
                    let let_name = aschar[index + 1];
                    Self::next(&mut iter, &mut index);
                    result.push(Token::Let(let_name.to_string()))
                }

                &"set" => {
                    let let_name = aschar[index + 1];
                    Self::next(&mut iter, &mut index);
                    result.push(Token::Set(let_name.to_string()))
                }

                &"fn" => {
                    // first find function name
                    let fn_name = aschar[index + 1];
                    Self::next(&mut iter, &mut index);

                    let mut fn_args: Vec<Let> = vec![];

                    Self::next(&mut iter, &mut index);
                    while aschar[index] != "->" {
                        fn_args.push(Let {
                            name: aschar[index].to_string(),
                            value: StackType::Float(0.0),
                        });
                        Self::next(&mut iter, &mut index);
                    }

                    result.push(Token::Function(Function::new(fn_name.to_string(), fn_args)));
                }

                &"call" => {
                    let name = aschar[index + 1];
                    Self::next(&mut iter, &mut index);
                    result.push(Token::Call(name.to_string()));
                }

                &"mempop" => result.push(Token::Mempop),
                &"memusage" => result.push(Token::Memusage),

                // Array
                &"[" => {
                    let mut array_body = String::new();
                    Self::next(&mut iter, &mut index);

                    while aschar[index] != "]" {
                        array_body.push_str(&(aschar[index].to_owned() + " "));
                        Self::next(&mut iter, &mut index);
                    }

                    result.push(Token::Array(Self::new(array_body).lex()));
                }

                // Scope
                &"{" => {
                    open_scope += 1;
                    let mut scope_body = String::new();
                    Self::next(&mut iter, &mut index);

                    while aschar[index] != "}" || open_scope != 1 {
                        if aschar[index] == "}" {
                            open_scope -= 1;
                        } else if aschar[index] == "{" {
                            open_scope += 1;
                        }

                        scope_body.push_str(&(aschar[index].to_owned() + " "));
                        Self::next(&mut iter, &mut index);
                    }

                    result.push(Token::Scope(Self::new(scope_body).lex()));
                }

                &"object" => {
                    Self::next(&mut iter, &mut index);
                    Self::next(&mut iter, &mut index);

                    let mut properties: Vec<(String, Token)>= Vec::new();

                    while aschar[index] != "}" {
                        if aschar[index + 1] != "=" {
                            panic!("You must define property with equal (=)");
                        }
                        let property_name = aschar[index];
                        let property_value = aschar[index + 2]; // 1 is ':' char

                        properties.push(
                            (property_name.to_owned(), Self::new(property_value.to_owned()).lex().get(0).unwrap().clone())
                        );

                        Self::next(&mut iter, &mut index);
                        Self::next(&mut iter, &mut index);
                        Self::next(&mut iter, &mut index);
                    }

                    result.push(Token::Object(properties));
                }

                &"get" => {
                    let property_name = aschar[index + 1];
                    Self::next(&mut iter, &mut index);
                    result.push(Token::Get(property_name.to_string()));
                }

                _ => {
                    if is_string_numeric(word.to_string()) {
                        result.push(Token::Number(word.parse::<f64>().unwrap()));
                    } else {
                        result.push(Token::Ident(word.to_string()))
                    }
                }
            }
            index += 1;
        }
        return result;
    }

    fn next<T>(iter: &mut T, index: &mut usize)
    where
        T: Iterator,
    {
        *index += 1;
        iter.next();
    }
}
