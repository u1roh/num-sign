//! ```
//! use num_sign::*;
//! assert_eq!(Positive as i32, 1);
//! assert_eq!(Negative as i32, -1);
//! assert!(Negative < Positive);
//! assert_eq!(Positive.cmp(&Negative), std::cmp::Ordering::Greater);
//! assert_eq!(1_i32, Positive.into());
//! assert_eq!(-1_i32, Negative.into());
//! assert_eq!(1.0, Positive.into());
//! assert_eq!(-1.0, Negative.into());
//! assert_eq!(-Positive, Negative);
//! assert_eq!(Positive, -Negative);
//! assert_eq!(Positive * Positive, Positive);
//! assert_eq!(Positive * Negative, Negative);
//! assert_eq!(Negative * Positive, Negative);
//! assert_eq!(Negative * Negative, Positive);
//! assert_eq!(Positive * 123, 123);
//! assert_eq!(Negative * 123, -123);
//! assert_eq!(Positive * 3.14, 3.14);
//! assert_eq!(Negative * 3.14, -3.14);
//! assert_eq!((123).sign(), Some(Positive));
//! assert_eq!((-123).sign(), Some(Negative));
//! assert_eq!((0).sign(), None);
//! assert_eq!((3.14).sign(), Some(Positive));
//! assert_eq!((-3.14).sign(), Some(Negative));
//! assert_eq!((0.0).sign(), Some(Positive));
//! assert_eq!((-0.0).sign(), Some(Negative));
//! assert_eq!(std::f64::NAN.sign(), None);
//! assert_eq!(std::f64::INFINITY.sign(), Some(Positive));
//! assert_eq!(std::f64::NEG_INFINITY.sign(), Some(Negative));
//! assert_eq!("+".parse::<Sign>().unwrap(), Positive);
//! assert_eq!("-".parse::<Sign>().unwrap(), Negative);
//! assert_eq!(Positive.to_string(), "+");
//! assert_eq!(Negative.to_string(), "-");
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sign {
    Positive = 1,
    Negative = -1,
}

pub use Sign::*;

impl Sign {
    pub fn to_isize(self) -> isize {
        self.into()
    }
    pub fn to_i64(self) -> i64 {
        self.into()
    }
    pub fn to_i32(self) -> i32 {
        self.into()
    }
    pub fn to_i16(self) -> i16 {
        self.into()
    }
    pub fn to_i8(self) -> i16 {
        self.into()
    }

    /// (-1)^n (i.e. `if n % 2 == 0 { Positive } else { Negative }`)
    /// ```
    /// use num_sign::*;
    /// assert_eq!(Sign::parity(0), Positive);
    /// assert_eq!(Sign::parity(7), Negative);
    /// assert_eq!(Sign::parity(-2), Positive);
    /// assert_eq!(Sign::parity(false), Positive);
    /// assert_eq!(Sign::parity(true), Negative);
    /// ```
    pub fn parity(n: impl Into<i32>) -> Self {
        if n.into() % 2 == 0 {
            Positive
        } else {
            Negative
        }
    }
}

impl std::ops::Neg for Sign {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Positive => Negative,
            Negative => Positive,
        }
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Mul<T> for Sign {
    type Output = T;
    fn mul(self, rhs: T) -> T {
        match self {
            Positive => rhs,
            Negative => -rhs,
        }
    }
}

impl std::str::FromStr for Sign {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Positive),
            "-" => Ok(Negative),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Positive => "+",
            Negative => "-",
        })
    }
}

pub trait Signed {
    fn sign(&self) -> Option<Sign>;
}

macro_rules! impl_traits_for_int {
    ($($int_type: ty),*) => {
        $(
            impl From<Sign> for $int_type {
                fn from(sign: Sign) -> $int_type {
                    match sign {
                        Positive => 1,
                        Negative => -1,
                    }
                }
            }
            impl Signed for $int_type {
                fn sign(&self) -> Option<Sign> {
                    match self.cmp(&0) {
                        std::cmp::Ordering::Equal => None,
                        std::cmp::Ordering::Greater => Some(Positive),
                        std::cmp::Ordering::Less => Some(Negative),
                    }
                }
            }
        )*
    };
}

impl_traits_for_int!(isize, i64, i32, i16, i8);

impl From<Sign> for f64 {
    fn from(sign: Sign) -> f64 {
        match sign {
            Positive => 1.0,
            Negative => -1.0,
        }
    }
}
impl From<Sign> for f32 {
    fn from(sign: Sign) -> f32 {
        match sign {
            Positive => 1.0,
            Negative => -1.0,
        }
    }
}

impl Signed for f64 {
    fn sign(&self) -> Option<Sign> {
        match self.signum().partial_cmp(&0.0) {
            Some(std::cmp::Ordering::Greater) => Some(Positive),
            Some(std::cmp::Ordering::Less) => Some(Negative),
            _ => None,
        }
    }
}
impl Signed for f32 {
    fn sign(&self) -> Option<Sign> {
        match self.signum().partial_cmp(&0.0) {
            Some(std::cmp::Ordering::Greater) => Some(Positive),
            Some(std::cmp::Ordering::Less) => Some(Negative),
            _ => None,
        }
    }
}

#[cfg(feature = "serde")]
mod impl_serde {
    use super::*;
    impl serde::Serialize for Sign {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(match self {
                Positive => "+",
                Negative => "-",
            })
        }
    }
    struct SignVisitor;
    impl<'de> serde::de::Visitor<'de> for SignVisitor {
        type Value = Sign;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("\"+\" or \"-\"")
        }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
            match value {
                "+" => Ok(Positive),
                "-" => Ok(Negative),
                _ => Err(E::custom("Sign must be + or -")),
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for Sign {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_str(SignVisitor)
        }
    }
    #[test]
    fn test_serde() {
        assert_eq!(serde_json::to_string(&Positive).unwrap(), "\"+\"");
        assert_eq!(serde_json::to_string(&Negative).unwrap(), "\"-\"");
        assert_eq!(Positive, serde_json::from_str("\"+\"").unwrap());
        assert_eq!(Negative, serde_json::from_str("\"-\"").unwrap());
    }
}
