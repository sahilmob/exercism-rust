use std::{convert::TryInto, str::FromStr};

pub type Value = i32;

type Result<T> = std::result::Result<T, Error>;

pub type ForthResult = Result<()>;

#[derive(Clone, Debug)]
enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    Drop,
    Over,
    Number(Value),
    Call(Value),
}

struct Definition {
    name: String,
    body: Vec<Instruction>,
}

#[derive(Default)]
pub struct Forth {
    dict: Vec<Definition>,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn parse_builtin(word: &str) -> Result<Instruction> {
    match word {
        "+" => Ok(Instruction::Add),
        "-" => Ok(Instruction::Sub),
        "*" => Ok(Instruction::Mul),
        "/" => Ok(Instruction::Div),
        "DUP" => Ok(Instruction::Dup),
        "SWAP" => Ok(Instruction::Swap),
        "DROP" => Ok(Instruction::Drop),
        "OVER" => Ok(Instruction::Over),
        _ => {
            if let Ok(num) = Value::from_str(word) {
                Ok(Instruction::Number(num))
            } else {
                eprintln!("parse_builtin failed");
                Err(Error::UnknownWord)
            }
        }
    }
}

impl Forth {
    fn parse_word<'a>(
        &mut self,
        word: &'a str,
        remaining_input: &mut impl Iterator<Item = &'a str>,
    ) -> ForthResult {
        if word == ":" {
            self.parse_definition(remaining_input)
        } else {
            let instr = self.parse_normal_word(word)?;
            self.eval_instruction(instr)
        }
    }
    fn parse_normal_word(&mut self, word: &str) -> Result<Instruction> {
        if word == ":" || word == ";" {
            Err(Error::InvalidWord)
        } else {
            let canonical = word.to_ascii_uppercase();
            if let Some(call) = self.find_defn(&canonical) {
                Ok(call)
            } else {
                parse_builtin(&canonical)
            }
        }
    }
    fn parse_definition<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> ForthResult {
        if let Some(new_word) = iter.next() {
            if let Ok(_) = Value::from_str(new_word) {
                return Err(Error::InvalidWord);
            }
            let name = new_word.to_ascii_uppercase();
            let mut body = Vec::new();
            for word in iter {
                if word == ";" {
                    self.dict.push(Definition { name, body });
                    return Ok(());
                } else {
                    body.push(self.parse_normal_word(word)?)
                }
            }
        }
        eprintln!("parse_definition failed");
        Err(Error::InvalidWord)
    }
    fn eval_instruction(&mut self, instr: Instruction) -> ForthResult {
        match instr {
            Instruction::Add => self.arith(|a, b| Ok(a + b)),
            Instruction::Sub => self.arith(|a, b| Ok(a - b)),
            Instruction::Mul => self.arith(|a, b| Ok(a * b)),
            Instruction::Div => self.arith(|a, b| {
                if b == 0 {
                    eprintln!("Division by zero");
                    Err(Error::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            }),
            Instruction::Dup => self.dup(),
            Instruction::Swap => self.swap(),
            Instruction::Drop => self.drop(),
            Instruction::Over => self.over(),
            Instruction::Number(n) => {
                self.push(n);
                Ok(())
            }
            Instruction::Call(idx) => self.call(idx),
        }
    }
    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }
    fn pop(&mut self) -> Result<Value> {
        if let Some(v) = self.stack.pop() {
            Ok(v)
        } else {
            eprintln!("Stack underflow!");
            Err(Error::StackUnderflow)
        }
    }
    fn arith<F: FnOnce(Value, Value) -> Result<Value>>(&mut self, op: F) -> ForthResult {
        let rhs = self.pop()?;
        let lhs = self.pop()?;
        self.push(op(lhs, rhs)?);
        Ok(())
    }
    fn dup(&mut self) -> ForthResult {
        let v = self.pop()?;
        self.push(v);
        self.push(v);
        Ok(())
    }
    fn swap(&mut self) -> ForthResult {
        let top = self.pop()?;
        let bottom = self.pop()?;
        self.push(top);
        self.push(bottom);
        Ok(())
    }
    fn drop(&mut self) -> ForthResult {
        self.pop()?;
        Ok(())
    }
    fn over(&mut self) -> ForthResult {
        let top = self.pop()?;
        let bottom = self.pop()?;
        self.push(bottom);
        self.push(top);
        self.push(bottom);
        Ok(())
    }
    fn call(&mut self, idx: Value) -> ForthResult {
        let idx = idx.try_into().unwrap();
        if self.dict.len() <= idx {
            eprintln!(
                "call {} but dict is only of length {}",
                idx,
                self.dict.len()
            );
            Err(Error::UnknownWord)
        } else {
            let def = self.dict[idx].body.clone();
            for instr in def {
                self.eval_instruction(instr)?;
            }
            Ok(())
        }
    }
    fn find_defn(&self, word: &str) -> Option<Instruction> {
        for (idx, defn) in self.dict.iter().enumerate().rev() {
            if defn.name == word {
                return Some(Instruction::Call(idx.try_into().unwrap()));
            }
        }
        None
    }
    pub fn new() -> Forth {
        std::default::Default::default()
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut iter = input.split_ascii_whitespace();
        while let Some(word) = iter.next() {
            self.parse_word(word, &mut iter)?;
        }
        Ok(())
    }
}
