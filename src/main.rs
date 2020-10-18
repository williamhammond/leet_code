mod tests;

use std::cell::RefCell;

const MOD: i32 = 1000000007;

fn main() {

}


struct Fancy {
    seq: RefCell<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {

    fn new() -> Self {
        return Fancy{
            seq: RefCell::new(Vec::new()),
        };
    }

    fn append(&self, val: i32) {
        self.seq.borrow_mut().push(val);
    }

    fn add_all(&self, inc: i32) {
        for i in self.seq.borrow_mut().iter_mut() {
            *i += inc;
        }
    }

    fn mult_all(&self, m: i32) {
        for i in self.seq.borrow_mut().iter_mut() {
            *i *= m;
        }
    }

    fn get_index(&self, idx: i32) -> i32 {
        if idx as usize >= self.seq.borrow_mut().len() {
            return -1;
        }
        return self.seq.borrow_mut()[idx as usize] % MOD;
    }
}
