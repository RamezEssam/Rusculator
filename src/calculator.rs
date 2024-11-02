use regex::Regex;
use crate::VecDeque;


#[derive(Debug)]
enum Token {
    Number(f64),
    Operator(char),
    Function(String),
    OpenParen,
    CloseParen,
}


fn tokenize(expression: &str) -> Vec<Token> {
    let re = Regex::new(r"\d+\.?\d*|[+\-^*/()]|cos|sin|tan").unwrap();
    let mut tokens: Vec<Token> = Vec::new();

    for cap in re.captures_iter(expression) {
        let part = cap.get(0).unwrap().as_str();
        tokens.push(
            match part {
                "(" => Token::OpenParen,
                ")" => Token::CloseParen,
                "+" | "-" | "*" | "/" | "^" => Token::Operator(part.chars().next().unwrap()),
                "cos"| "sin" | "tan" => Token::Function(part.to_string()),
                _ => Token::Number(part.parse().unwrap_or(0.0)),
            }
        );
    }

    tokens
}


fn shunting_yard(tokens: Vec<Token>) -> VecDeque<Token> {
    let mut output = VecDeque::new();
    let mut operators = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push_back(token),
            Token::Function(_) => operators.push(token),
            Token::Operator(op) => {
                while let Some(Token::Operator(top_op)) = operators.last() {
                    if precedence(op) <= precedence(*top_op) {
                        output.push_back(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::OpenParen => operators.push(token),
            Token::CloseParen => {
                while let Some(top) = operators.pop() {
                    if let Token::OpenParen = top {
                        break;
                    } else {
                        output.push_back(top);
                    }
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push_back(op);
    }

    output
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}


fn evaluate_rpn(queue: VecDeque<Token>) -> f64 {
    let mut stack = Vec::new();

    for token in queue {
        match token {
            Token::Number(num) => stack.push(num),
            Token::Operator(op) => {
                let b = stack.pop();
                let a = stack.pop();
                let result = match op {
                    '+' => a.unwrap_or(0.0) + b.unwrap_or(0.0),
                    '-' => a.unwrap_or(0.0) - b.unwrap_or(0.0),
                    '*' => a.unwrap_or(0.0) * b.unwrap_or(0.0),
                    '/' => a.unwrap_or(0.0) / b.unwrap_or(1.0),
                    '^' => a.unwrap_or(0.0).powf(b.unwrap_or(0.0)),
                    _ => panic!("Unsupported operator"),
                };
                stack.push(result);
            }
            Token::Function(func) => {
                let value = stack.pop();
                let result = match func.as_str() {
                    "cos" => value.unwrap_or(0.0).to_radians().cos(),
                    "sin" => value.unwrap_or(0.0).to_radians().sin(),
                    "tan" => value.unwrap_or(0.0).to_radians().tan(),
                    _ => panic!("Unsupported function"),
                };
                stack.push(result);
            }
            _ => {}
        }
    }

    stack.pop().unwrap()
}


pub fn calculate(expr: &str) -> String {
    let tokens = tokenize(expr);
    let rpn = shunting_yard(tokens);
    let result = evaluate_rpn(rpn);
    format!("{}", result)
}