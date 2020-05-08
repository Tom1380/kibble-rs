use {
    std::io::Write,
    termcolor::{Buffer, Color, ColorSpec, WriteColor},
};

#[derive(Debug, PartialEq)]
pub enum Value {
    Wall,
    Road,
    Start,
    End,
}

impl Value {
    pub fn from(c: char) -> Value {
        use Value::*;
        match c {
            'X' => Wall,
            ' ' => Road,
            'A' => Start,
            'B' => End,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Cell {
    pub value: Value,
    pub stepped_on: bool,
}

impl Cell {
    pub fn from(c: char) -> Cell {
        Cell {
            value: Value::from(c),
            stepped_on: false,
        }
    }

    pub fn mark_as_stepped(&mut self) {
        self.stepped_on = true;
    }

    pub fn print_possibly_hidden(&self, mut buffer: &mut Buffer) {
        let mut t = self.matching_string_and_color();
        if !(self.stepped_on || self.value == Value::Start || self.value == Value::End) {
            t = ("°".to_string(), Color::Rgb(110, 110, 110));
        }

        Self::write_to_buffer(&mut buffer, t.0, t.1);
    }

    pub fn print_not_hidden(&self, mut buffer: &mut Buffer) {
        let t = self.matching_string_and_color();
        Self::write_to_buffer(&mut buffer, t.0, t.1);
    }

    fn matching_string_and_color(&self) -> (String, Color) {
        use Value::*;
        match self.value {
            Wall => ("X".to_string(), Color::Rgb(255, 0, 0)),
            Road => ("O".to_string(), Color::White),
            Start => ("A".to_string(), Color::Rgb(65, 105, 225)),
            End => ("B".to_string(), Color::Rgb(0, 255, 0)),
        }
    }

    fn write_to_buffer(buffer: &mut Buffer, s: String, color: Color) {
        buffer
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        write!(buffer, "{} ", s).unwrap();
    }
}
