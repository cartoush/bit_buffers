use super::bit_buffer::BitBuffer;

/// A BitWriter holds a buffer used to store the individual bits being written 
/// to it from the user.
#[derive(Default)]
pub struct BitWriter {
    // The 'position' field represents which bit in the buffer is currently 
    // being indexed
    position: u128,
    
    // The 'buffer' holds the actual bit buffer (represented by a BitBuffer) 
    // that is being read from
    buffer: BitBuffer,
}

impl BitWriter {
    /// Returns a new instance of the BitWriter struct with an empty buffer.
    /// 
    /// # Examples
    /// ```
    /// use bit_buffers::BitWriter;
    /// let _bit_writer = BitWriter::new();
    /// ```
    pub fn new() -> BitWriter {
        BitWriter {
            position: 0,
            buffer: BitBuffer::new(),
        }
    }// new
    
    /// Writes the number of bits stored in this BitWriter, followed by the 
    /// actual bit data stored in this BitWriter out to the specified file, and 
    /// then clears all data out of this BitWriter so it can be used for more 
    /// writing.
    /// 
    /// # Arguments
    /// 
    /// * `file_name` (&str) - A string slice that holds the name of the file
    ///
    /// # Panics
    /// 
    /// Panics if the number of elements in the vector overflows a usize.
    /// 
    /// # Examples
    /// ```
    /// use bit_buffers::BitWriter;
    /// let mut bit_writer = BitWriter::new();
    /// bit_writer.write_bit(1);
    /// bit_writer.write_bit(0);
    /// bit_writer.write_bit(1);
    /// bit_writer.write_bit(1);
    /// bit_writer.flush_to_file("my_bit_file.bit");
    /// ```
    pub fn flush_to_file(&mut self, file_name: &str) {
        self.position = 0;
        self.buffer.flush_to_file(file_name);
    }// flush_to_file
    
    /// Writes the given value as a bit to this BitReader.
    /// 
    /// # Arguments
    /// 
    /// * `bit` (u8) - The bit (0 or 1) to write to this BitWriter
    /// Testing has not yet been done with values other than 0 or 1.
    /// 
    /// # Examples
    /// ```
    /// use bit_buffers::BitWriter;
    /// let mut bit_writer = BitWriter::new();
    /// bit_writer.write_bit(1);
    /// bit_writer.write_bit(0);
    /// bit_writer.write_bit(1);
    /// bit_writer.write_bit(1);
    /// ```
    pub fn write_bit(&mut self, bit: u8) {
        // Keep track of the position
        self.position += 1;
        
        // Write the bit to the buffer
        self.buffer.push_bit(bit);
    }// write_bit
    
}