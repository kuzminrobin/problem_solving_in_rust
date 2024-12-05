fn main() {
    {
        fn vec_reverse_inplace<T>(v: &mut Vec<T>) {
            let len = v.len();
            for i in 0..len / 2 {
                let p1 = &mut v[i] as *mut T;
                let p2 = &mut v[len - 1 - i] as *mut T;
                unsafe {
                    std::ptr::swap_nonoverlapping(p1, p2, 1);
                }
            }
        }

        let mut v: Vec<String> = vec![];
        vec_reverse_inplace(&mut v);
        assert_eq!(v, Vec::<String>::new());

        let mut v = vec!["A".to_string(), "B".to_string()];
        vec_reverse_inplace(&mut v);
        assert_eq!(v, vec!["B".to_string(), "A".to_string()]);

        let mut v = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];
        vec_reverse_inplace(&mut v);
        assert_eq!(
            v,
            vec![
                "E".to_string(),
                "D".to_string(),
                "C".to_string(),
                "B".to_string(),
                "A".to_string()
            ]
        );
    }
}
