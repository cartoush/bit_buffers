//! # Bit Buffers
//! 
//! `bit_buffers` is a collection of utilities designed to make it easy
//! to perform IO at the bit level, i.e. writing and reading individual 
//! bits to and from a file or buffer

mod bit_buffer;
pub mod bit_reader;
pub mod bit_writer;

pub use self::bit_reader::BitReader;
pub use self::bit_writer::BitWriter;