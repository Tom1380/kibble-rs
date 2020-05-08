mod cell;
mod coordinates;
mod get_maze;

pub use cell::Value;

use {
    super::Direction,
    cell::Cell,
    coordinates::Coordinates,
    std::io::Write,
    termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor},
};

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        println!("{}", "\n".repeat(30));
    } else {
        print!("\x1b[2J\x1b[1;1H");
    }
}

#[derive(Debug)]
pub struct Maze {
    size: u8,
    map: Vec<Vec<Cell>>,
    cur: Coordinates,
}

impl Maze {
    pub fn with_size(size: u8) -> Maze {
        let raw = get_maze::RawMaze::random_via_api(size);
        let mut m = Maze {
            size: raw.size,
            map: raw
                .map
                .iter()
                .map(|v| v.iter().map(|s| Cell::from(*s)).collect())
                .collect(),
            cur: Coordinates {
                x: raw.start.x + 1,
                y: raw.start.y + 1,
            },
        };
        m.read_cur_mut().mark_as_stepped();
        m
    }

    fn read_cur_mut(&mut self) -> &mut Cell {
        self.read_coordinates_mut(self.cur)
    }

    pub fn read_cur(&self) -> &Cell {
        self.read_coordinates(&self.cur)
    }

    fn read_coordinates_mut(&mut self, c: Coordinates) -> &mut Cell {
        &mut self.map[c.y as usize - 1][c.x as usize - 1]
    }

    fn read_coordinates(&self, c: &Coordinates) -> &Cell {
        &self.map[c.y as usize - 1][c.x as usize - 1]
    }

    fn is_steppable(&self, c: &Coordinates) -> bool {
        if c.y > self.size || c.x > self.size || c.y == 0 || c.x == 0 {
            false
        } else {
            match self.read_coordinates(c).value {
                cell::Value::Wall => false,
                _ => true,
            }
        }
    }

    // Return value is whether the cursor actually moved.
    pub fn move_in_direction(&mut self, d: Direction) -> bool {
        let c = self.cur.peek(d);
        if self.is_steppable(&c) {
            self.cur = c;
            self.read_cur_mut().mark_as_stepped();
            true
        } else {
            false
        }
    }

    pub fn print(&self) {
        clear_screen();
        let bufwtr = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();
        for (x, row) in self.map.iter().enumerate() {
            write!(&mut buffer, "            ").unwrap();
            for (y, cell) in row.iter().enumerate() {
                if self.cur == Coordinates::from(y as u8 + 1, x as u8 + 1) {
                    buffer
                        .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 191, 255))))
                        .unwrap();
                    write!(&mut buffer, "* ").unwrap();
                } else if self
                    .cur
                    .distance(&Coordinates::from(y as u8 + 1, x as u8 + 1))
                    == 1
                {
                    cell.print_not_hidden(&mut buffer);
                } else {
                    cell.print_possibly_hidden(&mut buffer);
                }
            }
            writeln!(&mut buffer, "").unwrap();
        }

        bufwtr.print(&buffer).unwrap();
    }
}
