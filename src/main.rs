// use std::env;
// use clap::{arg, App};
use clap::Parser;

// fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // clapを使ってコマンドライン引数を受け取る
    /*let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("Your name")
        .about("Super awesome sample RPN calculator")
        .arg(arg!([FILE] "Formulas written in RPN").required(false))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity").required(false))
        .get_matches();

    match matches.value_of("FILE") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);*/

// }

#[derive(Parser, Debug)]
#[clap(
    name="My RPN program",
    version="1.0.0",
    author="Your name",
    about="Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name="FILE")]
    formula_file: Option<String>,
}
fn main() {
    let opts = Opts::parse();

    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}
