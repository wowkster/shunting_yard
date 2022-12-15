use std::io::{self, Write};

mod parse;
mod token;

use crate::{
    parse::{to_rpn, ParsingError},
    token::{tokenize, TokenizationError},
};

fn main() -> io::Result<()> {
    print!("Please enter your equation: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_owned();

    let mut tokens = match tokenize(&input) {
        Ok(tokens) => tokens,
        Err(err) => match err {
            TokenizationError::NotAscii => {
                eprintln!("Unexpected non-ascii text in input",);
                std::process::exit(1)
            }
            TokenizationError::Empty => {
                eprintln!("Input is empty!");
                std::process::exit(1)
            }
            TokenizationError::UnexpectedChar(position) => {
                let chars: Vec<_> = input.chars().collect();

                report_error(
                    format!(
                        "Unexpected character `{}` in input",
                        chars.get(position as usize).unwrap()
                    ),
                    &input,
                    position,
                    position + 1,
                )
            }
        },
    };

    println!("{:#?}", tokens);

    let rpn = match to_rpn(&mut tokens) {
        Ok(tokens) => tokens,
        Err(err) => match err {
            ParsingError::UnbalancedParens(token) => {
                let position = token.start;
                report_error(
                    "Unexpected closing parenthesis `)` in input",
                    &input,
                    position,
                    position + 1,
                )
            }
        },
    };

    println!("{:#?}", rpn);

    Ok(())
}

pub fn report_error(error: impl ToString, input: &String, start: u32, end: u32) -> ! {
    eprintln!("Error: {}", error.to_string());

    eprintln!("{input}");

    eprintln!(
        "{}{}",
        " ".repeat(start as usize),
        "^".repeat((end - start) as usize)
    );

    std::process::exit(1);
}
