extern crate rand;
extern crate time;

use self::rand::{Rng, SeedableRng, XorShiftRng};

pub fn rand_dna(n: usize) -> String {
    let seed_val = time::precise_time_ns() as u32;
    let seed = [seed_val, seed_val + 2239901u32, seed_val + 12212212u32, seed_val + 3265347u32];
    let mut sequence = String::new();
    let mut rng = XorShiftRng::from_seed(seed);

    for _ in 0..n {
        sequence.push(
            match rng.next_u32() % 4 {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                _ => 'T'
            }
        );
    }

    println!("{:?}",&sequence);
    return sequence;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_dna() {
        let seq_len = 1000000;
        let seq = rand_dna(seq_len);
        assert_eq!(seq_len, seq.len());
    }
}
