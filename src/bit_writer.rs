use super::bit_buffer::BitBuffer;

/// A BitWriter holds a buffer used to store the individual bits being written
/// to it from the user.
#[derive(Default)]
pub struct BitWriter {
    /// The 'position' field represents which bit in the buffer is currently
    /// being indexed
    position: u128,

    /// The 'buffer' holds the actual bit buffer (represented by a BitBuffer)
    /// that is being read from
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
            buffer: BitBuffer::new(None, None),
        }
    } // new

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
    } // write_bit

    /// Writes the given value in the bit buffer following the previous written value.
    ///
    /// # Arguments
    ///
    /// * `bits` (u128) - The bits to write to this BitWriter
    /// * `len` (u8) - The number of bits to write
    ///
    /// # Examples
    /// ```
    /// use bit_buffers::BitWriter;
    /// let mut bit_writer = BitWriter::new();
    /// bit_writer.write_bits(0x42, 16);
    /// ```
    pub fn write_bits(&mut self, bits: u128, len: u8) {
        self.position += len as u128;

        self.buffer.push_bits(bits, len);
    }

    /// Get the underlying bit buffer
    ///
    /// # Examples
    /// ```
    /// use bit_buffers::BitWriter;
    /// use bit_buffers::bit_buffer::BitBuffer;
    /// let mut bit_writer = BitWriter::new();
    /// bit_writer.write_bits(0x42, 16);
    /// let bit_buffer = bit_writer.get_buffer();
    /// ```
    pub fn get_buffer(&mut self) -> BitBuffer {
        self.buffer.clone()
    }
}
