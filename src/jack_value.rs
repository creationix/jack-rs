

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;
use std::ops::Neg;
use std::fmt;

#[derive (Debug, Clone, Copy)]
pub enum JackValue {
    Nil,
    Boolean(bool),
    Integer(i32),
    Rational(i32,i32),
    String(u32),
    Buffer(u32),
    List(u32),
    Map(u32),
    TypeError(&'static str),
}

impl fmt::Display for JackValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &JackValue::Integer(a) => write!(f, "{}", a),
            &JackValue::Rational(a, b) => write!(f, "{}/{}", a, b),
            &JackValue::Nil => write!(f, "nil"),
            &JackValue::TypeError(msg) => write!(f, "<TypeError: {}>", msg),
            _ => write!(f, "TODO: implement display")
        }
    }
}

impl JackValue {
    pub fn new_rational(mut a: i32, mut b: i32) -> JackValue {
        if b != 0 {
            let g = gcd(a, b);
            a /= g;
            b /= g;
            if b == 1 { return JackValue::Integer(a); }
            if b < 0 {
                b = -b;
                a = -a;
            }
        }
        JackValue::Rational(a, b)
    }

    pub fn add(lhs: JackValue, rhs: JackValue) -> JackValue {
        match (lhs, rhs) {
            (JackValue::Boolean(a), JackValue::Boolean(b)) => JackValue::Boolean(a || b),
            (JackValue::Integer(a), JackValue::Integer(b)) => JackValue::Integer(a + b),
            (JackValue::Integer(a), JackValue::Rational(c, d)) => rational_add(a, 1, c, d),
            (JackValue::Rational(a, b), JackValue::Rational(c, d)) => rational_add(a, b, c, d),
            (JackValue::Rational(a, b), JackValue::Integer(c)) =>rational_add(a, b, c, 1),
            _ => JackValue::TypeError("Add requires two numbers or two booleans"),
        }
    }

    pub fn sub(lhs: JackValue, rhs: JackValue) -> JackValue {
        match (lhs, rhs) {
            (JackValue::Integer(a), JackValue::Integer(b)) => JackValue::Integer(a - b),
            (JackValue::Integer(a), JackValue::Rational(c, d)) => rational_add(a, 1, -c, d),
            (JackValue::Rational(a, b), JackValue::Rational(c, d)) => rational_add(a, b, -c, d),
            (JackValue::Rational(a, b), JackValue::Integer(c)) => rational_add(a, b, -c, 1),
            _ => JackValue::TypeError("Sub requires two numbers"),
        }
    }

    pub fn mul(lhs: JackValue, rhs: JackValue) -> JackValue {
        match (lhs, rhs) {
            (JackValue::Boolean(a), JackValue::Boolean(b)) => JackValue::Boolean(a & b),
            (JackValue::Integer(a), JackValue::Integer(b)) => JackValue::Integer(a * b),
            (JackValue::Integer(a), JackValue::Rational(c, d)) => rational_mul(a, 1, c, d),
            (JackValue::Rational(a, b), JackValue::Rational(c, d)) => rational_mul(a, b, c, d),
            (JackValue::Rational(a, b), JackValue::Integer(c)) => rational_mul(a, b, c, 1),
            _ => JackValue::TypeError("Mul requires two numbers or two booleans"),
        }
    }

    pub fn div(lhs: JackValue, rhs: JackValue) -> JackValue {
        match (lhs, rhs) {
            (JackValue::Integer(a), JackValue::Integer(b)) => JackValue::new_rational(a, b),
            (JackValue::Integer(a), JackValue::Rational(c, d)) => rational_mul(a, 1, d, c),
            (JackValue::Rational(a, b), JackValue::Rational(c, d)) => rational_mul(a, b, d, c),
            (JackValue::Rational(a, b), JackValue::Integer(c)) => rational_mul(a, b, 1, c),
            _ => JackValue::TypeError("Div requires two numbers"),
        }
    }

