use std::env;
use std::collections::HashMap;

mod frequency_analysis;

fn main() {
    let args: Vec<String> = env::args().collect();
    let analysis: HashMap<String, i8> = frequency_analysis::main(args[1].clone());
    dbg!(analysis);
}