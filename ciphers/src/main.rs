use std::env;
use std::collections::HashMap;

mod frequency_analysis;
pub mod dictionaries;
pub mod proj_utils;

mod bacon;
mod caesar;
mod morse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let analysis: Vec<(String, i32)> = frequency_analysis::main(args[1].clone());

    let (d_bacon, d_bacon_rev) = bacon::main(args[1].clone(), analysis.clone());
    let (d_morse, d_morse_rev) = morse::main(args[1].clone(), analysis.clone());

    let mut decoded: HashMap<String, String> = HashMap::from([
        ("Bacon Cipher".to_string(), d_bacon),
        ("Bacon Cipher (reverse)".to_string(), d_bacon_rev),
        ("Morse Code".to_string(), d_morse),
        ("Morse Code (reverse)".to_string(), d_morse_rev)
    ]);

    for i in 1..25 {
        let stri = "Caesar Cipher 🠜".to_owned() + &i.to_string();
        decoded.insert(stri, caesar::main(args[1].clone(), analysis.clone(), i));
    }

    dbg!(decoded);
    dbg!(analysis);
}