    pub fn idiv(lhs: JackValue, rhs: JackValue) -> JackValue {
        match (lhs, rhs) {
            (JackValue::Integer(a), JackValue::Integer(b)) => JackValue::Integer(a / b),
            (JackValue::Integer(a), JackValue::Rational(c, d)) => rational_idiv(a, 1, c, d),
            (JackValue::Rational(a, b), JackValue::Rational(c, d)) => rational_idiv(a, b, c, d),
            (JackValue::Rational(a, b), JackValue::Integer(c)) => rational_idiv(a, b, c, 1),
            _ => JackValue::TypeError("Rem requires two integers"),
        }
    }

    pub fn rem(lhs: JackValue, rhs: JackValue) -> JackValue {
        match (lhs, rhs) {
            (JackValue::Integer(a), JackValue::Integer(b)) => JackValue::Integer(a % b),
            (JackValue::Integer(a), JackValue::Rational(c, d)) => rational_rem(a, 1, c, d),
            (JackValue::Rational(a, b), JackValue::Rational(c, d)) => rational_rem(a, b, c, d),
            (JackValue::Rational(a, b), JackValue::Integer(c)) => rational_rem(a, b, c, 1),
            _ => JackValue::TypeError("Rem requires two integers"),
        }
    }

    pub fn neg(lhs: JackValue) -> JackValue {
        match lhs {
            JackValue::Integer(a) => JackValue::Integer(-a),
            JackValue::Rational(a, b) => JackValue::Rational(-a, b),
            _ => JackValue::TypeError("Neg requires a number"),
        }
    }
}

impl Add for JackValue {
    type Output = JackValue;
    fn add(self, rhs: JackValue) -> JackValue {
        JackValue::add(self, rhs)
    }
}

impl Sub for JackValue {
    type Output = JackValue;
    fn sub(self, rhs: JackValue) -> JackValue {
        JackValue::sub(self, rhs)
    }
}

impl Mul for JackValue {
    type Output = JackValue;
    fn mul(self, rhs: JackValue) -> JackValue {
        JackValue::mul(self, rhs)
    }
}

impl Div for JackValue {
    type Output = JackValue;
    fn div(self, rhs: JackValue) -> JackValue {
        JackValue::div(self, rhs)
    }
}

impl Rem for JackValue {
    type Output = JackValue;
    fn rem(self, rhs: JackValue) -> JackValue {
        JackValue::rem(self, rhs)
    }
}

impl Neg for JackValue {
    type Output = JackValue;
    fn neg(self) -> JackValue {
        JackValue::neg(self)
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn rational_add(a: i32, b: i32, c: i32, d: i32) -> JackValue {
    // If either number is NaN, return NaN
    if a == 0 && b == 0 || c == 0 && d == 0 {
        JackValue::Rational(0, 0)
    }
    // If Both are Infinity, but have different signs, return nan
    else if b == 0 && d == 0 && (a < 0 && c > 0 || a > 0 && c < 0) {
        JackValue::Rational(0, 0)
    }
    // If the denominators match, simply add numerators
    else if b == d {
        JackValue::new_rational(a + c, d)
    }
    else {
        // Calculate the full value using GCD to prevent overflows in temporary values.
        let g = gcd(b, d);
        let num = a * (d / g) + c * (b / g);
        let den = (d / g) * b;
        if den == 1 {
            JackValue::Integer(num)
        }
        else {
            JackValue::new_rational(num, den)
        }
    }
}

fn rational_mul(a: i32, b: i32, c: i32, d: i32) -> JackValue {
    // TODO: we use gcd somehow to limit overflows in temporary values?
    JackValue::new_rational(a * c, b * d)
}

fn rational_rem(a: i32, b: i32, c: i32, d: i32) -> JackValue {
    if b == 0 { return JackValue::Rational(0, 0); }
    if c == 0 { return JackValue::Rational(a, b); }
    let num = a * d;
    let den = b * c;
    let k = num / den;
    rational_add(a, b, -k * c, d)
}

// 1/2 / 3/2 -> 3 r 0
// 3/2 / 2 -> 1 r 1/2
fn rational_idiv(a: i32, b: i32, c: i32, d: i32) -> JackValue {
    let num = a * d;
    let den = b * c;
    if den == 0 { return JackValue::Rational(num, den); }
    JackValue::Integer(num / den)
}
