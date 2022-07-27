use std::io;
use super::bit_buffer::BitBuffer;

/// A BitReader holds a buffer used to store the individual bits read in from 
/// a file.
#[derive(Default)]
pub struct BitReader {
    // The 'position' field represents which bit in the buffer is currently 
    // being indexed
    position: u128,
    
    // The 'buffer' holds the actual bit buffer (represented by a BitBuffer) 
    // that is being read from
    buffer: BitBuffer,
}

impl BitReader {
    /// Returns a new instance of the BitReader struct with an empty buffer.
    /// 
    /// # Examples
    /// ```
    /// use bit_io::BitReader;
    /// let _bit_reader = BitReader::new();
    /// ```
    pub fn new() -> BitReader {
        BitReader {
            position: 0,
            buffer: BitBuffer::new(),
        }
    }// new
    
    /// Loads the bits stored in a file into this BitReader.
    /// 
    /// # Arguments
    /// 
    /// * `file_name` (&str) - A string slice that holds the name of the file
    /// 
    /// # Errors
    /// This function will return an error if `file_name` does not already 
    /// exist. Other errors may be returned according to `OpenOptions::open`.
    /// 
    /// It will also return an error if it encounters while reading an error of 
    /// a kind other than `io::ErrorKind::Interrupted`.
    ///
    /// # Examples
    /// ```
    /// use bit_io::BitReader;
    /// let mut bit_reader = BitReader::new();
    /// bit_reader.load_from_file("my_bit_file.bit");
    /// ```
    pub fn load_from_file(&mut self, file_name: &str) -> io::Result<()> {
        // Reset this reader
        self.position = 0;
        self.buffer.flush();
        
        // Load bit data into this reader
        self.buffer.load_from_file(file_name)
    }// load_from_file
    
    /// Reads the current bit being referenced by this BitReader.
    /// 
    /// # Examples
    /// ```
    /// use bit_io::BitReader;
    /// let mut bit_reader = BitReader::new();
    /// bit_reader.load_from_file("my_bit_file.bit");
    /// let bit_option = bit_reader.read_bit();
    /// if let Some(bit) = bit_option {
    ///     println!("Read in a {} bit from bit_reader", bit);
    /// }
    /// ```
    pub fn read_bit(&mut self) -> Option<u8> {
        
        // If we already read the last bit, return None
        if self.position >= self.buffer.get_count() {
            return None;
        }
        
        // Extract the bit
        let bit_option = Some(self.buffer.get_bit(self.position));
        
        // Increment the position to get the next bit
        self.position += 1;
        
        // Return the retrieved bit
        bit_option
    }// read_bit
    
}