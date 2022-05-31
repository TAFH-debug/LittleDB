const EXPR_START_SYM: char = '{';
const EXPR_END_SYM: char = '}';

const ARG_START_SYM: char = '(';
const ARG_END_SYM: char = ')';

#[derive(Debug)]
pub enum LexemeType {
    EXPRESSION,
    ARGUMENT,
    TOKEN
}

#[derive(Debug)]
pub struct Lexeme {
    pub value: String,
    pub lexeme_type: LexemeType,
    pub lexems: Vec<Lexeme>
}

impl Lexeme {
    pub fn new(lexeme_type: LexemeType, lexems: Vec<Lexeme>, value: String) -> Lexeme {
        Self {
            value,
            lexeme_type,
            lexems
        }
    }
}

fn split_arg(arg: String) -> Vec<Lexeme> {
    let mut new = String::new();
    let mut count_arg = 0;
    let mut is_str = false;

    for i in arg.chars() {
        if i == '"' {
            is_str = !is_str;
        }
        if is_str {
            new += i.to_string().as_str();
            continue;
        }
        if i == ARG_START_SYM {
            count_arg += 1
        }
        else if i == ARG_END_SYM {
            count_arg -= 1;
        }
        if count_arg > 0 {
            new += i.to_string().as_str();
        }
        else if i == ',' {
            new += " ";
        }
        else {
            new += i.to_string().as_str();
        }
    }
    split_to_lexems(new)
}

pub fn split_to_lexems(arg: String) -> Vec<Lexeme> {
    let mut exprs = Vec::new();
    let mut expr = String::new();
    let mut expr_count = 0;
    let mut is_expr = false;
    let mut is_string = false;
    let mut is_arg = false;
    let mut arg_count = 0;

    for i in arg.chars() {
        if is_string {
            if i == '"' {
                is_string = false;
            }
            expr += i.to_string().as_str();
            continue;
        }
        if i == '"' {
            is_string = true;
            expr += i.to_string().as_str();
            continue;
        }

        if i == EXPR_START_SYM && !is_expr && !is_arg {
            if !expr.trim().is_empty() {
                exprs.push(Lexeme::new(LexemeType::TOKEN, vec!(), expr));
            }
            expr = String::new();
            is_expr = true;
            continue;
        }

        if i == EXPR_START_SYM && !is_arg {
            expr_count += 1;
        }

        if i == EXPR_END_SYM && !is_arg {
            if expr_count > 0 {
                expr_count -= 1;
                expr += i.to_string().as_str();
                continue;
            }
            is_expr = false;
            exprs.push(Lexeme::new(LexemeType::EXPRESSION,
                                  split_to_lexems(expr.clone()),
                                  expr));
            expr = String::new();
            continue;
        }

        if is_expr {
            expr += i.to_string().as_str();
            continue;
        }

        if i == ARG_START_SYM {
            if is_arg {
                arg_count += 1;
                expr += i.to_string().as_str();
                continue;
            }
            exprs.push(Lexeme::new(LexemeType::TOKEN, vec!(), expr));
            expr = String::new();
            is_arg = true;
            continue;
        }

        if i == ARG_END_SYM {
            if arg_count > 0 {
                arg_count -= 1;
                expr += i.to_string().as_str();
                continue;
            }
            is_arg = false;
            exprs.push(Lexeme::new(LexemeType::ARGUMENT, split_arg(expr.clone()), expr));
            expr = String::new();
            continue;
        }

        if is_arg {
            expr += i.to_string().as_str();
            continue;
        }



        if i.is_ascii_whitespace() {
            if expr.trim().is_empty() {
                continue;
            }
            exprs.push(Lexeme::new(LexemeType::TOKEN, vec!(), expr));
            expr = String::from("");
        }
        else {
            expr += i.to_string().as_str();
        }
    }
    if !expr.trim().is_empty() {
        exprs.push(Lexeme::new(LexemeType::TOKEN, vec!(), expr));
    }
    exprs
}