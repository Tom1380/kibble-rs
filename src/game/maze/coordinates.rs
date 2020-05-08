use super::super::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub x: u8,
    pub y: u8,
}

impl Coordinates {
    pub fn from(x: u8, y: u8) -> Coordinates {
        Coordinates { x, y }
    }

    fn north(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn east(&self) -> Coordinates {
        Coordinates {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn south(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn west(&self) -> Coordinates {
        Coordinates {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn peek(&self, d: Direction) -> Coordinates {
        use Direction::*;
        match d {
            Up => self.north(),
            Right => self.east(),
            Down => self.south(),
            Left => self.west(),
        }
    }

    pub fn distance(&self, c: &Coordinates) -> u8 {
        let x1 = self.x as i8;
        let y1 = self.y as i8;
        let x2 = c.x as i8;
        let y2 = c.y as i8;
        ((x1 - x2).abs() + (y1 - y2).abs()) as u8
    }
}
