mod lexer;
mod parser;
mod plot;

use plot::Plot;

fn main() {
    let mut p = Plot::new("-1 * log2($)");
    p.plot("1.0:0.1:5.0");
    p.plot("10.0:1:25.0");
}
