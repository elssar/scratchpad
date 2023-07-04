use std::{
    fmt,
    ops,
};

use num::{
    Signed,
    integer::gcd,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction<T: Signed> {
    numerator: T,
    denominator: T,
    sign: i8
}

impl<T: Signed> Fraction<T> {
    pub fn new(numerator: T, denominator: T) -> Fraction<T> {
       let mut sign: i8 = 1;

       if numerator.abs() != numerator {
           sign *= -1;
       }

       if denominator.abs() != denominator {
           sign *= -1;
       }

       let fraction = Fraction {
           numerator: numerator.abs(),
           denominator: denominator.abs(),
           sign: sign
       };

       fraction.reduce();

       fraction
    }

    pub fn reduce(&mut self) {
        let div = gcd(self.numerator, self.denominator);

        self.numerator /= div;
        self.denominator /= div;
    }
}

impl<T: Signed> Default for Fraction<T> {
    fn default() -> Fraction<T> {
        Fraction {
            numerator: 1,
            denominator: 1,
            sign: 1
        }
    }
}

impl<T: Signed> fmt::Display for Fraction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl<T: Signed> ops::Add for Fraction<T> {
    type Output = Fraction<T>;

    fn add(self, rhs: Self) -> Self {
        let d = self.denominator * rhs.denominator;
        let n = self.numerator * rhs.denominator + rhs.numerator * self.denominator;

        Fraction::new(d, n)
    }
}

#[test]
fn test_fraction_reduce() {
    let mut f1 = Fraction::new("2/4");

    f1.reduce();
    assert_eq!(f1.numerator, 1);
    assert_eq!(f1.denominator, 2);
}

#[test]
fn test_fraction_add() {
    let f1 = Fraction::new("1/2");
    let f2 = Fraction::new("2/3");
    let f3 = f1 + f2;

    assert_eq!(f3.numerator, 7);
    assert_eq!(f3.denominator, 6);
}
