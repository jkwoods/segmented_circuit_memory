use crate::utils::*;
use ::serde::{Deserialize, Serialize};
use ark_serialize::*;

#[derive(Clone, Eq, Debug, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MemType {
    Stack(usize, usize), //always private
    PubROM(usize, usize),
    PubRAM(usize, usize),
    PrivROM(usize, usize),
    PrivRAM(usize, usize),
}

impl MemType {
    pub fn priv_ram(tag: usize, elem_len: usize) -> Self {
        MemType::PrivRAM(tag, elem_len)
    }

    pub fn priv_rom(tag: usize, elem_len: usize) -> Self {
        MemType::PrivROM(tag, elem_len)
    }

    pub fn pub_ram(tag: usize, elem_len: usize) -> Self {
        MemType::PubRAM(tag, elem_len)
    }

    pub fn pub_rom(tag: usize, elem_len: usize) -> Self {
        MemType::PubROM(tag, elem_len)
    }

    pub fn stack(tag: usize, elem_len: usize) -> Self {
        MemType::Stack(tag, elem_len)
    }

    pub fn new(private: bool, ram: bool, stack: bool, tag: usize, elem_len: usize) -> Self {
        match (private, ram, stack) {
            (true, true, false) => MemType::PrivRAM(tag, elem_len),
            (true, false, false) => MemType::PrivROM(tag, elem_len),
            (false, true, false) => MemType::PubRAM(tag, elem_len),
            (false, false, false) => MemType::PubROM(tag, elem_len),
            (true, false, true) => MemType::Stack(tag, elem_len),
            _ => panic!("weird combination"),
        }
    }

    pub fn tag_replace(&mut self, sr_tag: usize) -> usize {
        let (t, m) = match &self {
            MemType::PubROM(t, l) => (*t, MemType::PubROM(sr_tag, *l)),
            MemType::PubRAM(t, l) => (*t, MemType::PubRAM(sr_tag, *l)),
            MemType::PrivROM(t, l) => (*t, MemType::PrivROM(sr_tag, *l)),
            MemType::PrivRAM(t, l) => (*t, MemType::PrivRAM(sr_tag, *l)),
            MemType::Stack(t, l) => (*t, MemType::Stack(sr_tag, *l)),
        };
        *self = m;
        t
    }

    pub fn elem_len(&self) -> usize {
        match &self {
            MemType::PubROM(_, l) => *l,
            MemType::PubRAM(_, l) => *l,
            MemType::PrivROM(_, l) => *l,
            MemType::PrivRAM(_, l) => *l,
            MemType::Stack(_, l) => *l,
        }
    }

    pub fn tag(&self) -> usize {
        match &self {
            MemType::PubROM(t, _) => *t,
            MemType::PubRAM(t, _) => *t,
            MemType::PrivROM(t, _) => *t,
            MemType::PrivRAM(t, _) => *t,
            MemType::Stack(t, _) => *t,
        }
    }

    pub fn is_stack(&self) -> bool {
        match &self {
            MemType::Stack(_, _) => true,
            _ => false,
        }
    }
}

impl CanonicalSerialize for MemType {
    #[inline]
    fn serialize_with_mode<W: Write>(
        &self,
        mut writer: W,
        mode: Compress,
    ) -> Result<(), SerializationError> {
        let (ty, tag, len): (u8, usize, usize) = match &self {
            MemType::PubROM(tag, len) => (0, *tag, *len),
            MemType::PubRAM(tag, len) => (1, *tag, *len),
            MemType::PrivROM(tag, len) => (2, *tag, *len),
            MemType::PrivRAM(tag, len) => (3, *tag, *len),
            MemType::Stack(tag, len) => (4, *tag, *len),
        };

        ty.serialize_with_mode(&mut writer, mode)?;
        tag.serialize_with_mode(&mut writer, mode)?;
        len.serialize_with_mode(&mut writer, mode)?;
        Ok(())
    }

    #[inline]
    fn serialized_size(&self, mode: Compress) -> usize {
        let (ty, tag, len): (u8, usize, usize) = match &self {
            MemType::PubROM(tag, len) => (0, *tag, *len),
            MemType::PubRAM(tag, len) => (1, *tag, *len),
            MemType::PrivROM(tag, len) => (2, *tag, *len),
            MemType::PrivRAM(tag, len) => (3, *tag, *len),
            MemType::Stack(tag, len) => (4, *tag, *len),
        };

        ty.serialized_size(mode) + tag.serialized_size(mode) + len.serialized_size(mode)
    }
}

