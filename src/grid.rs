use std::fmt;
use ncurses::{clear, mvprintw, refresh};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<bool>>,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(w: usize, h: usize) -> Grid {
        Grid {
            cells: vec![vec![false; h]; w],
        }
    }

    pub fn from_vec(s: Vec<Vec<bool>>) -> Grid {
        Grid { cells: s }
    }

    pub fn from_file(input: &str) -> Grid {
        debug!("opening file {}", input);
        let mut f = File::open(input).expect("error opening file");
        trace!("opened file {:?}", f);

        let mut s = String::new();
        f.read_to_string(&mut s).expect("error reading file");
        trace!("read file {:?}", s);

        Grid::from_string(s.as_str())
    }

    fn from_string(s: &str) -> Grid {
        let mut v: Vec<Vec<bool>> = Vec::new();
        for (x, line) in s.lines().enumerate() {
            trace!("read line {:?}", line);
            let mut linevec = vec![false; line.len()];
            for (y, cell) in line.chars().enumerate() {
                trace!("read cell {} {} {:?}", x, y, cell);
                linevec[y] = match cell {
                    '0' => false,
                    _ => true,
                };
            }
            v.push(linevec);
        }
        trace!("new vector: {:?}", v);
        Grid { cells: v }
    }

    pub fn save_to_file(&self, output: &str) {
        let mut f = File::create(output).expect("error opening file");
        debug!("opened file {:?}", f);

        // f.read_to_string(&mut s).expect("error reading file");
        f.write_fmt(format_args!("{}", self))
            .expect("error writing file");
        debug!("wrote file {:?}", f);
    }
    pub fn set_cell(&mut self, x: usize, y: usize, v: bool) {
        self.cells[x][y] = v;
    }

    pub fn flip_cell(&mut self, x: usize, y: usize) {
        self.cells[x][y] = !self.cells[x][y];
    }

    pub fn update(&self) {
        debug!("Update");
        clear();
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell {
                    debug!(
                        "({}, {}) = {}",
                        x as i32,
                        y as i32,
                        match *cell {
                            true => "1",
                            _ => "0",
                        }
                    );
                    mvprintw(y as i32, x as i32, "0");
                }
            }
        }
        refresh();
    }

    pub fn step(&mut self) {
        debug!("Step");
        trace!("diff: {:?}", self.cells);
        let prev_state = self.cells.clone();
        let new_cells = prev_state.iter().enumerate().map(|(x, row)| {
            // debug!("{}", x);
            let m = row.iter().enumerate().map(|(y, cell)| {
                match count_live_neighbours(&prev_state, x, y) {
                    0 | 1 => false,
                    2 => *cell,
                    3 => true,
                    _ => false,
                }
            });
            let coll: Vec<bool> = m.collect();
            return coll;
        });
        self.cells = new_cells.collect();
        trace!("diff: {:?}", self.cells);
    }
}

fn count_live_neighbours(state: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut n = 0;
    let xx: i32 = x as i32;
    let yy: i32 = y as i32;

    for i in xx - 1..xx + 2 {
        if let Some(row) = state.get(i as usize) {
            for j in yy - 1..yy + 2 {
                if let Some(cell) = row.get(j as usize) {
                    trace!("check for {},{}: {},{}={}", x, y, i, j, cell);
                    if (i != xx || j != yy) && *cell {
                        n += 1;
                    }
                }
            }
        }
    }
    if n > 0 {
        trace!("Neighbours: {}", n);
    }
    return n;
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for (_x, row) in self.cells.iter().enumerate() {
            for (_y, cell) in row.iter().enumerate() {
                out = format!(
                    "{}{}",
                    out,
                    match *cell {
                        true => "1",
                        _ => "0",
                    }
                );
            }
            out = format!("{}\n", out,);
        }
        write!(f, "{}", out)
    }
}
