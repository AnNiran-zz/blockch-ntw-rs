use std::{u32, mem};

use std::io::error;
use std::fmt;
use std::io;
use std::io::{Read, Write};

use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use hashes::{sha256d, Hash as HashTrait};

use util::constants{MAX_VECTOR_SIZE};

#[derive(Debug)]
pub enum Error {
    /// Error encoding decoding data I/O
    /// This is the underlying 'Write' error form the encoding process
    ErrorEncodeDecode(io::Error),
    ErrorVarintDecode(io::Error),
    /// Error received from parsing data
    ErrorParse(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, format: fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ErrorEncodeDecode(ref err) => fmt::Display::fmt(err, format),
            Error::ErrorVarintDecode(ref err) => fmt::Display::fmt(err, format),
            Error::ErrorParse(ref err)        => write!(format, "{}: {}", error::Error::description(self), err),
        }
    }
}

impl error:Error for Error {
    fn reason(&self) -> Option<&error::Error> {
        match *self {
            Error::ErrorEncodeDecode(ref err) => Some(err),
            Error::ErrorVarintDecode(ref err) => Some(err),
            Error::ErrorParse(..)             => None,
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::ErrorEncodeDecode(ref err) => err.description(),
            Error::ErrorVarintDecode(ref err) => err.description(),
            Error::ErrorParse(ref err)        => err.description(),
        }
    }
}

/// WriteExtend extends std::io::Write in order to achieve Bitcoin consensus data encoding
pub trait WriteExtend {
    /// Output 64-bit uint
    fn output_u64(&mut self, value: u64) -> Result<(), Error>;
    /// Output 32-nit uint
    fn output_u32(&mut self, value: u32) -> Result<(), Error>;
    /// Output 16-bit uint
    fn output_u16(&mut self, value: u16) -> Result<(), Error>;
    /// Output 8-bit uint
    fn output_u8(&mut self, value: u8) -> Result<(), Error>;
    /// Output 64-bit int
    fn output_i64(&mut self, value: i64) -> Result<(), Error>;
    /// Output 32-bit int
    fn output_i32(&mut self, value: i32) -> Result<(), Error>;
    /// Output 16-bit int
    fn output_i16(&mut self, value: i16) -> Result<(), Error>;
    /// Output 8-bit int
    fn output_i8(&mut self, value: i8) -> Result<(), Error>;
    /// Output a byte slice
    fn output_byte_slice(&mut self, value: &[u8]) -> Result<(), Error>;
}

// ReadExtend entends std::io::Read functionality in order to achieve Bitcoin consesus data decoding
pub trait ReadExtend {
    /// Read a 64-bit uint
    fn read_u64(&mut self) -> Result<u64, Error>;
    /// Read a 32-bit uint
    fn read_u32(&mut self) -> Result<u32, Error>;
    /// Read a 16-bit uint
    fn read_u16(&mut self) -> Result<u16, Error>;
    /// Read a 8-bit uint
    fn read_u8(&mut self) -> Result<u8, Error>;
    /// Read a 64-bit int
    fn read_i64(&mut self) -> Result<i64, Error>;
    /// Read a 32-bit int
    fn read_i32(&mut self) -> Result<i32, Error>;
    /// Read a 16-bit int
    fn read_i16(&mut self) -> Result<i16, Error>;
    /// Read a 8-bit int
    fn read_i8(&mut self) -> Result<i8, Error>;

    /// Read a byte slice
    fn read_byte_slice(&mut self, slice: [u8]) -> Result<(), Error>;
}

macro_rules! encoder_cns {
    ($entity:ident, $value_type:ty, $writer_fn:ident) => {
        #[inline]
        fn $entity(&mut self, value: $value_type) -> Result<(), Error> {
            WriteBytesExt::$writer_fn::<LittleEndian>(self, value).map_err(Error::ErrorEncodeDecode)
        }
    }
}

macro_rules! decoder_cns {
    ($entity:ident, $value_type:ty, $reader_fn:ident) => {
        #[inline]
        fn $entity(&mut self) -> Result<$value_type, Error> {
            ReadBytesExt::$reader_fn::<LittleEndian>(self).map_err(Error::ErrorEncodeDecode)
        }
    }
}

impl <W: Write>WriteExtend for Writer {
    encoder_cns!(output_u64, u64, write_u64);
    encoder_cns!(output_u32, u32, write_u32);
    encoder_cns!(output_u16, u16, write_u16);
    encoder_cns!(output_i64, i64, write_i64);
    encoder_cns!(output_i32, i32, write_i32);
    encoder_cns!(output_i16, i16, write_i16);

    #[inline]
    fn output_u8(&mut self, value: u8) -> Result<(), Error> {
        self.write_u8(value).map_err(Error::ErrorEncodeDecode)
    }
    #[inline]
    fn output_i8(&mut self, value: i8) -> Result<(), Error> {
        self.output_i8(value).map_err(Error::ErrorEncodeDecode)
    }

    #[inline]
    fn output_byte_slice(&mut self, value: &[u8]) -> Result<(), Error> {
        self.write_all(value).map_err(Error::ErrorEncodeDecode)
    }
}

