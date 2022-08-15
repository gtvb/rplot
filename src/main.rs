mod lexer;
mod parser;
mod plot;

use plot::Plot;

fn main() {
    let mut p = Plot::new("sin($ * cos($))", "-6.28:0.01:6.28");
    p.plot()
}
