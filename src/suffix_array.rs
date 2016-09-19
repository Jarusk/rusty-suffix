pub fn build_suffix_array(s: &str) -> Vec<usize> {
    let n = s.len() + 1;
    let mut t = vec![true;n];
    let mut s1 = vec![0usize;n];
    let mut p1 = vec![0usize;n];

    // Classify the type of each element in the list
    for x in (0..n).rev() {
        println!("{:?}",&x);
    }
    Vec::new()
}
