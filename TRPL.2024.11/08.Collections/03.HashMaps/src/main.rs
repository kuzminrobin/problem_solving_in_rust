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
            let mut hm: HashMap<i32, usize> = HashMap::new(); // maps the element value to count.
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

    // Using a hash map and vectors, create a text interface to allow a user
    // to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company by department,
    // sorted alphabetically.
    {
        use std::collections::HashMap;
        use std::io::Write;

        let mut db = HashMap::new();
        loop {
            print!("> ");
            std::io::stdout().flush().expect("Failed to flush output");

            let mut command = String::new();
            std::io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");

            let mut command_iter = command.split_whitespace();
            let cmd_id = command_iter.next();
            if cmd_id.is_none() {
                continue;
            }
            match cmd_id.unwrap() {
                "q" => break,
                "+" => {
                    let mut name = String::new();
                    let mut dept = String::new();
                    if let Some(name_entry) = command_iter.next() {
                        name = String::from(name_entry);
                    }
                    if let Some(dept_entry) = command_iter.next() {
                        dept = String::from(dept_entry);
                    }
                    if name.len() != 0 && dept.len() != 0 {
                        let dept_staff = db.entry(dept).or_insert(Vec::new());
                        dept_staff.push(name);
                    } else {
                        println!("The name or department is missing");
                    }
                }
                "L" => {
                    if let Some(dept_entry) = command_iter.next() {
                        let dpt_staff = db.get_mut(dept_entry);
                        if let Some(dpt_staff) = dpt_staff {
                            dpt_staff.sort();
                            println!("\"{dept_entry}\": {dpt_staff:?}");
                        } else {
                            println!("Unknown department \"{dept_entry}\"");
                        }
                    } else {
                        println!("db: {db:?}");
                    }
                }
                unknown => println!("Unknown command \"{}\"", unknown),
            }
        }
        // > L
        // db: {}
        // > + N1 D1
        // > L
        // db: {"D1": ["N1"]}
        // > + N2 D2
        // > L
        // db: {"D1": ["N1"], "D2": ["N2"]}
        // > L N0 D1
        // Unknown department "N0"
        // > + N0 D1
        // > L
        // db: {"D1": ["N1", "N0"], "D2": ["N2"]}
        // > + N3 D1
        // > L
        // db: {"D1": ["N1", "N0", "N3"], "D2": ["N2"]}
        // > L D1
        // "D1": ["N0", "N1", "N3"]
        // > L
        // db: {"D1": ["N0", "N1", "N3"], "D2": ["N2"]}
        // > q        
    }
}
