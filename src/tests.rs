#[cfg(test)]
mod tests {
    use crate::Fancy;

    #[test]
    fn it_pow_mod() {
        let mut fancy = Fancy::new();
        let result = fancy.pow_mod(10, 2, 1000);
        assert_eq!(result, 100);
    }

    #[test]
    fn it_handles_single() {
        let mut fancy = Fancy::new();
        fancy.append(2);
        fancy.add_all(3);
        assert_eq!(fancy.get_index(0), 5);

        fancy.mult_all(10);
        assert_eq!(fancy.get_index(0), 50);
    }

    #[test]
    fn it_handles_basic() {
        let mut fancy = Fancy::new();

        fancy.append(2);   // fancy sequence: [2]
        fancy.add_all(3);   // fancy sequence: [2+3] -> [5]
        fancy.append(7);   // fancy sequence: [5, 7]
        fancy.mult_all(2);  // fancy sequence: [5*2, 7*2] -> [10, 14]

        assert_eq!(fancy.get_index(0), 10); // return 10

        fancy.add_all(3);   // fancy sequence: [10+3, 14+3] -> [13, 17]
        fancy.append(10);  // fancy sequence: [13, 17, 10]
        fancy.mult_all(2);  // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]

        assert_eq!(fancy.get_index(0), 26); // return 26
        assert_eq!(fancy.get_index(1), 34); // return 34
        assert_eq!(fancy.get_index(2), 20); // return 20
    }

    #[test]
    fn it_handles_big_number() {
        let mut fancy = Fancy::new();

        fancy.append(12);
        fancy.append(8);
        assert_eq!(fancy.get_index(1), 8);
        fancy.append(12);
        assert_eq!(fancy.get_index(0), 12);
        fancy.add_all(12);
        fancy.append(8);
        assert_eq!(fancy.get_index(2), 24);
        assert_eq!(fancy.get_index(2), 24);
        fancy.append(4);
        fancy.append(13);
        assert_eq!(fancy.get_index(4), 4);
        fancy.append(12);
        assert_eq!(fancy.get_index(6), 12);
        fancy.append(11);
        assert_eq!(fancy.get_index(1), 20);
        fancy.append(10);
        assert_eq!(fancy.get_index(2), 24);
        fancy.mult_all(3);
        fancy.add_all(1);
        assert_eq!(fancy.get_index(6), 37);
        fancy.append(14);
        fancy.add_all(5);
        assert_eq!(fancy.get_index(6), 42);
        fancy.mult_all(12);
        assert_eq!(fancy.get_index(3), 360);
        fancy.mult_all(12);
        fancy.add_all(15);
        fancy.add_all(6);
        fancy.append(7);
        fancy.mult_all(8);
        fancy.append(13);
        fancy.append(15);
        fancy.append(15);
        fancy.mult_all(10);
        assert_eq!(fancy.get_index(9), 220560);
        fancy.mult_all(12);
        fancy.mult_all(12);
        fancy.mult_all(9);
        assert_eq!(fancy.get_index(9), 285845760);
        fancy.add_all(9);
        fancy.append(9);
        fancy.mult_all(4);
        fancy.add_all(8);
        fancy.add_all(11);
        fancy.mult_all(15);
        fancy.add_all(9);
        fancy.add_all(1);
        fancy.append(4);
        fancy.append(10);
        assert_eq!(fancy.get_index(9), 150746316);
    }
}
