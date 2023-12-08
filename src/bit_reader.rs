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
    /// use bit_buffers::bit_buffer::BitBuffer;
    /// use bit_buffers::BitReader;
    /// let _bit_reader = BitReader::new(BitBuffer::new(None, None));
    /// ```
    pub fn new(buffer: BitBuffer) -> BitReader {
        BitReader {
            position: 0,
            buffer: buffer,
        }
    } // new

    /// Reads the current bit being referenced by this BitReader.
    ///
    /// # Examples
    /// ```
    /// use bit_buffers::bit_buffer::BitBuffer;
    /// use bit_buffers::BitReader;
    /// let mut bit_reader = BitReader::new(BitBuffer::new(None, None));
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
    } // read_bit

    pub fn read_bits(&mut self, len: u8) -> Option<u128> {
        if self.position >= self.buffer.get_count() {
            return None;
        }

        let bits = self.buffer.get_bits(self.position, len);
        if bits != None {
            self.position += len as u128;
        }
        bits
    }
}
