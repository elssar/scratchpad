use std::{
    fmt,
    ops,
};

use num::{
    integer::gcd,
};

fn reduce(num: i64, den: i64) -> (i64, i64) {
    let div = gcd(num, den);

    return (num / div, den / div);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
    sign: i8
}

impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Fraction {
       let mut sign: i8 = 1;

       if numerator.abs() != numerator {
           sign *= -1;
       }

       if denominator.abs() != denominator {
           sign *= -1;
       }

       let reduced = reduce(numerator.abs(), denominator.abs());

       let fraction = Fraction {
           numerator: reduced.0,
           denominator: reduced.1,
           sign: sign
       };

       fraction
    }

    pub fn reduce(&mut self) {
        let div = gcd(self.numerator, self.denominator);

        self.numerator /= div;
        self.denominator /= div;
    }
}

impl Default for Fraction {
    fn default() -> Fraction {
        Fraction {
            numerator: 1,
            denominator: 1,
            sign: 1
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self {
        let d = self.denominator * rhs.denominator;
        let n = self.numerator * rhs.denominator + rhs.numerator * self.denominator;

        Fraction::new(d, n)
    }
}

#[test]
fn test_fraction_reduce() {
    let mut f1 = Fraction::new(2, 4);

    f1.reduce();
    assert_eq!(f1.numerator, 1);
    assert_eq!(f1.denominator, 2);
}

#[test]
fn test_fraction_add() {
    let f1 = Fraction::new(1, 3);
    let f2 = Fraction::new(2, 3);
    let f3 = f1 + f2;

    assert_eq!(f3.numerator, 1);
    assert_eq!(f3.denominator, 1);
}
