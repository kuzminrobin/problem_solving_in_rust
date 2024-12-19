fn main() {
    {
        // Find the n-th largest element in a slice:
        fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
            let mut refs: Vec<&T> = elems.iter().collect();
            refs.sort();
            refs[n].clone()
        }

        assert_eq!(find_nth(&[0, 1, 2, 3, 4, 5], 3), 3);
        assert_eq!(find_nth(&[6, 5, 4, 3, 2, 1, 0], 2), 2);
    }
}
