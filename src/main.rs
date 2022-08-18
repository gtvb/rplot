mod lexer;
mod parser;
mod plot;

use clap::{Arg, Command};
use plot::Plot;

fn main() {
    let matches = Command::new("rplot")
        .arg(Arg::new("equation").allow_hyphen_values(true))
        .arg(Arg::new("domain").allow_hyphen_values(true))
        .get_matches();

    let equation = matches.get_one::<String>("equation").expect("error: missing equation argument");
    let domain = matches.get_one::<String>("domain").expect("error: missing domain argument");

    let mut plotter = Plot::new(&equation, &domain);
    plotter.plot();
}
