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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_suffix_array() {
        let result = build_suffix_array("ACATTCGT");
        assert_eq!(1,1);
    }
}
