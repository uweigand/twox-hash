use core::hash::Hasher;

use digest::{
    generic_array::{
        typenum::consts::{U16, U4, U8},
        GenericArray,
    },
    Digest,
};

use crate::{xxh3, XxHash32, XxHash64};

impl Digest for XxHash32 {
    type OutputSize = U4;

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.write(data.as_ref());
    }

    fn chain(mut self, data: impl AsRef<[u8]>) -> Self
    where
        Self: Sized,
    {
        self.update(data);
        self
    }

    fn finalize(self) -> GenericArray<u8, Self::OutputSize> {
        self.finish().to_be_bytes().into()
    }

    fn finalize_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.clone().finalize();
        self.reset();
        result
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn output_size() -> usize {
        4
    }

    fn digest(data: &[u8]) -> GenericArray<u8, Self::OutputSize> {
        Self::new().chain(data).finalize()
    }
}

impl Digest for XxHash64 {
    type OutputSize = U8;

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.write(data.as_ref());
    }

    fn chain(mut self, data: impl AsRef<[u8]>) -> Self
    where
        Self: Sized,
    {
        self.update(data);
        self
    }

    fn finalize(self) -> GenericArray<u8, Self::OutputSize> {
        self.finish().to_be_bytes().into()
    }

    fn finalize_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.clone().finalize();
        self.reset();
        result
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn output_size() -> usize {
        8
    }

    fn digest(data: &[u8]) -> GenericArray<u8, Self::OutputSize> {
        Self::new().chain(data).finalize()
    }
}

impl Digest for xxh3::Hash64 {
    type OutputSize = U8;

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.write(data.as_ref());
    }

    fn chain(mut self, data: impl AsRef<[u8]>) -> Self
    where
        Self: Sized,
    {
        self.update(data);
        self
    }

    fn finalize(self) -> GenericArray<u8, Self::OutputSize> {
        self.finish().to_be_bytes().into()
    }

    fn finalize_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.clone().finalize();
        self.reset();
        result
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn output_size() -> usize {
        8
    }

    fn digest(data: &[u8]) -> GenericArray<u8, Self::OutputSize> {
        Self::new().chain(data).finalize()
    }
}

impl Digest for xxh3::Hash128 {
    type OutputSize = U16;

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.write(data.as_ref());
    }

    fn chain(mut self, data: impl AsRef<[u8]>) -> Self
    where
        Self: Sized,
    {
        self.update(data);
        self
    }

    fn finalize(self) -> GenericArray<u8, Self::OutputSize> {
        xxh3::HasherExt::finish_ext(&self).to_be_bytes().into()
    }

    fn finalize_reset(&mut self) -> GenericArray<u8, Self::OutputSize> {
        let result = self.clone().finalize();
        self.reset();
        result
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    fn output_size() -> usize {
        8
    }

    fn digest(data: &[u8]) -> GenericArray<u8, Self::OutputSize> {
        Self::new().chain(data).finalize()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::prelude::v1::*;

    #[test]
    fn ingesting_byte_by_byte_is_equivalent_to_large_chunks_64() {
        let bytes: Vec<_> = (0..32).map(|_| 0).collect();

        let mut byte_by_byte = XxHash64::new();
        for byte in bytes.chunks(1) {
            byte_by_byte.update(byte);
        }

        let mut one_chunk = XxHash64::new();
        one_chunk.update(&bytes);

        assert_eq!(byte_by_byte.finalize(), one_chunk.finalize());
    }

    #[test]
    fn hash_of_nothing_matches_c_implementation_64() {
        let mut hasher = XxHash64::new();
        hasher.update(&[]);
        assert_eq!(
            hasher.finalize()[..],
            0xef46_db37_51d8_e999_u64.to_be_bytes()
        );
    }

    #[test]
    fn hash_of_single_byte_matches_c_implementation_64() {
        let mut hasher = XxHash64::new();
        hasher.update(&[42]);
        assert_eq!(
            hasher.finalize()[..],
            0x0a9e_dece_beb0_3ae4_u64.to_be_bytes()
        );
    }

    #[test]
    fn hash_of_multiple_bytes_matches_c_implementation_64() {
        assert_eq!(
            XxHash64::digest(b"Hello, world!\0")[..],
            0x7b06_c531_ea43_e89f_u64.to_be_bytes()
        );
    }

    #[test]
    fn hash_of_multiple_chunks_matches_c_implementation_64() {
        let bytes: Vec<_> = (0..100).collect();
        assert_eq!(
            XxHash64::digest(&bytes)[..],
            0x6ac1_e580_3216_6597_u64.to_be_bytes()
        );
    }

    #[test]
    fn hash_with_different_seed_matches_c_implementation_64() {
        let mut hasher = XxHash64::with_seed(0xae05_4331_1b70_2d91);
        hasher.update(&[]);
        assert_eq!(
            hasher.finalize()[..],
            0x4b6a_04fc_df7a_4672_u64.to_be_bytes()
        );
    }

    #[test]
    fn hash_with_different_seed_and_multiple_chunks_matches_c_implementation_64() {
        let bytes: Vec<_> = (0..100).collect();
        let mut hasher = XxHash64::with_seed(0xae05_4331_1b70_2d91);
        hasher.update(&bytes);
        assert_eq!(
            hasher.finalize()[..],
            0x567e_355e_0682_e1f1_u64.to_be_bytes()
        );
    }

    #[test]
    fn ingesting_byte_by_byte_is_equivalent_to_large_chunks_32() {
        let bytes: Vec<_> = (0..32).map(|_| 0).collect();

        let mut byte_by_byte = XxHash32::new();
        for byte in bytes.chunks(1) {
            byte_by_byte.update(byte);
        }

        let mut one_chunk = XxHash32::new();
        one_chunk.update(&bytes);

        assert_eq!(byte_by_byte.finalize(), one_chunk.finalize());
    }

    #[test]
    fn hash_of_nothing_matches_c_implementation_32() {
        let mut hasher = XxHash32::new();
        hasher.update(&[]);
        assert_eq!(hasher.finalize()[..], 0x02cc_5d05_u32.to_be_bytes());
    }

    #[test]
    fn hash_of_single_byte_matches_c_implementation_32() {
        let mut hasher = XxHash32::new();
        hasher.update(&[42]);
        assert_eq!(hasher.finalize()[..], 0xe0fe_705f_u32.to_be_bytes());
    }

    #[test]
    fn hash_of_multiple_bytes_matches_c_implementation_32() {
        assert_eq!(
            XxHash32::digest(b"Hello, world!\0")[..],
            0x9e5e_7e93_u32.to_be_bytes()
        );
    }

    #[test]
    fn hash_of_multiple_chunks_matches_c_implementation_32() {
        let bytes: Vec<_> = (0..100).collect();
        assert_eq!(XxHash32::digest(&bytes)[..], 0x7f89_ba44_u32.to_be_bytes());
    }

    #[test]
    fn hash_with_different_seed_matches_c_implementation_32() {
        let mut hasher = XxHash32::with_seed(0x42c9_1977);
        hasher.update(&[]);
        assert_eq!(hasher.finalize()[..], 0xd6bf_8459_u32.to_be_bytes());
    }

    #[test]
    fn hash_with_different_seed_and_multiple_chunks_matches_c_implementation_32() {
        let bytes: Vec<_> = (0..100).collect();
        let mut hasher = XxHash32::with_seed(0x42c9_1977);
        hasher.update(&bytes);
        assert_eq!(hasher.finalize()[..], 0x6d2f_6c17_u32.to_be_bytes());
    }
}
