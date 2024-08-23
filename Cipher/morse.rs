use std::collections::HashMap;
use dictionaries;
use utils::get_from_analysis;

fn decode(string_: String, dot: String, dash: String, space: String) -> String {
    //let temp_replacement: String = "Ç".to_string();
    let ascii_morse_dict: HashMap<&str, i8> = dictionaries::main("morse");


    let mut final_str: String = "".to_string();
    let replaced_string = string_.clone().replace(&dot, ".").replace(&dash, "-").replace(&space, " ");
    println!("{}" ,replaced_string);
    for morse in replaced_string.split(" ") {
        let ascii = ascii_morse_dict.get(morse);
        if ascii != None {
            final_str = final_str + &(*ascii.unwrap() as u8 as char).to_string();
        }
    }

    return final_str;
}

pub fn main(string_: String, analysis: Vec<(String, i32)>) -> (String, String) {   
    let a1 = get_from_analysis(analysis.clone(), 0); // dot
    let a2 = get_from_analysis(analysis.clone(), 1); // dash
    let a3 = get_from_analysis(analysis, 2); // "/"

    let final_str: String = decode(string_.clone(), a1.clone(), a2.clone(), a3.clone());
    let rev_str: String = decode(string_, a2, a1, a3);

    return (final_str, rev_str)
}