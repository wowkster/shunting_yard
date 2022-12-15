use core::panic;
use std::collections::VecDeque;

pub enum TokenizationError {
    NotAscii,
    Empty,
    UnexpectedChar(u32),
}

pub fn tokenize(input: &String) -> Result<VecDeque<Token>, TokenizationError> {
    if input.is_empty() {
        return Err(TokenizationError::Empty);
    }

    if !input.is_ascii() {
        return Err(TokenizationError::NotAscii);
    }

    let mut chars: VecDeque<_> = input.chars().collect();

    let mut tokens: VecDeque<Token> = VecDeque::new();

    let mut col_number: u32 = 0;

    while !chars.is_empty() {
        let token_start = col_number;
        col_number += 1;

        let char = chars.pop_front().unwrap();
        let char = char.to_ascii_lowercase();

        match (
            char,
            char.is_ascii_alphabetic(),
            char.is_ascii_digit(),
            OperatorKind::from_char(char),
        ) {
            (_, true, _, _) => {
                let rest_of_string = read_until_end_of_identifier(&mut chars, &mut col_number);

                let full_string = format!("{}{}", char, rest_of_string);

                if let Some(function) = FunctionKind::from_string(full_string.clone()) {
                    tokens.push_back(Token {
                        kind: TokenKind::Function(function),
                        start: token_start,
                        end: col_number,
                        value: String::from(full_string.clone()),
                    })
                } else {
                    tokens.push_back(Token {
                        kind: TokenKind::Identifier(full_string.clone()),
                        start: token_start,
                        end: col_number,
                        value: String::from(full_string),
                    })
                }
            }
            (_, _, true, _) => {
                let rest_of_num = read_until_end_of_number(&mut chars, &mut col_number);

                let full_number = format!("{}{}", char, rest_of_num);

                let parsed = full_number
                    .parse::<f32>()
                    .expect("Could not parse float from number string");

                tokens.push_back(Token {
                    kind: TokenKind::Number(parsed),
                    start: token_start,
                    end: col_number,
                    value: String::from(full_number),
                })
            }
            (_, _, _, Some(operator)) => {
                if operator == OperatorKind::Minus {
                    if tokens.len() == 0 {
                        // Should be a negative number
                        let rest_of_num = read_until_end_of_number(&mut chars, &mut col_number);

                        let full_number = format!("{}{}", char, rest_of_num);

                        let parsed = full_number
                            .parse::<f32>()
                            .expect("Could not parse float from number string");

                        tokens.push_back(Token {
                            kind: TokenKind::Number(parsed),
                            start: token_start,
                            end: col_number,
                            value: String::from(full_number),
                        })
                    }

                    let last_token = tokens.back().unwrap();

                    if matches!(last_token.kind, TokenKind::Number(_))
                        || matches!(last_token.kind, TokenKind::RightParenthesis)
                    {
                        // Should be minus

                        tokens.push_back(Token {
                            kind: TokenKind::Operator(operator),
                            start: token_start,
                            end: col_number,
                            value: String::from(char),
                        })
                    } else {
                        // Should be a negative number
                        let rest_of_num = read_until_end_of_number(&mut chars, &mut col_number);

                        let full_number = format!("{}{}", char, rest_of_num);

                        let parsed = full_number
                            .parse::<f32>()
                            .expect("Could not parse float from number string");

                        tokens.push_back(Token {
                            kind: TokenKind::Number(parsed),
                            start: token_start,
                            end: col_number,
                            value: String::from(full_number),
                        })
                    }
                } else {
                    tokens.push_back(Token {
                        kind: TokenKind::Operator(operator),
                        start: token_start,
                        end: col_number,
                        value: String::from(char),
                    })
                }
            }
            ('(', _, _, _) => tokens.push_back(Token {
                kind: TokenKind::LeftParenthesis,
                start: token_start,
                end: col_number,
                value: String::from(char),
            }),
            (')', _, _, _) => tokens.push_back(Token {
                kind: TokenKind::RightParenthesis,
                start: token_start,
                end: col_number,
                value: String::from(char),
            }),
            (' ', _, _, _) => continue,
            (_, _, _, None) => return Err(TokenizationError::UnexpectedChar(token_start)),
        }
    }

    Ok(tokens)
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub start: u32,
    pub end: u32,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number(f32),
    Identifier(String),
    Function(FunctionKind),
    Operator(OperatorKind),
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, PartialEq)]
pub enum OperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorAssociativity {
    Left,
    Right,
}