impl CanonicalDeserialize for MemType {
    #[inline]
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let ty = u8::deserialize_with_mode(&mut reader, compress, validate)?;
        let tag = usize::deserialize_with_mode(&mut reader, compress, validate)?;
        let len = usize::deserialize_with_mode(&mut reader, compress, validate)?;

        match ty {
            0 => Ok(MemType::PubROM(tag, len)),
            1 => Ok(MemType::PubRAM(tag, len)),
            2 => Ok(MemType::PrivROM(tag, len)),
            3 => Ok(MemType::PrivRAM(tag, len)),
            4 => Ok(MemType::Stack(tag, len)),
            _ => Err(SerializationError::InvalidData),
        }
    }
}

impl Valid for MemType {
    #[inline]
    fn check(&self) -> Result<(), SerializationError> {
        let (ty, tag, len): (u8, usize, usize) = match &self {
            MemType::PubROM(tag, len) => (0, *tag, *len),
            MemType::PubRAM(tag, len) => (1, *tag, *len),
            MemType::PrivROM(tag, len) => (2, *tag, *len),
            MemType::PrivRAM(tag, len) => (3, *tag, *len),
            MemType::Stack(tag, len) => (4, *tag, *len),
        };

        ty.check()?;
        tag.check()?;
        len.check()?;
        Ok(())
    }
}

//nova elt
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ICCmt {
    v: Vec<N2>,
}

impl ICCmt {
    pub fn new(v: Vec<N2>) -> Self {
        Self { v }
    }

    pub fn inner(&self) -> &Vec<N2> {
        &self.v
    }
}

impl CanonicalSerialize for ICCmt {
    #[inline]
    fn serialize_with_mode<W: Write>(
        &self,
        mut writer: W,
        mode: Compress,
    ) -> Result<(), SerializationError> {
        let len: usize = self.v.len();
        len.serialize_with_mode(&mut writer, mode)?;

        for elt in &self.v {
            elt.to_bytes().serialize_with_mode(&mut writer, mode)?;
        }

        Ok(())
    }

    #[inline]
    fn serialized_size(&self, mode: Compress) -> usize {
        let len: usize = self.v.len();

        let mut sum = len.serialized_size(mode);
        for elt in &self.v {
            sum += elt.to_bytes().serialized_size(mode);
        }
        sum
    }
}

impl CanonicalDeserialize for ICCmt {
    #[inline]
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        let mut v = vec![];
        let len = usize::deserialize_with_mode(&mut reader, compress, validate)?;
        for _ in 0..len {
            let elt = <[u8; 32]>::deserialize_with_mode(&mut reader, compress, validate)?;
            v.push(N2::from_bytes(&elt).unwrap());
        }

        Ok(Self { v })
    }
}

impl Valid for ICCmt {
    #[inline]
    fn check(&self) -> Result<(), SerializationError> {
        let len: usize = self.v.len();
        len.check()?;
        for elt in &self.v {
            elt.to_bytes().check()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::memory::mem_type::*;
    use ff::Field;
    use rand::rngs::OsRng;
    use std::fmt::Debug;

    #[test]
    fn serialize_mem_types() {
        let a = MemType::priv_ram(4, 10);
        serialize_tester::<MemType>(a);

        let b = MemType::priv_rom(3, 40);
        serialize_tester::<MemType>(b);

        let c = MemType::pub_ram(99, 3);
        serialize_tester::<MemType>(c);

        let d = MemType::pub_rom(0, 19);
        serialize_tester::<MemType>(d);

        let e = MemType::stack(2, 55);
        serialize_tester::<MemType>(e);
    }

    #[test]
    fn serialize_cmt() {
        let mut v = vec![];
        for _ in 0..15 {
            v.push(N2::random(&mut OsRng));
        }

        let a = ICCmt::new(v);

        serialize_tester::<ICCmt>(a);
    }

    fn serialize_tester<
        T: CanonicalSerialize + CanonicalDeserialize + Valid + Debug + PartialEq,
    >(
        a: T,
    ) {
        let mut compressed_bytes = Vec::new();
        a.serialize_compressed(&mut compressed_bytes).unwrap();
        let mut uncompressed_bytes = Vec::new();
        a.serialize_uncompressed(&mut uncompressed_bytes).unwrap();

        let a_compressed = T::deserialize_compressed(&*compressed_bytes).unwrap();
        let a_uncompressed = T::deserialize_uncompressed(&*uncompressed_bytes).unwrap();

        assert_eq!(a_compressed, a);
        assert_eq!(a_uncompressed, a);
    }
}
