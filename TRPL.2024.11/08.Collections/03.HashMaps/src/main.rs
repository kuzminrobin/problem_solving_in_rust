
fn main() {
    // https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#summary
    // Given a list of integers, use a vector and return 
    // the median (when sorted, the value in the middle position) 
    // and mode (the value that occurs most often; a hash map will be helpful here) 
    // of the list.
    {
        fn median(s: &[i32]) -> Option<i32> {
            let mut v = s.to_vec();
            v.sort();
            let len = v.len();
            if len == 0 {
                return None;
            }
            Some(v[len / 2])
        }
        fn mode(s: &[i32]) -> Option<i32> {
            if s.len() == 0 {
                return None
            }
            use std::collections::HashMap;
            let mut hm = HashMap::new();
            for el in s {
                let count = hm.entry(*el).or_insert(0_usize);   //.insert(el)
                *count += 1;
            }
            let mut most_freq_el = s[0];
            let mut highest_count = *hm.get(&most_freq_el).unwrap();
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
}
