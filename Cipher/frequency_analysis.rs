use std::collections::HashMap;

pub fn main(string_: String) -> HashMap<String, i8> {
    // create a table for every ascii character that is 1 byte long
    let mut analysis: HashMap<String, i8> = HashMap::new();

    for character in string_.chars() {
        let non_ascii = character.to_string();
        if analysis.get(&non_ascii) == None {
            analysis.insert(non_ascii, 1);
        }
        else {
            let count = analysis.get(&non_ascii).unwrap();
            analysis.insert(non_ascii, count + 1);
        }   
    }
    return analysis;
}