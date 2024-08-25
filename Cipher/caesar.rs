use std::cmp::max;

fn decode(string_: String, shift: i8) -> String {
    let mut final_str = "".to_string();

    for character in string_.to_lowercase().chars() {
        let kchar = character as u8;

        if kchar >= 97 && kchar <= 122 {
            let new_character = max((character as i8 - shift) % 122, 97);
            final_str = final_str + &(new_character as u8 as char).to_string();
        } else {
            final_str = final_str + &(kchar as char).to_string();
        }
    }

    return final_str
}

pub fn main(string_: String, _analysis: Vec<(String, i32)>) -> String {
    return decode(string_, 3); // example shift until every other cipher is done
}