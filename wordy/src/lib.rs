use regex::Regex;

struct ParserState {
    index: usize,
    target: String,
    is_error: bool,
    result: String,
    error_message: String,
}

impl ParserState {
    fn new(target: String) -> Self {
        Self {
            target,
            index: 0,
            is_error: false,
            result: String::new(),
            error_message: String::new(),
        }
    }
}

trait ParserTransformer {
    fn run(state: ParserState) -> ParserState;
    fn update_state(state: ParserState, index: usize, result: String) -> ParserState {
        ParserState {
            index,
            result,
            ..state
        }
    }

    fn update_error(
        state: ParserState,
        index: usize,
        is_error: bool,
        error: String,
    ) -> ParserState {
        ParserState {
            index,
            is_error,
            error_message: error,
            ..state
        }
    }
}

struct LettersParserTransformer {}

static PLUS: &str = "plus";
static MINUS: &str = "minus";
static MULTIPLY: &str = "multiplied by";
static DIVIDE: &str = "divided by";
static SPACE: &str = " ";
static QUESTION_MARK: &str = "?";
static NUMBER: &str = r"^-?\d+";
static WORD: &str = r"^[a-zA-Z]+";

fn match_word(command: &str, i: usize) -> bool {
    let r = Regex::new(WORD).unwrap();
    r.is_match(&command[i..])
}

fn match_num(command: &str, i: usize) -> bool {
    let r = Regex::new(NUMBER).unwrap();
    r.is_match(&command[i..])
}

fn match_space(command: &str, i: usize) -> bool {
    command[i..].starts_with(SPACE)
}

fn match_question_mark(command: &str, i: usize) -> bool {
    command[i..].starts_with(QUESTION_MARK)
}
fn match_plus(command: &str, i: usize) -> bool {
    command[i..].starts_with(PLUS)
}
fn match_minus(command: &str, i: usize) -> bool {
    command[i..].starts_with(MINUS)
}
fn match_multiply(command: &str, i: usize) -> bool {
    command[i..].starts_with(MULTIPLY)
}
fn match_divide(command: &str, i: usize) -> bool {
    command[i..].starts_with(DIVIDE)
}

fn eat(str: &str, mut i: usize) -> usize {
    i += str.len();

    i
}

fn evaluate(mut tokens: Vec<&str>) -> Option<i32> {
    let mut stack: Vec<String> = Vec::new();

    tokens.reverse();

    while let Some(token) = tokens.pop() {
        stack.push(token.to_string());

        if stack.len() == 3 {
            let right = stack.pop().unwrap().parse::<i32>().unwrap();
            let operator = stack.pop().unwrap();
            let left = stack.pop().unwrap().parse::<i32>().unwrap();

            match operator.as_str() {
                "+" => {
                    let r = left + right;
                    stack.push(format!("{}", r))
                }
                "-" => {
                    let r = left - right;
                    stack.push(format!("{}", r))
                }
                "*" => {
                    let r = left * right;
                    stack.push(format!("{}", r))
                }
                "/" => {
                    let r = left / right;
                    stack.push(format!("{}", r))
                }
                _ => panic!("Unexpected operator {operator}"),
            }
        }
    }
    Some(stack.pop().unwrap().parse::<i32>().unwrap())
}

pub fn answer(command: &str) -> Option<i32> {
    let mut i = 0;
    let mut tokens: Vec<&str> = Vec::new();

    while i < command.len() {
        if match_num(command, i) {
            let r = Regex::new(NUMBER).unwrap();
            let num = r.captures(&command[i..]).unwrap().get(0).unwrap().as_str();
            tokens.push(num);
            i = eat(num, i);
            i = eat(SPACE, i);
            if match_num(command, i) {
                return None;
            }
            if match_word(command, i)
                && !match_plus(command, i)
                && !match_minus(command, i)
                && !match_multiply(command, i)
                && !match_divide(command, i)
            {
                return None;
            }
        } else if match_plus(command, i) {
            tokens.push("+");
            i = eat(PLUS, i);
            i = eat(SPACE, i);
            if !match_num(command, i) {
                return None;
            }
        } else if match_minus(command, i) {
            tokens.push("-");
            i = eat(MINUS, i);
            i = eat(SPACE, i);
            if !match_num(command, i) {
                return None;
            }
        } else if match_multiply(command, i) {
            tokens.push("*");
            i = eat(MULTIPLY, i);
            i = eat(SPACE, i);
            if !match_num(command, i) {
                return None;
            }
        } else if match_divide(command, i) {
            tokens.push("/");
            i = eat(DIVIDE, i);
            i = eat(SPACE, i);
            if !match_num(command, i) {
                return None;
            }
        } else if match_space(command, i) {
            i = eat(SPACE, i);
        } else if match_question_mark(command, i) {
            i = eat(QUESTION_MARK, i);
        } else if match_word(command, i) {
            let r = Regex::new(WORD).unwrap();
            let word = r.captures(&command[i..]).unwrap().get(0).unwrap().as_str();
            i = eat(word, i);
        } else {
            let c = &command[i..];
            panic!("Unexpected token! {c}")
        }
    }

    if !tokens.is_empty() {
        evaluate(tokens)
    } else {
        None
    }
}
