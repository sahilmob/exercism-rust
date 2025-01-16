use std::cmp::Ordering::*;

struct State {
    first_char: char,
    second_char: char,
    digits: i16,
}

impl State {
    pub fn new(&mut self) -> String {
        match self.digits.cmp(&999) {
            Less => {
                self.digits += 1;
                let digits = self.digits;
                let first_char = self.first_char;
                let second_char = self.second_char;
                format!("{first_char}{second_char}{digits}")
            }
            Equal => {
                self.digits = 100;
                let digits = self.digits;
                let first_char = self.first_char;
                let second_char = self.second_char;
                match second_char == 'Z' {
                    true => {
                        self.first_char = *('A'..'Z')
                            .filter(|c| c.gt(&first_char))
                            .collect::<Vec<char>>()
                            .first()
                            .unwrap();
                        self.second_char = 'A'
                    }
                    false => {
                        self.second_char = *('A'..'Z')
                            .filter(|c| c.gt(&second_char))
                            .collect::<Vec<char>>()
                            .first()
                            .unwrap()
                    }
                }
                format!("{first_char}{second_char}{digits}")
            }
            Greater => panic!(),
        }
    }
}

static mut STATE: State = State {
    digits: 99,
    first_char: 'A',
    second_char: 'A',
};

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        unsafe { Self { name: STATE.new() } }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        unsafe { self.name = STATE.new() }
    }
}
