#[cfg(test)]
mod tests {
    use crate::Fancy;
    #[test]
    fn it_handles_basic() {
        let fancy = Fancy::new();

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
}
