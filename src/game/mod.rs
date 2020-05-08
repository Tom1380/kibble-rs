mod maze;

use {
    std::io::Write,
    termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor},
};

pub use maze::clear_screen;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub enum Key {
    Arrow(Direction),
    Space,
}

fn read_key(g: &getch::Getch) -> Option<Key> {
    match g.getch() {
        Ok(87) | Ok(119) => Some(Key::Arrow(Direction::Up)),
        Ok(68) | Ok(100) => Some(Key::Arrow(Direction::Right)),
        Ok(83) | Ok(115) => Some(Key::Arrow(Direction::Down)),
        Ok(65) | Ok(97) => Some(Key::Arrow(Direction::Left)),
        Ok(32) => Some(Key::Space),
        _ => None,
    }
}

pub fn play() {
    let mut m = maze::Maze::with_size(20);
    let g = getch::Getch::new();
    m.print();
    while maze::Value::End != m.read_cur().value {
        if let Some(k) = read_key(&g) {
            match k {
                Key::Arrow(d) => {
                    if m.move_in_direction(d) {
                        m.print()
                    }
                }
                Key::Space => return,
            }
        }
    }
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    buffer
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .unwrap();
    writeln!(buffer, "HAI VINTO NEGRO.").unwrap();
    bufwtr.print(&buffer).unwrap();
}
