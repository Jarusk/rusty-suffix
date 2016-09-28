extern crate time;

use kissrng::KissRng;

pub fn rand_dna(n: usize) -> String {
    let mut sequence = String::new();
    let mut rng = KissRng::new();

    for _ in 0..n {
        sequence.push(
            match rng.next() % 4 {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                _ => 'T'
            }
        );
    }

    return sequence;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_dna() {
        let seq_len = 1000;
        let seq = rand_dna(seq_len);
        println!("{:?}",&seq);
        assert_eq!(seq_len, seq.len());
    }
}
