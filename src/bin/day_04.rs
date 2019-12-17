fn main() {
    let valid_1 = (265_275..=781_584)
        .map(Digits::new)
        .filter(Digits::is_valid_password_1)
        .collect::<Vec<Digits>>();

    let part_1 = valid_1.len();
    let part_2 = valid_1
        .iter()
        .filter(|digits| digits.has_double_strict())
        .count();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

struct Digits([usize; 6]);

impl Digits {
    fn new(x: usize) -> Digits {
        let digits = [
            ((x / 100_000) % 10),
            ((x / 10_000) % 10),
            ((x / 1_000) % 10),
            ((x / 100) % 10),
            ((x / 10) % 10),
            (x % 10),
        ];

        Digits(digits)
    }

    fn has_double(&self) -> bool {
        for i in 1..6 {
            if self.0[i] == self.0[i - 1] {
                return true;
            }
        }
        false
    }

    fn has_double_strict(&self) -> bool {
        let mut count = 0;

        for i in 1..6 {
            if self.0[i] == self.0[i - 1] {
                count += 1;
            } else {
                if count == 1 {
                    return true;
                }

                count = 0;
            }
        }

        count == 1
    }

    fn is_increasing(&self) -> bool {
        for i in 1..6 {
            if self.0[i] < self.0[i - 1] {
                return false;
            }
        }
        true
    }

    fn is_valid_password_1(&self) -> bool {
        self.is_increasing() && self.has_double()
    }
}

#[cfg(test)]
mod day_04 {
    use super::*;
    #[test]
    fn test_1_valid() {
        assert!(Digits::new(111_111).is_valid_password_1());
    }

    #[test]
    fn test_1_decreasing() {
        assert!(!Digits::new(223_450).is_valid_password_1());
    }

    #[test]
    fn test_1_missing_double() {
        assert!(!Digits::new(123_789).is_valid_password_1());
    }

    #[test]
    fn test_2_valid() {
        let uut = Digits::new(112_233);
        assert!(uut.is_valid_password_1());
        assert!(uut.has_double_strict());
    }

    #[test]
    fn test_2_invalid_double() {
        let uut = Digits::new(123_444);
        assert!(uut.is_valid_password_1());
        assert!(!uut.has_double_strict());
    }

    #[test]
    fn test_2_valid_double() {
        let uut = Digits::new(111_122);
        assert!(uut.is_valid_password_1());
        assert!(uut.has_double_strict());
    }
}