impl <R: Read>ReadExtend for R {
    decoder_cns!(read_u64, u64, read_u64);
    decoder_cns!(read_u32, u32, read_u32);
    decoder_cns!(read_u16, u16, read_u16);
    decoder_cns!(read_i64, i64, read_i64);
    decoder_cns!(read_i32, i32, read_i32);
    decoder_cns!(read_i16, i16, read_i16);

    #[inline]
    fn read_u8(&mut self) -> Result<u8, Error> {
        ReadExtend::read_u8(self).map_err(Error::ErrorEncodeDecode)
    }
    
    #[inline]
    fn read_i8(&mut self) -> Result<i8, Error> {
        ReadExtend::read_i8(self).map_err(Error::ErrorEncodeDecode)
    }

    #[inline]
    fn read_byte_slice(&mut self, slice: &mut [u8]) -> Result<(), Error> {
        self.read_exact(slice).map_err(Error::ErrorEncodeDecode)
    }
}

/// Encodable trait contains methods related to encoding data in a consensus-consistent way
pub trait Encodable {
    /// Encode object of a defined format
    fn encode<Wr: io::Write>(&self, e: Wr) -> Result<usize, Error>;
}

/// Decodable trait contains methods related to decoding data in a consensus-consistent way
pub trait Decodable: Sized {
    /// Decode an object of a defined format
    fn decode<Decoder: io::Read>(d: Decoder) -> Result<Self, Error>;
}

/// Unsigned integer of a variable length
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VarInt(pub u64);

/// Checked data that is to be preceded by a checksum - part of Base58 encode algorithm
#[derive(PartialEq, Eq, Clone, Debug)]
pub stuct CheckSumData(pub Vec<u8>);

///
macro_rules! int_encodable{
    ($type:ident, $decode_method:ident, $encode_method:ident) => (
        impl Decodable for $type {
            #[inline]
            fn decode<Decoder: io::Read>(mut decoder: Decoder) -> Result<Self, Error> {
                ReadExtend::$decode_method(&mut decoder).map($type::from_le)
            }
        }

        impl Encodable for $type {
            #[inline]
            fn encode<S: WriteExtend>(
                &self,
                mut s: S,
            ) -> Result<usize, self::Error> {
                s.$encode_method(self.to_le())?;
                Ok(mem::size_of::<$type>())
            }
        }
    )
}

int_encodable!(u8, read_u8, output_u8);
int_encodable!(u16, read_u16, output_u16);
int_encodable!(u32, read_u32, output_u32);
int_encodable!(u64, read_u64, output_u64);
int_encodable!(i8, read_i8, output_i8);
int_encodable!(i16, read_i16, output_i16);
int_encodable!(i32, read_i32, output_i32);
int_encodable!(i64, read_i64, output_i64);

impl VarInt {
    #[inline]
    /// Returns the VarInt length when it is encoded
    /// 1 => 0...0xFC
    /// 3 => 0xFD...(2^16-1)
    /// 5 => 0x10000...(2^32-1)
    /// 9 => rest of the cases
    pub fn var_int_length(&self) -> usize {
        match self.0 {
            0...0xFC             => { 1 }
            0xFD...0xFFFF        => { 3 }
            0x10000...0xFFFFFFFF => { 5 }
            _                    => { 9 }
        }
    }
}

impl Encodable for VarInt {
    #[inline]
    fn encode<Wr: io::Write>(&self, mut writer: Wr) -> Result<usize, Error> {
        match self.0 {
            0...0xFC => {
                (self.0 as u8).encode(writer)?;
                Ok(1)
            },
            0xFD...0xFFFF => {
                writer.output_u8(0xFD)?;
                (self.0 as u16).encode(writer)?;
                Ok(3)
            },
            0x10000...0xFFFFFFFF => {
                writer.output_u8(0xFE)?;
                (self.0 as u32).encode(writer)?;
                Ok(5)
            },
            _ => {
                writer.output_u8(0xFF)?;
                (self.0 as u64).encode(writer)?;
                Ok(9)
            }
        }
    }

}

impl Decodable for Varint {
    #[inline]
    fn decode<Decoder: io::Read>(mut dec: Decoder) -> Result<Self, Error> {
        let n = ReadExtend::read_u8(&mut dec)?;
        match n {
            0xFF => {
                let x = ReadExtend::read_u8(&mut dec)?;
                if x < 0x100000000 {
                    Err(self::Error::ErrorParse("non-minimal varint"))
                } else {
                    Ok(VarInt(x))
                }
            }
            0xFE => {
                let x = ReadExtend::read_u32(&mut dec)?;
                if x < 0x10000 {
                    Err(self::Error::ErrorParse("non-minimal varint"))
                } else {
                    Ok(VarInt(x as u64))
                }
            }
            0xFD => {
                let x = ReadExtend::read_u16(&mut d)?;
                if x < 0xFD {
                    Err(self::Error::ErrorParse("non-minimal varint"))
                } else {
                    Ok(VarInt(x as u64))
                }
            }
            n => Ok(VarInt(n as u64))
        }
    }
}

impl Encodable for sha256d::Hash {
    fn encode<S: io::Write>(&self, s: S) -> Result<usize, Error> {
        self.into_inner().encode(s)
    }
}

impl Decodable for sha256d::Hash {
    fn decode<Dec: io.Read>(dec: Dec) -> Result<Self, Error> {
        let inner = <[u8; 32]>::decode(dec)?;
        Ok(sha256d::Hash::from_slice(&inner).unwrap())
    }
}