use std::env;

mod frequency_analysis;
pub mod dictionaries;
pub mod utils;

mod bacon;
mod caesar;
mod morse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let analysis: Vec<(String, i32)> = frequency_analysis::main(args[1].clone());

    dbg!(morse::main(args[1].clone(), analysis.clone()));
    dbg!(bacon::main(args[1].clone(), analysis.clone()));
    dbg!(caesar::main(args[1].clone(), analysis.clone()));

    dbg!(analysis);
}