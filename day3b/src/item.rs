use std::fmt::{Debug, Display};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Item(u8);

impl Item {
    pub fn priority(&self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Item {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as char {
            'a'..='z' | 'A'..='Z' => Ok(Item(value)),
            _ => Err(()),
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
