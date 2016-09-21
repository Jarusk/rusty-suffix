pub fn build_suffix_array(s: &str) -> Vec<usize> {
    let n = s.len() + 1;
    let mut stype = build_stype(s,n);
    let mut s1 = vec![0usize;n];
    let mut p1 = vec![0usize;n];

    println!("{:?}",&stype);
    Vec::new()
}

/// Classify the string characters as either S or L.
/// 'True' signifies the corresponding character is S type.
pub fn build_stype(s: &str, size: usize) -> Vec<bool> {
    //Handle the termination char
    let mut cur_pos = size-1;
    let mut item_1 = '$';
    let mut item_1_stype = true;
    let mut stype = vec![true;size];
    stype[cur_pos] = true;
    
    // Classify the type of each element in the list
    for item in s.chars().rev() {
        cur_pos -= 1;
        if item.lt(&item_1) {
            item_1_stype = true;
            stype[cur_pos] = true;
        }else if item.gt(&item_1) {
            item_1_stype = false;
            stype[cur_pos] = false;
        }else{
            stype[cur_pos] = item_1_stype;
        }

        item_1 = item;
    }

    return stype;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_suffix_array() {
        let result = build_suffix_array("ACATTCGT");
        assert_eq!(1,1);
    }

    #[test]
    fn test_build_stype() {
        let expected = vec![true, false, true, false, false, true, true, false, true];
        assert_eq!(expected, build_stype("ACATTCGT",9));
    }
}
