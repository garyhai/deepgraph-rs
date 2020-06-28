/*!
# Variants

## Plan

- ~~Plan.~~
- Design.
- MVP.

## Code

```rust
//
*/

use crate::{
    graph::{Hyperedge, Scalar},
    scalar,
};
use std::{
    cmp::{Ord, Ordering, PartialEq, PartialOrd},
    collections::BTreeMap,
    convert::TryFrom,
};

use SimpleValue::*;

pub trait PlainScalar:
    Scalar<bool> + Scalar<i128> + Scalar<f64> + Scalar<String> + Scalar<Vec<u8>>
{
    fn bool(&self) -> Option<bool> {
        scalar!(self, to, bool)
    }

    fn is_bool(&self) -> bool {
        scalar!(self, is, bool)
    }

    fn integer(&self) -> Option<i128> {
        scalar!(self, to, i128)
    }

    fn is_integer(&self) -> bool {
        scalar!(self, is, i128)
    }

    fn as_integer(&self) -> Option<i128> {
        scalar!(self, as, i128)
    }

    fn as_usize(&self) -> Option<usize> {
        self.integer().and_then(|n| usize::try_from(n).ok())
    }

    fn as_u64(&self) -> Option<u64> {
        self.integer().and_then(|n| u64::try_from(n).ok())
    }

    fn float(&self) -> Option<f64> {
        scalar!(self, to, f64)
    }

    fn is_float(&self) -> bool {
        scalar!(self, is, f64)
    }

    fn as_float(&self) -> Option<f64> {
        scalar!(self, as, f64)
    }

    fn text(&self) -> Option<&str> {
        scalar!(self, String).map(String::as_str)
    }

    fn is_text(&self) -> bool {
        scalar!(self, is, String)
    }

    fn as_text(&self) -> Option<String> {
        scalar!(self, as, String)
    }

    fn binary(&self) -> Option<&[u8]> {
        scalar!(self, Vec<u8>).map(Vec::as_ref)
    }

    fn is_binary(&self) -> bool {
        scalar!(self, is, Vec<u8>)
    }

    fn as_binary(&self) -> Option<Vec<u8>> {
        scalar!(self, as, Vec<u8>)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SimpleValue {
    Bool(bool),
    Integer(i128),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
}

impl SimpleValue {
    fn weight(&self) -> usize {
        match self {
            Bool(_) => 1,
            Integer(_) => 2,
            Float(_) => 3,
            Text(_) => 4,
            Bytes(_) => 5,
        }
    }
}

impl PlainScalar for SimpleValue {}

impl Scalar<bool> for SimpleValue {
    fn scalar(&self) -> Option<&bool> {
        if let Bool(x) = self {
            Some(x)
        } else {
            None
        }
    }

    fn to_scalar(&self) -> Option<bool> {
        self.scalar().map(|x| *x)
    }

    fn as_scalar(&self) -> Option<bool> {
        Some(match self {
            Bool(true) => true,
            Integer(x) if *x != 0 => true,
            Text(x) if !x.is_empty() => true,
            Bytes(x) if !x.is_empty() => true,
            Float(_) => return None,
            _ => false,
        })
    }
}

impl Scalar<i128> for SimpleValue {
    fn scalar(&self) -> Option<&i128> {
        if let Integer(x) = self {
            Some(x)
        } else {
            None
        }
    }

    fn to_scalar(&self) -> Option<i128> {
        self.scalar().map(|x| *x)
    }

    fn as_scalar(&self) -> Option<i128> {
        match self {
            Integer(x) => Some(*x),
            Text(x) => x.parse().ok(),
            // convert binary as big-endian data to i128
            Bytes(x) => {
                let sz = x.len();
                if sz > 16 {
                    None
                } else {
                    let start = 16 - sz;
                    let mut b = [0; 16];
                    b[start..].copy_from_slice(x);
                    Some(i128::from_be_bytes(b))
                }
            }
            Float(x) => Some(x.round() as i128),
            _ => None,
        }
    }
}

impl Scalar<f64> for SimpleValue {
    fn scalar(&self) -> Option<&f64> {
        if let Float(x) = self {
            Some(x)
        } else {
            None
        }
    }

    fn to_scalar(&self) -> Option<f64> {
        self.scalar().map(|x| *x)
    }

    fn as_scalar(&self) -> Option<f64> {
        match self {
            Integer(x) => Some((*x) as f64),
            Float(x) => Some(*x),
            Text(x) => x.parse().ok(),
            Bytes(x) if x.len() == 8 => {
                let mut b = [0; 8];
                b.copy_from_slice(&x[..]);
                Some(f64::from_be_bytes(b))
            }
            _ => None,
        }
    }
}

impl Scalar<String> for SimpleValue {
    fn scalar(&self) -> Option<&String> {
        if let Text(x) = self {
            Some(x)
        } else {
            None
        }
    }

    fn to_scalar(&self) -> Option<String> {
        self.scalar().map(Clone::clone)
    }

    fn as_scalar(&self) -> Option<String> {
        Some(match self {
            Bool(x) => x.to_string(),
            Integer(x) => x.to_string(),
            Text(x) => x.clone(),
            Bytes(x) => String::from_utf8_lossy(x).to_string(),
            Float(x) => x.to_string(),
        })
    }
}

impl Scalar<Vec<u8>> for SimpleValue {
    fn scalar(&self) -> Option<&Vec<u8>> {
        if let Bytes(x) = self {
            Some(x)
        } else {
            None
        }
    }

    fn to_scalar(&self) -> Option<Vec<u8>> {
        self.scalar().map(Clone::clone)
    }

    fn as_scalar(&self) -> Option<Vec<u8>> {
        Some(match self {
            Bool(true) => vec![1],
            Bool(false) => vec![0],
            Integer(x) => x.to_be_bytes().to_vec(),
            Text(x) => x.as_bytes().to_vec(),
            Bytes(x) => x.clone(),
            Float(x) => x.to_be_bytes().to_vec(),
        })
    }
}

impl Eq for SimpleValue {}

impl Ord for SimpleValue {
    fn cmp(&self, other: &SimpleValue) -> Ordering {
        if let Some(ord) = self.partial_cmp(other) {
            ord
        } else {
            self.weight().cmp(&other.weight())
        }
    }
}

pub enum Variant {
    Null,
    Scalar(SimpleValue),
    Array(Vec<Variant>),
    Table(BTreeMap<SimpleValue, Variant>),
    Index(SimpleValue),
}

impl Hyperedge<Variant> for usize {
    type Output = Variant;

    fn get(self, vector: &Variant) -> Option<&Self::Output> {
        if let Variant::Array(x) = vector {
            x.get(self)
        } else {
            None
        }
    }

    fn get_mut(self, vector: &mut Variant) -> Option<&mut Self::Output> {
        if let Variant::Array(x) = vector {
            x.get_mut(self)
        } else {
            None
        }
    }
}

impl Hyperedge<Variant> for &SimpleValue {
    type Output = Variant;

    fn get(self, vector: &Variant) -> Option<&Self::Output> {
        match vector {
            Variant::Table(x) => x.get(self),
            Variant::Array(x) if self.is_integer() => self.as_usize().and_then(|n| x.get(n)),
            _ => None,
        }
    }

    fn get_mut(self, vector: &mut Variant) -> Option<&mut Self::Output> {
        match vector {
            Variant::Table(x) => x.get_mut(self),
            Variant::Array(x) if self.is_integer() => {
                self.as_usize().and_then(move |n| x.get_mut(n))
            }
            _ => None,
        }
    }
}

impl Hyperedge<Variant> for SimpleValue {
    type Output = Variant;

    fn get(self, vector: &Variant) -> Option<&Self::Output> {
        (&self).get(vector)
    }

    fn get_mut(self, vector: &mut Variant) -> Option<&mut Self::Output> {
        (&self).get_mut(vector)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_graph() {}
}

/*
```
*/
