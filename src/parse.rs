use std::collections::VecDeque;

use crate::token::{OperatorAssociativity, Precedent, Token, TokenKind};

pub enum ParsingError {
    UnbalancedParens(Token),
}

pub fn to_rpn(tokens: &mut VecDeque<Token>) -> Result<String, ParsingError> {
    let mut output_queue: VecDeque<Token> = VecDeque::new();
    let mut operator_stack: VecDeque<Token> = VecDeque::new();

    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();

        match token.kind {
            crate::token::TokenKind::Number(_) => output_queue.enqueue(token),
            crate::token::TokenKind::Identifier(_) => output_queue.enqueue(token),
            crate::token::TokenKind::Function(_) => operator_stack.push(token),
            crate::token::TokenKind::Operator(ref o1) => {
                while !operator_stack.is_empty() {
                    let o2 = operator_stack.peek().unwrap();

                    if !(matches!(o2.kind, TokenKind::Operator(_))
                        || matches!(o2.kind, TokenKind::RightParenthesis))
                    {
                        break;
                    }

                    if o2.precedence() > o1.precedence()
                        || (o2.precedence() == o1.precedence()
                            && matches!(o1.associativity(), OperatorAssociativity::Left))
                    {
                        output_queue.enqueue(operator_stack.pop().unwrap());
                    } else {
                        break
                    }
                }

                operator_stack.push(token);
            }
            crate::token::TokenKind::LeftParenthesis => operator_stack.push(token),
            crate::token::TokenKind::RightParenthesis => {
                // Assert stack is balanced
                if operator_stack.is_empty() {
                    return Err(ParsingError::UnbalancedParens(token));
                }

                while !operator_stack.is_empty() {
                    let operator = operator_stack.peek().unwrap();

                    if matches!(operator.kind, TokenKind::LeftParenthesis) {
                        break;
                    } else {
                        output_queue.enqueue(operator_stack.pop().unwrap())
                    }
                }

                // If a left paren was not found, then the parens were not balanced
                if operator_stack.is_empty()
                    || operator_stack.peek().unwrap().kind != TokenKind::LeftParenthesis
                {
                    return Err(ParsingError::UnbalancedParens(token));
                }

                // Pop and discard the left paren
                operator_stack.pop().unwrap();

                if !operator_stack.is_empty()
                    && matches!(operator_stack.peek().unwrap().kind, TokenKind::Function(_))
                {
                    output_queue.enqueue(operator_stack.pop().unwrap());
                }
            }
        }
    }

    while !operator_stack.is_empty() {
        let operator = operator_stack.pop().unwrap();

        if matches!(operator.kind, TokenKind::LeftParenthesis) {
            return Err(ParsingError::UnbalancedParens(operator));
        }

        output_queue.enqueue(operator);
    }

    let strings: Vec<_> = output_queue.iter().map(|t| t.to_string()).collect();
    Ok(strings.join(" "))
}

trait Queue<T> {
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Option<T>;
}

impl<T> Queue<T> for VecDeque<T> {
    fn enqueue(&mut self, value: T) {
        self.push_back(value)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.pop_front()
    }
}

trait Stack<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&mut self) -> Option<&T>;
}

impl<T> Stack<T> for VecDeque<T> {
    fn push(&mut self, value: T) {
        self.push_back(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }

    fn peek(&mut self) -> Option<&T> {
        self.back()
    }
}
