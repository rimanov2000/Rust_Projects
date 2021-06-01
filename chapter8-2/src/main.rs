// Convert strings to pig latin. The first consonant of each word is moved
// to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
// Keep in mind the details about UTF-8 encoding!

// by Rufat Imanov

use std::env;

fn pigify(my_word: &mut str) -> String {
    let my_string : Vec<char> = my_word.chars().collect();
    match my_string[0] {
        'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u'  => {
            let mut mod_string = String::from(my_word);
            mod_string.push_str("-hay");
            return mod_string;
        },
        'B' | 'b' | 'C' | 'c' | 'D' | 'd' | 'F' | 'f' | 'G' | 'g' | 'H' | 'h' |
        'J' | 'j' | 'K' | 'k' | 'L' | 'l' | 'M' | 'm' | 'N' | 'n' | 'P' | 'p' |
        'Q' | 'q' | 'R' | 'r' | 'S' | 's' | 'T' | 't' | 'V' | 'v' | 'W' | 'w' |
        'X' | 'x' | 'Y' | 'y' | 'Z' | 'z'  => {
            let mut mod_string = String::from(&my_word[1..]);
            mod_string.push_str(&("-".to_string() + my_string[0].to_string().as_str() + "ay")[..]);
            return mod_string;
        },
        _ => panic!("Non ASCII character is given!"),
    }
}

fn main() {
    let mut args = env::args().skip(1).next().expect("should have one argument");
    println!("The Pig Latin word is {}",pigify(args.as_mut_str()));
}
