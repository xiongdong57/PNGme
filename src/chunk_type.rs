use color_eyre::eyre::{eyre, Report};
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    pub bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Report;
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type_bytes: [u8; 4] = [
            bytes[0] as u8,
            bytes[1] as u8,
            bytes[2] as u8,
            bytes[3] as u8,
        ];
        Ok(ChunkType {
            bytes: chunk_type_bytes,
        })
    }
}

impl FromStr for ChunkType {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes().to_owned();
        let chunk_type_bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        for i in 0..4 {
            if !bytes[i].is_ascii_alphabetic() {
                return Err(eyre!("Chunk type must be alphanumeric"));
            }
        }

        Ok(ChunkType {
            bytes: chunk_type_bytes,
        })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

//    bLOb  <-- 32 bit chunk type code represented in text form
//    ||||
//    |||+- Safe-to-copy bit is 1 (lowercase letter; bit 5 is 1)
//    ||+-- Reserved bit is 0     (uppercase letter; bit 5 is 0)
//    |+--- Private bit is 0      (uppercase letter; bit 5 is 0)
//    +---- Ancillary bit is 1    (lowercase letter; bit 5 is 1)
#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
