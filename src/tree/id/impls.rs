use super::ID;
use crate::util::XString;
use std::{fmt, ops::Deref};

impl Deref for ID {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl fmt::Debug for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Display>::fmt(self, f)
    }
}

impl From<&str> for ID {
    fn from(s: &str) -> Self {
        ID::new(s)
    }
}

impl From<XString> for ID {
    fn from(id: XString) -> Self {
        ID { id }
    }
}

impl From<ID> for XString {
    fn from(id: ID) -> Self {
        id.id
    }
}

impl From<&ID> for XString {
    fn from(id: &ID) -> Self {
        id.id.clone()
    }
}

impl ID {
    pub fn new(s: &str) -> ID {
        ID { id: s.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.id
    }
}
