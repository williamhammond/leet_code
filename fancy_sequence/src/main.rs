mod tests;

use std::cell::RefCell;

const MOD: usize = 1000000007;

fn main() {

}

#[derive(Debug)]
struct Fancy {
    seq: RefCell<Vec<(usize, usize, usize)>>,
    inc: usize,
    mult: usize,
}

impl Fancy {

    fn new() -> Self {
        return Fancy{
            seq: RefCell::new(Vec::new()),
            inc: 0,
            mult: 1,
        };
    }

    fn append(&self, val: i32) {
        self.seq.borrow_mut().push((val as usize, self.inc, self.mult));
    }

    fn add_all(&mut self, inc: i32) {
        self.inc += inc as usize;
    }

    fn mult_all(&mut self, m: i32) {
        self.mult = (self.mult * m as usize) % MOD;
        self.inc = (self.inc * m as usize) % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        if idx as usize >= self.seq.borrow_mut().len() {
            return -1;
        }
        let (v, v_inc, v_mult) = self.seq.borrow()[idx as usize];
        let ratio = (self.mult * self.pow_mod(v_mult, MOD - 2, MOD)) % MOD;

        return (((ratio * (MOD + v - v_inc)) + self.inc) % MOD) as i32;
    }

    fn pow_mod(&self, base: usize, exp: usize, modulo: usize) -> usize {
        // Sanity check:  Verify that the exponent is not negative:
        assert!(exp >= 0);

        if exp == 0 {
            return 1;
        }

        // Now do the modular exponentiation algorithm:
        let mut base = base % modulo;
        let mut exp = exp;

        let mut result = 1;
        while exp > 0 {
            if (exp & 1) != 0 {
                result = (self.mod_mult(result, base, MOD)) % modulo;
            }
            base = self.mod_mult(base, base, MOD) % modulo;
            exp = exp >> 1;
        }

        return result;
    }

    fn mod_mult(&self, a: usize , b: usize, m: usize) -> usize {
        let mut a= a;
        let mut b = b;

        let mut result = 0;
        while a != 0 {
            if (a & 1) != 0 {
                result = (result + b) % m;
            }
            a >>= 1;
            b = (b << 1) % m;
        }

        return result;
    }
}
