use std::collections::HashMap;

fn decode(string_: String, dot: String, dash: String) -> String {
    //let temp_replacement: String = "Ç".to_string();
    let ascii_morse_dict = HashMap::from([
        ("/" , 32),
        (".-...", 38),
        (".----.", 39),
        ("-.--.", 40),
        ("-.--.-", 41),
        ("--..--", 44),
        ("-....-", 45),
        (".-.-.-", 46),
        ("-..-.", 47),
        ("-----", 48),
        (".----", 49),
        ("..---", 50),
        ("...--", 51),
        ("....-", 52),
        (".....", 53),
        ("-....", 54),
        ("--...", 55),
        ("---..", 56),
        ("----.", 57),
        ("---...", 58),
        ("-.-.-.", 59),
        ("-...-", 61),
        ("..--..", 63),
        (".-", 65),
        ("-...", 66),
        ("-.-.", 67),
        ("-..", 68),
        (".", 69),
        ("--.", 71),
        ("....", 72),
        ("..", 73),
        (".---", 74),
        ("-.-", 75),
        (".-..", 76),
        ("--", 77),
        ("-.", 78),
        ("---", 79),
        (".--.", 80),
        ("--.-", 81),
        (".-.", 82),
        ("...", 83),
        ("-", 84),
        ("..-", 85),
        ("...-", 86),
        ("..-.", 70),
        (".--", 87),
        ("-..-", 88),
        ("-.--", 89),
        ("--..", 90),
        ("..--.-", 95),
        (".-", 97),
        ("-...", 98),
        ("-.-.", 99),
        ("-..", 100),
        (".", 101),
        ("..-.", 102),
        ("--.", 103),
        ("....", 104),
        ("..", 105),
        (".---", 106),
        ("-.-", 107),
        (".-..", 108),
        ("--", 109),
        ("-.", 110),
        ("---", 111),
        (".--.", 112),
        ("--.-", 113),
        (".-.", 114),
        ("...", 115),
        ("-", 116),
        ("..-", 117),
        ("...-", 118),
        (".--", 119),
        ("-..-", 120),
        ("-.--", 121),
        ("--..", 122)
    ]);

    let mut final_str: String = "".to_string();
    let replaced_string = string_.clone().replace(&dot, ".").replace(&dash, "-");

    for morse in replaced_string.split(" ") {
        let ascii = ascii_morse_dict.get(morse);
        if ascii != None {
            final_str = final_str + &(*ascii.unwrap() as u8 as char).to_string();
        }
    }

    return final_str;
}

pub fn main(string_: String, analysis: Vec<(String, i8)>) -> (String, String) {   
    let a1 = (analysis[0].0.parse::<i8>().unwrap() as u8 as char).to_string();
    let a2 = (analysis[1].0.parse::<i8>().unwrap() as u8 as char).to_string();

    let final_str: String = decode(string_.clone(), a1.clone(), a2.clone());
    let rev_str: String = decode(string_, a2, a1);

    return (final_str, rev_str)
}