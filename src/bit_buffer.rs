use std::cmp::min;

/// We need to know how many bits are in each u8 (byte) so we can index into the
/// byte buffer and work on the appropriate bit inside the desired byte
const BITS_PER_BYTE: u128 = 8;

/// BitBuffer
///
/// A BitBuffer stores a vector of bytes (u8) that can be transferred to and
/// from files, and can be queried. It tracks the total number of bits written
/// to it, as well as the current position (in terms of bits)
#[derive(Default, Clone, Debug)]
pub struct BitBuffer {
    count: u128,
    buffer: Vec<u8>,
} // pub struct BitBuffer

impl BitBuffer {
    /// Creates a new instance of a BitBuffer, with 0 bits being contained
    /// within, and with an empty `Vec<u8>`
    pub fn new(count: Option<u128>, buffer: Option<Vec<u8>>) -> BitBuffer {
        BitBuffer {
            count: count.unwrap_or(0),
            buffer: buffer.unwrap_or(Vec::new()),
        }
    } // new

    /// Clears out the contents of the vector without affecting capacity, and
    /// resets the count and position to 0, essentially resetting this buffer
    /// back to a new instance
    pub fn flush(&mut self) {
        // Reset the count and position to 0 since all bits in this buffer will
        // be deleted
        self.count = 0;

        // Clear out the vector's contents
        self.buffer.clear();
    } // flush

    /// Returns the bit at the indicated position
    pub fn get_bit(&self, index: u128) -> u8 {
        // Extract the bit at the current position within the buffer
        let byte_index = (index / BITS_PER_BYTE) as usize;
        let bit_index = index % BITS_PER_BYTE;
        let bit = (self.buffer[byte_index] & (1 << (BITS_PER_BYTE - 1 - bit_index))) as u8;

        // Return the bit
        bit >> (BITS_PER_BYTE - 1 - bit_index)
    } // get_bit

    /// Returns up to 128 bits from the indicated position
    pub fn get_bits(&self, index: u128, len: u8) -> Option<u128> {
        if len > 128 || self.count == 0 || self.count - index < len as u128 {
            return None;
        } else if len == 1 {
            return Some(self.get_bit(index).into());
        } else if len == 0 {
            return Some(0);
        }

        let bit_index: u8 = (index % BITS_PER_BYTE) as u8;
        let mut byte_index = (index / BITS_PER_BYTE) as usize;
        let mut read: u8 = 0;
        let mut bits: u128 = 0;
        if bit_index != 0 {
            let mut mask = 1;
            read = min(BITS_PER_BYTE as u8 - bit_index, len.into());
            for _ in 1..read {
                mask = mask << 1;
                mask |= 1;
            }
            mask = mask << (BITS_PER_BYTE as u8 - bit_index - read);

            bits = (self.buffer[byte_index] & mask) as u128
                >> (BITS_PER_BYTE as u8 - bit_index - read);
            if read + bit_index == BITS_PER_BYTE as u8 {
                byte_index += 1;
            }
        }

        let mut len_left_bits: u8 = len - read;
        if len_left_bits == 0 {
            return Some(bits);
        }

        let len_left_bytes: u8 = len_left_bits / BITS_PER_BYTE as u8;
        for _ in 0..len_left_bytes {
            bits = bits << BITS_PER_BYTE;
            bits |= self.buffer[byte_index] as u128;
            byte_index += 1;
        }

        len_left_bits = len_left_bits % BITS_PER_BYTE as u8;
        if len_left_bits != 0 {
            let mut mask = 1;
            for _ in 1..len_left_bits {
                mask = mask << 1;
                mask |= 1;
            }
            mask = mask << BITS_PER_BYTE as u8 - len_left_bits;
            bits = bits << len_left_bits;
            bits |=
                ((self.buffer[byte_index] & mask) >> (BITS_PER_BYTE as u8 - len_left_bits)) as u128;
        }
        Some(bits)
    }

    /// Pushes the given bit to the end of the buffer
    pub fn push_bit(&mut self, bit: u8) {
        // If we hit a byte boundary, push another byte to the vector
        let bit_index = self.count % BITS_PER_BYTE;
        if bit_index == 0 {
            self.buffer.push(0);
        }

        // We need to set a bit in the last byte of the vector if the given
        // bit is 1; otherwise, we just need to advance the count
        if bit == 1 {
            let mask = (1 << (BITS_PER_BYTE - 1 - bit_index)) as u8;
            let byte_index = self.buffer.len() - 1;
            self.buffer[byte_index] |= mask;
        }

        // Make sure we increment the number of bits written
        self.count += 1;
    } // push_bit

    /// Pushes up to 128 bits to the end of the buffer
    pub fn push_bits(&mut self, bits: u128, len: u8) {
        if len > 128 || len == 0 {
            return;
        } else if len == 1 {
            self.push_bit(bits as u8);
        }

        let bytes_needed = {
            let nb_bits = (self.count + len as u128) - (self.buffer.len() as u128 * BITS_PER_BYTE);
            nb_bits / BITS_PER_BYTE + (nb_bits % BITS_PER_BYTE != 0) as u128
        };
        if bytes_needed != 0 {
            for _ in 0..bytes_needed {
                self.buffer.push(0);
            }
        }

        let bit_index: u8 = (self.count % BITS_PER_BYTE) as u8;
        let mut byte_index = (self.count / BITS_PER_BYTE) as usize;
        let mut written: u8 = 0;
        if bit_index != 0 {
            let mut mask: u8 = 1;
            written = min(BITS_PER_BYTE as u8 - bit_index, len);
            for _ in 1..written {
                mask = mask << 1;
                mask |= 1;
            }
            mask = mask << (BITS_PER_BYTE as u8 - bit_index - written);

            let byte_value: u8 = (bits >> (len - written)) as u8;
            self.buffer[byte_index] |= byte_value & mask;
            byte_index += 1;
        }

        let mut len_left_bits = len - written;
        if len_left_bits == 0 {
            self.count += len as u128;
            return;
        }

        let len_left_bytes = len_left_bits / BITS_PER_BYTE as u8;
        len_left_bits = len_left_bits % BITS_PER_BYTE as u8;
        for i in (0..len_left_bytes).rev() {
            let byte_value: u8 =
                (bits >> (i * BITS_PER_BYTE as u8 + (len % BITS_PER_BYTE as u8))) as u8;
            self.buffer[byte_index] = byte_value;
            byte_index += 1;
        }

        self.count += len as u128;
        if len_left_bits == 0 {
            return;
        }
        self.buffer[byte_index] = ((bits) << (BITS_PER_BYTE as u8 - len_left_bits)) as u8;
    }

    /// Get the current number of bits contained in this buffer
    pub fn get_count(&self) -> u128 {
        self.count
    }
}
