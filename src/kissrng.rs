pub struct KissRng {
    x: usize,
    y: usize,
    z: usize,
    c: usize,
}

impl KissRng {
    pub fn new() -> KissRng {
        return KissRng {
            x: 123456789,
            y: 987654321,
            z: 43219876,
            c: 6543217,
        };
    }

    pub fn next(&mut self) -> usize {
        self.x = 314527869usize.wrapping_mul(self.x).wrapping_add(1234567);
        self.y ^= self.y.rotate_left(5);
        self.y ^= self.y.rotate_right(7);
        self.y ^= self.y.rotate_left(22);
        self.z = 4294584393usize.wrapping_mul(self.z).wrapping_add(self.c);
        self.c = self.z.rotate_right(32);
        return self.x.wrapping_add(self.y).wrapping_add(self.z);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kissrng_test_for_overflow() {
        let mut rng = KissRng::new();
        for _ in 0..1000000 {
            rng.next();
        }
    }
}