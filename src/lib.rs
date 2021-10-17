//! Typed Unique Identifier gives you ability to create and use identifiers bound to specified type.
//!
//! Typical usage example:
//! ```
//! use typid::ID;
//!
//! struct Foo {
//!     pub id: ID<Foo>,
//! }
//!
//! let a = Foo { id: ID::new() };
//! let b = Foo { id: ID::new() };
//! assert_ne!(a.id, b.id);
//! ```

use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IDDef(pub String);

/// Typed Unique Identifier (uuidv4).
#[derive(Serialize, Deserialize)]
#[repr(C)]
#[serde(try_from = "IDDef")]
#[serde(into = "IDDef")]
pub struct ID<T> {
    id: Uuid,
    #[serde(skip_serializing, skip_deserializing)]
    _phantom: PhantomData<fn() -> T>,
}

impl<T> ID<T> {
    /// Creates new identifier.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates new identifier from raw bytes.
    #[inline]
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self {
            id: Uuid::from_bytes(bytes),
            _phantom: PhantomData,
        }
    }

    /// Gets underlying UUID object.
    #[inline]
    pub fn uuid(&self) -> Uuid {
        self.id
    }
}

impl<T> Default for ID<T> {
    #[inline]
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            _phantom: PhantomData,
        }
    }
}

impl<T> fmt::Debug for ID<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T> ToString for ID<T> {
    #[inline]
    fn to_string(&self) -> String {
        format!("{}", self.id)
    }
}

impl<T> FromStr for ID<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Uuid::parse_str(s) {
            Ok(uuid) => Ok(Self {
                id: uuid,
                _phantom: PhantomData,
            }),
            Err(_) => Err(s.to_owned()),
        }
    }
}

impl<T> Hash for ID<T> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state)
    }
}

impl<T> PartialEq for ID<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for ID<T> {}

impl<T> Copy for ID<T> {}

impl<T> PartialOrd for ID<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl<T> Ord for ID<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> Clone for ID<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T> TryFrom<IDDef> for ID<T> {
    type Error = String;

    fn try_from(id: IDDef) -> Result<Self, Self::Error> {
        Self::from_str(&id.0)
    }
}

#[allow(clippy::from_over_into)]
impl<T> Into<IDDef> for ID<T> {
    fn into(self) -> IDDef {
        IDDef(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general() {
        struct Foo {
            pub id: ID<Foo>,
        }

        let a = Foo { id: ID::new() };
        let b = Foo { id: ID::new() };
        assert_ne!(a.id, b.id);
    }
}
