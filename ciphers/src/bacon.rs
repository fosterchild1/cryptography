use std::collections::HashMap;
use std::str;
use dictionaries;
use utils::get_from_analysis;

fn decode(string_: String, ah: String, bah: String) -> String {
    let temp_replacement: String = "Ç".to_string();
    let bacon_dict: HashMap<&str, i8> = dictionaries::main("bacon");

    let mut final_str: String = "".to_string();
    let replaced_string = string_.clone().replace(&bah, &temp_replacement).replace(&ah, "A").replace(&temp_replacement, "B").replace(" ", "");

    // wtf
    let subs = replaced_string.as_bytes().chunks(5).map(|buf| unsafe { str::from_utf8_unchecked(buf) }).collect::<Vec<&str>>();

    for bacon in subs {
        let ascii = bacon_dict.get(bacon);
        if ascii != None {
            final_str = final_str + &(*ascii.unwrap() as u8 as char).to_string();
        }
    }

    return final_str;
}

pub fn main(string_: String, analysis: Vec<(String, i32)>) -> (String, String) {   
    let a1 = get_from_analysis(analysis.clone(), 0); // A
    let a2 = get_from_analysis(analysis.clone(), 1); // B

    let final_str: String = decode(string_.clone(), a1.clone(), a2.clone());
    let rev_str: String = decode(string_, a2, a1);

    return (final_str, rev_str)
}