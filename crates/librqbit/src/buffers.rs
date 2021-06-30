use serde::{Deserialize, Deserializer};

use crate::clone_to_owned::CloneToOwned;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ByteString(pub Vec<u8>);

#[derive(Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(transparent)]
pub struct ByteBuf<'a>(pub &'a [u8]);

pub trait ByteBufT {
    fn as_slice(&self) -> &[u8];
}

impl ByteBufT for ByteString {
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<'a> ByteBufT for ByteBuf<'a> {
    fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
}

fn debug_bytes(b: &[u8], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if b.iter().all(|b| *b == 0) {
        return write!(f, "<{} bytes, all zeroes>", b.len());
    }
    match std::str::from_utf8(b) {
        Ok(s) => write!(f, "{:?}", s),
        Err(_e) => write!(f, "<{} bytes>", b.len()),
    }
}

fn display_bytes(b: &[u8], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if b.iter().all(|b| *b == 0) {
        return write!(f, "<{} bytes, all zeroes>", b.len());
    }
    match std::str::from_utf8(b) {
        Ok(s) => write!(f, "{}", s),
        Err(_e) => write!(f, "<{} bytes>", b.len()),
    }
}

impl<'a> std::fmt::Debug for ByteBuf<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_bytes(self.0, f)
    }
}

impl<'a> std::fmt::Display for ByteBuf<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_bytes(self.0, f)
    }
}

impl std::fmt::Debug for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_bytes(&self.0, f)
    }
}

impl std::fmt::Display for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_bytes(&self.0, f)
    }
}

impl<B: ByteBufT> CloneToOwned for B {
    type Target = ByteString;

    fn clone_to_owned(&self) -> Self::Target {
        ByteString(self.as_slice().to_owned())
    }
}

impl<'a> std::convert::AsRef<[u8]> for ByteBuf<'a> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::convert::AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> std::ops::Deref for ByteBuf<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for ByteString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for ByteBuf<'a> {
    fn from(b: &'a [u8]) -> Self {
        Self(b)
    }
}

impl<'a> From<&'a [u8]> for ByteString {
    fn from(b: &'a [u8]) -> Self {
        Self(b.into())
    }
}

impl From<Vec<u8>> for ByteString {
    fn from(b: Vec<u8>) -> Self {
        Self(b)
    }
}

impl<'de> serde::de::Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("byte string")
            }
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.to_owned())
            }
        }
        Ok(ByteString(deserializer.deserialize_byte_buf(Visitor {})?))
    }
}
