use crate::chunk_type::ChunkType;
use color_eyre::eyre::{eyre, Report, Result};
use crc::Crc;
use std::fmt;
pub struct Chunk {
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Report;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let data_length = u32::from_be_bytes(bytes[0..4].try_into()?);
        let chunk_type_bytes: [u8; 4] = [
            bytes[4] as u8,
            bytes[5] as u8,
            bytes[6] as u8,
            bytes[7] as u8,
        ];
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;
        let data = bytes[8..(8 + data_length as usize)].to_vec();

        // check crc
        let crc = u32::from_be_bytes(
            bytes[8 + data_length as usize..12 + data_length as usize].try_into()?,
        );
        let hasher = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut digest = hasher.digest();
        digest.update(&chunk_type_bytes);
        digest.update(&data);
        let calculated_crc = digest.finalize() as u32;
        if calculated_crc != crc {
            return Err(eyre!(
                "CRC failed, calculated CRC: {}, chunk CRC: {}",
                calculated_crc,
                crc
            ));
        }

        Ok(Chunk { chunk_type, data })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Chunk { chunk_type, data }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        let hasher = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut digest = hasher.digest();
        digest.update(&self.chunk_type.bytes);
        digest.update(&self.data);
        digest.finalize() as u32
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8_lossy(&self.data).to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let data: Vec<u8> = self
            .length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes.iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect();
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
