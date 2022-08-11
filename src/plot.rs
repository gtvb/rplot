use std::cmp::Ordering;
use term_size;

use crate::parser;

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0) + to_range.0
}

fn generate_domain_with_step(lower: f64, upper: f64, step: f64) -> Vec<f64> {
    let mut domain = Vec::new();
    let mut i = lower;
    while i.partial_cmp(&upper).unwrap() == Ordering::Less {
        domain.push(i);
        i += step;
    }
    domain
}

fn parse_domain(domain_str: &str) -> Vec<f64> {
    let parts: Vec<_> = domain_str.split(":").map(|val| val.parse::<f64>().unwrap()).collect();
    generate_domain_with_step(parts[0], parts[2], parts[1])
}

fn get_bounds<T: Copy + PartialOrd>(v: &Vec<T>) -> (T, T) {
    let min = v.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    let max = v.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();

    (*min, *max)
}

pub struct Plot {
    grid: Vec<char>,
    equation: String,
    term_w: usize,
    term_h: usize,
}

impl Plot {
    pub fn new(equation: &str) -> Plot {
        let (w, h) = match term_size::dimensions() {
            Some((w, h)) => (w, h),
            None => panic!("could not get terminal dimensions"),
        };

        Plot {
            grid: vec![' '; w*h],
            equation: String::from(equation),
            term_w: w,
            term_h: h,
        }
    }

    fn fill_points(&mut self, domain: &str) {
        let domain = parse_domain(domain);
        let image = parser::eval_with_variables(&self.equation, &domain);

        let domain_bounds = get_bounds(&domain);
        let image_bounds = get_bounds(&image);

        let points = domain.into_iter().zip(image.into_iter());        
        let points: Vec<_> = points.map(|p| {
            let d = map_range(domain_bounds, (0.0, self.term_w as f64 - 1.0), p.0) as usize;
            let i = map_range(image_bounds, (0.0, self.term_h as f64 - 1.0), p.1) as usize;

            (d, i)
        }).collect();

        points.iter().for_each(|p| {
            let i = p.0 + self.term_w * p.1;
            self.grid[i] = '*';
        });
    }

    pub fn plot(&mut self, domain: &str) {
        self.fill_points(domain);
        for (i, el) in self.grid.iter().enumerate() {
            if i % self.term_w == 0 {
                println!("{}", el);
                continue;
            }

            print!("{}", el);
        }
    }
}

