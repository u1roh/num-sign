# Sign type

## `Sign` enum
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sign {
    Positive = 1,
    Negative = -1,
}

pub use Sign::*;
```

## `Signed` trait
```rust
pub trait Signed {
    fn sign(&self) -> Option<Sign>;
}
```

## Tests
```rust
use num_sign::*;
assert_eq!(Positive as i32, 1);
assert_eq!(Negative as i32, -1);
assert!(Negative < Positive);
assert_eq!(Positive.cmp(&Negative), std::cmp::Ordering::Greater);
assert_eq!(1_i32, Positive.into());
assert_eq!(-1_i32, Negative.into());
assert_eq!(1.0, Positive.into());
assert_eq!(-1.0, Negative.into());
assert_eq!(-Positive, Negative);
assert_eq!(Positive, -Negative);
assert_eq!(Positive * Positive, Positive);
assert_eq!(Positive * Negative, Negative);
assert_eq!(Negative * Positive, Negative);
assert_eq!(Negative * Negative, Positive);
assert_eq!(Positive * 123, 123);
assert_eq!(Negative * 123, -123);
assert_eq!(Positive * 3.14, 3.14);
assert_eq!(Negative * 3.14, -3.14);
assert_eq!((123).sign(), Some(Positive));
assert_eq!((-123).sign(), Some(Negative));
assert_eq!((0).sign(), None);
assert_eq!((3.14).sign(), Some(Positive));
assert_eq!((-3.14).sign(), Some(Negative));
assert_eq!((0.0).sign(), Some(Positive));
assert_eq!((-0.0).sign(), Some(Negative));
assert_eq!(std::f64::NAN.sign(), None);
assert_eq!(std::f64::INFINITY.sign(), Some(Positive));
assert_eq!(std::f64::NEG_INFINITY.sign(), Some(Negative));
assert_eq!("+1".parse::<Sign>().unwrap(), Positive);
assert_eq!("-1".parse::<Sign>().unwrap(), Negative);
assert_eq!("Positive".parse::<Sign>().unwrap(), Positive);
assert_eq!("Negative".parse::<Sign>().unwrap(), Negative);
assert_eq!(Positive.to_string(), "+1");
assert_eq!(Negative.to_string(), "-1");
```
