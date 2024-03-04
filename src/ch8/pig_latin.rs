fn convert_str(word: String) -> String {
    let first_c = word.chars().next().unwrap().to_ascii_lowercase();
    let first_is_vocal = {
        match first_c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    };
    let mut i = 0;
    let mut res = String::from("");
    for c in word.chars() {
        if i == 0 && !first_is_vocal {
            i += 1;
            continue;
        }
        res.push(c);
        i += 1;
    }
    if first_is_vocal {
        format!("{res}-hay")
    }
    else {
        format!("{res}-{first_c}ay")
    }
}

pub fn pig_latin(word: String) {
    let word = convert_str(word);
    println!("{word}");
}

#[cfg(test)]
mod tests {
    use crate::ch8::pig_latin::convert_str;
    #[test]
    fn pig_latin_works() {
        assert_eq!(convert_str("apple".to_string()), "apple-hay");
        assert_eq!(convert_str("hello".to_string()), "ello-hay");
        assert_eq!(convert_str("fantastic".to_string()), "antastic-fay");
    }
}
