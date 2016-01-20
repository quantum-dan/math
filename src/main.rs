struct Fraction {
    num: i64, // Numerator
	den: i64, // Denominator
}

impl Copy for Fraction {}
impl Clone for Fraction {
    fn clone(&self) -> Fraction {
        Fraction::new(self.num, self.den)
    }
}

impl Fraction {
	fn new(n: i64, d: i64) -> Fraction {
		Fraction {
			num: n,
			den: d,
		}
	}
    fn to_float(&self) -> f64 {
        (self.num as f64)/(self.den as f64)
    }
    fn simplify(&self) -> Fraction {
        let val = gcd(self.num, self.den);
        if val > 1 {
            Fraction::new(self.num/val, self.den/val).simplify()
        } else {
            Fraction::new(self.num, self.den)
        }
    }
    fn add(x: Fraction, y: Fraction) -> Fraction {
        if x.den != y.den {
            Fraction::new(x.num * y.den + y.num * x.den, x.den * y.den)
        } else {
            Fraction::new(x.num + y.num, x.den)
        }
    }
    fn mult(x: Fraction, y: Fraction) -> Fraction {
        Fraction::new(x.num * y.num, x.den * y.den)
    }
    fn negative(&self) -> Fraction {
        Fraction::new(-self.num, self.den)
    }
    fn sub(x: Fraction, y: Fraction) -> Fraction {
        Fraction::add(x, y.negative())
    }
    fn inverse(&self) -> Fraction {
        Fraction::new(self.den, self.num)
    }
    fn div(x: Fraction, y: Fraction) -> Fraction {
        Fraction::mult(x, y.inverse())
    }
    fn comp(x: Fraction, y: Fraction) -> i8 {
        let val = x.to_float() - y.to_float();
        if val > 0.0 {
            1
        } else if val == 0.0 {
            0
        } else {
            -1
        }
    }
    fn zero() -> Fraction {
        Fraction::new(0, 1)
    }
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x > 0 && y > 0 {
        if x > y {
            x -= y;
        } else if y > x {
            y -= x;
        } else {
            return x
        }
    }
    return x
}

fn integral<F>(function: F, min: Fraction, max: Fraction, step: Fraction) -> Fraction
    where F: Fn(Fraction) -> Fraction {
        if Fraction::comp(min, max) == -1 {
            Fraction::add(Fraction::mult(function(min), step), (if Fraction::comp(Fraction::add(min, step), max) == -1 {
                integral(function, Fraction::add(min, step).simplify(), max, step)
            } else {
                Fraction::zero()
            })).simplify()
        } else {
            Fraction::zero()
        }
}

fn main() {
    println!("{}", integral(|x: Fraction| { Fraction::mult(x, x) }, Fraction::zero(), Fraction::new(10, 1), Fraction::new(1, 1)).to_float());
}