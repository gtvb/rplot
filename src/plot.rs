use std::cmp::Ordering;
use term_size;

use crate::parser;

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0) + to_range.0
}

fn generate_domain_with_step(lower: f64, upper: f64, step: f64) -> Vec<f64> {
    let mut domain = Vec::new();
    let mut i = lower;
    let mut cmp = i.partial_cmp(&upper).unwrap();
    while cmp == Ordering::Less || cmp == Ordering::Equal {
        domain.push(i);
        i += step;
        cmp = i.partial_cmp(&upper).unwrap();
    }
    domain
}

fn parse_domain(domain_str: &str) -> Vec<f64> {
    let parts: Vec<_> = domain_str
        .split(":")
        .map(|val| val.parse::<f64>().unwrap())
        .collect();
    generate_domain_with_step(parts[0], parts[2], parts[1])
}

fn get_bounds<T: Copy + PartialOrd>(v: &Vec<T>) -> (T, T) {
    let min = v.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let max = v.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

    (*min, *max)
}

pub struct Plot {
    grid: Vec<char>,

    domain: Vec<f64>,
    image: Vec<f64>,

    equation: String,
    term_w: usize,
    term_h: usize,
}

impl Plot {
    pub fn new(equation: &str, domain: &str) -> Plot {
        let (w, h) = match term_size::dimensions() {
            Some((w, h)) => (w, h),
            None => panic!("could not get terminal dimensions"),
        };
        let domain = parse_domain(domain);
        let image = parser::eval_with_variables(equation, &domain);

        Plot {
            grid: vec![' '; w * h],
            domain,
            image,
            equation: String::from(equation),
            term_w: w,
            term_h: h,
        }
    }

    fn fill_points(&mut self) {
        let (d1, d2) = get_bounds(&self.domain);
        let (i1, i2) = get_bounds(&self.image);

        self.domain.iter().zip(self.image.iter()).for_each(|p| {
            let d = map_range((d1, d2), (1.0, self.term_w as f64 - 1.0), *p.0) as usize;
            let i = map_range((i2, i1), (1.0, self.term_h as f64 - 1.0), *p.1) as usize;
            let idx = d + (self.term_w * i);

            self.grid[idx] = '*';
        });
    }

    fn fill_chart_data(&mut self) {
        for i in 0..self.term_h - 1 {
            self.grid[self.term_w * i] = '│';
        }

        for i in 1..self.term_w {
            self.grid[self.term_w * self.term_h - i] = '─';
        }

        self.grid[self.term_w * self.term_h - self.term_w] = '└';
    }

    pub fn plot(&mut self) {
        self.fill_chart_data();
        self.fill_points();
        for (i, el) in self.grid.iter().enumerate() {
            if i % self.term_w + 1 == 0 {
                println!("{}", el);
                continue;
            }

            print!("{}", el);
        }
        println!();
    }
}
