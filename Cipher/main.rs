use std::env;

mod frequency_analysis;

mod morse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let analysis: Vec<(String, i8)> = frequency_analysis::main(args[1].clone());
    dbg!(morse::main(args[1].clone(), analysis.clone()));
    dbg!(analysis);
}