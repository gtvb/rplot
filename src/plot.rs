#![allow(unused)]
use std::io::{self, Write};

const AXIS_HORIZONTAL_PIPE: char = '─';
const AXIS_VERTICAL_PIPE: char = '│';
const AXIS_CROSS_PIPE: char = '┼';
const POINT: char = '■';

pub struct Plot {
    grid: Vec<char>,
    func_expression: String,
    w: usize,
    h: usize,
}

impl Plot {
    pub fn new(func_expression: String) -> Plot {
        let (w, h) = term_size::dimensions().expect("could not get term dimensions");
        let mut grid: Vec<char> = vec![' '; w * h];

        Plot {
            grid,
            func_expression,
            w,
            h,
        }
    }

    fn map_range(from_range: (f64, f64), to_range: (f64, f64), value: f64) -> usize {
        (to_range.0
            + (value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0))
            .round() as usize
    }

    fn fill(&mut self) {
        let mid_w = self.w / 2;
        let mid_h = self.h / 2;

        for r in 0..self.h {
            for c in 0..self.w {
                if r == mid_h && c == mid_w {
                    print!("{}", AXIS_CROSS_PIPE);
                } else if r == mid_h {
                    print!("{}", AXIS_HORIZONTAL_PIPE);
                } else if c == mid_w {
                    print!("{}", AXIS_VERTICAL_PIPE);
                } else {
                    print!(" ");
                }
                // print!("{}-{} ", r, c);
            }
            println!();
        }
    }
}
