fn main() {
    // https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary
    // Given a list of integers, use a vector and return
    // the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; a hash map will be helpful here)
    // of the list.
    {
        fn median(s: &[i32]) -> Option<i32> {
            let len: usize = s.len();
            if len == 0 {
                return None;
            }
            let mut v: Vec<i32> = s.to_vec();
            v.sort();
            Some(v[len / 2])
        }
        fn mode(s: &[i32]) -> Option<i32> {
            if s.len() == 0 {
                return None;
            }
            use std::collections::HashMap;
            let mut hm: HashMap<i32, usize> = HashMap::new();   // maps the element value to count.
            for el in s {
                let count: &mut usize = hm.entry(*el).or_insert(0_usize);
                *count += 1;
            }
            let mut most_freq_el: i32 = s[0];
            let mut highest_count: usize = *hm.get(&most_freq_el).unwrap();
            for (k_el, v_count) in hm {
                if v_count > highest_count {
                    highest_count = v_count;
                    most_freq_el = k_el;
                }
            }
            Some(most_freq_el)
        }
        let v = [-4, 9, 7, -6, 3, -4, 7, 9, 3, 7, 0, 1];
        let mut vec = v.to_vec();
        vec.sort();
        println!("sorted: {:?}", vec);
        println!("median: {:?}", median(&v));
        println!("most_often: {:?}", mode(&v));
        // sorted: [-6, -4, -4, 0, 1, 3, 3, 7, 7, 7, 9, 9]
        // median: Some(3)
        // most_often: Some(7)
    }

    // Convert strings to pig latin. 
    // The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
    // Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
    // Keep in mind the details about UTF-8 encoding!    
    {
        fn is_vowel(ch: char) -> bool {
            "aeiouy".contains(ch)
        }
        fn to_pig_latin(text: &str) -> String {
            let mut translated = String::new();
            for word in text.split_whitespace() {
                let start_char = word.chars().nth(0).unwrap();
                if is_vowel(start_char) {
                    translated.push_str(word);
                    translated.push_str("-hay");
                } else {
                    let (head_first_char, tail) = word.split_at(1);
                    let translated_word = String::from(tail) + "-" + head_first_char + "ay";
                    translated.push_str(&translated_word);
                }
                translated.push_str(" ");
            }
            translated
        }
        let text = "run spot arch go you i hi";
        println!("original: {text}\ntranslated: {}", to_pig_latin(text));
        // original: run spot arch go you i hi
        // translated: un-ray pot-say arch-hay o-gay you-hay i-hay i-hay        
    }
}
