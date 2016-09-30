extern crate time;

pub struct KissRng {
    x: usize,
    y: usize,
    z: usize,
    c: usize,
}

impl KissRng {
    pub fn new() -> KissRng {
        let now = time::precise_time_ns() as usize;
        return KissRng {
            x: 123456789usize ^ now,
            y: 987654321usize ^ now,
            z: 43219876usize ^now,
            c: 6543217usize ^ now
        };
    }

    pub fn next(&mut self) -> usize {
        self.x = 314527869usize.wrapping_mul(self.x).wrapping_add(1234567);
        self.y ^= self.y.wrapping_shl(5);
        self.y ^= self.y.wrapping_shr(7);
        self.y ^= self.y.wrapping_shl(22);
        self.z = 4294584393usize.wrapping_mul(self.z).wrapping_add(self.c);
        self.c = self.z.wrapping_shr(32);
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
