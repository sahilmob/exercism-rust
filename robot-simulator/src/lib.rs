// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Clone for Robot {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction: self.direction,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }

        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::West => self.direction = Direction::South,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
        }

        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        dbg!(self.direction);
        match self.direction {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        }

        self
    }

    fn do_instruction(self, i: char) -> Self {
        match i {
            'A' => self.advance(),
            'R' => self.turn_right(),
            'L' => self.turn_left(),
            _ => self,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;

        for i in instructions.chars() {
            robot = robot.do_instruction(i);
        }

        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
