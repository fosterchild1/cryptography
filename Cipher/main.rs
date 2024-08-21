use std::env;

mod frequency_analysis;

fn main() {
    let args: Vec<String> = env::args().collect();
    let analysis: Vec<i8> = frequency_analysis::main(args[1].clone());
    dbg!(analysis);
}