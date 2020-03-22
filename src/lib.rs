#![no_std]

//#![feature(const_generics)]
#![feature(const_fn)]
//#![allow(incomplete_features)]

/// Stolen from https://github.com/japaric/hash32/blob/master/src/lib.rs

pub trait Hasher32 {
    fn finish(&self) -> u32;
    fn write(&mut self, bytes: &[u32]);
}

pub trait Hash32 {
    /// Feeds this value into the given `Hasher`.
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher32;

    /// Feeds a slice of this type into the given `Hasher`.
    fn hash_slice<H>(data: &[Self], state: &mut H)
    where
        H: Hasher32,
        Self: Sized,
    {
        for piece in data {
            piece.hash(state);
        }
    }
}

pub trait DeltaEncode {
    fn straight(&mut self) -> &[u8];
    fn delta(&mut self) -> &[u8];
}

pub trait BlockPack<H>
where 
    H: FnMut(&[u32]) -> u32
{
    type Result;
    fn sample(&mut self, enc: &mut impl DeltaEncode, hasher: &mut H) -> Self::Result;
    fn event(&mut self, event: &[u8], hasher: &mut H) -> Self::Result;
    fn finalize(&mut self, hasher: &mut H) -> Self::Result;
}

pub trait BlockWrite {
    /// Write bytes to block
    fn write(&mut self, buf: &[u8]);
    /// Fill count with val
    fn write_byte(&mut self, val: u8, count: usize);
    /// Index words inside the block
    fn slice(&self) -> &'static [u32];
    /// Commit block to queues
    fn commit(&mut self);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