impl OperatorKind {
    fn from_char(char: char) -> Option<OperatorKind> {
        match char {
            '+' => Some(OperatorKind::Plus),
            '-' => Some(OperatorKind::Minus),
            '*' => Some(OperatorKind::Multiply),
            '/' => Some(OperatorKind::Divide),
            '^' => Some(OperatorKind::Exponent),
            _ => None,
        }
    }

    pub fn associativity(&self) -> OperatorAssociativity {
        match self {
            OperatorKind::Exponent => OperatorAssociativity::Right,
            _ => OperatorAssociativity::Left,
        }
    }
}

pub trait Precedent {
    fn precedence(&self) -> u32;
}

impl Precedent for OperatorKind {
    fn precedence(&self) -> u32 {
        match self {
            OperatorKind::Plus => 0,
            OperatorKind::Minus => 0,
            OperatorKind::Multiply => 1,
            OperatorKind::Divide => 1,
            OperatorKind::Exponent => 2,
        }
    }
}

impl Precedent for Token {
    fn precedence(&self) -> u32 {
        match &self.kind {
            TokenKind::LeftParenthesis => 0,
            TokenKind::RightParenthesis => 0,
            TokenKind::Operator(op) => op.precedence(),
            _ => panic!("Only operators and parens can have a precedence"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum FunctionKind {
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Ln,
    Log2,
    Log10,
}

impl FunctionKind {
    fn from_string(string: String) -> Option<FunctionKind> {
        match string.as_str() {
            "sin" => Some(FunctionKind::Sin),
            "cos" => Some(FunctionKind::Cos),
            "tan" => Some(FunctionKind::Tan),
            "sinh" => Some(FunctionKind::Sinh),
            "cosh" => Some(FunctionKind::Cosh),
            "tanh" => Some(FunctionKind::Tanh),
            "ln" => Some(FunctionKind::Ln),
            "log_2" => Some(FunctionKind::Log2),
            "log_10" => Some(FunctionKind::Log10),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let string = match self {
            FunctionKind::Sin => "sin",
            FunctionKind::Cos => "cos",
            FunctionKind::Tan => "tan",
            FunctionKind::Sinh => "sinh",
            FunctionKind::Cosh => "cosh",
            FunctionKind::Tanh => "tanh",
            FunctionKind::Ln => "ln",
            FunctionKind::Log2 => "log_2",
            FunctionKind::Log10 => "log_10",
        };

        String::from(string)
    }
}

fn read_until_end_of_number(chars: &mut VecDeque<char>, col_number: &mut u32) -> String {
    read_while(
        |char| char.is_ascii_digit() || *char == '.',
        chars,
        col_number,
    )
}

fn read_until_end_of_identifier(chars: &mut VecDeque<char>, col_number: &mut u32) -> String {
    read_while(|char| char.is_ascii_alphabetic(), chars, col_number)
}

fn read_while(
    predicate: fn(char: &char) -> bool,
    chars: &mut VecDeque<char>,
    col_number: &mut u32,
) -> String {
    let mut res = String::new();

    while !chars.is_empty() {
        let peek = chars.front().unwrap();

        if predicate(peek) {
            res.push(chars.pop_front().unwrap());
            *col_number += 1;
        } else {
            break;
        }
    }

    res
}
