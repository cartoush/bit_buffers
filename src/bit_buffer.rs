use std::io;
use std::fs;

// We need to know how many bits are in each u8 (byte) so we can index into the 
// byte buffer and work on the appropriate bit inside the desired byte
const BITS_PER_BYTE: u128 = 8;

// BitBuffer
// 
// A BitBuffer stores a vector of bytes (u8) that can be transferred to and 
// from files, and can be queried. It tracks the total number of bits written 
// to it, as well as the current position (in terms of bits)
#[derive(Default)]
pub struct BitBuffer {
    count: u128,
    buffer: Vec<u8>,
}// pub struct BitBuffer

impl BitBuffer {
    
    // new
    // 
    // Creates a new instance of a BitBuffer, with 0 bits being contained 
    // within, and with an empty Vec<u8>
    pub fn new() -> BitBuffer {
        BitBuffer {
            count: 0,
            buffer: Vec::new(),
        }
    }// new
    
    // load_from_file
    // 
    // Load the bits from the specified file into this buffer
    pub fn load_from_file(&mut self, file_name: &str) -> io::Result<()> {
        // This buffer may be used to read from multiple files
        self.flush();
        
        // Read the contents of the file into into this buffer's vector
        let mut file_buffer = fs::read(file_name)?;
        self.buffer = file_buffer.split_off(16);
        
        // Extract the bit count from the first 16 bytes of the file vector
        let mut count_buffer: [u8; 16] = [0;16];
        count_buffer[..16].copy_from_slice(&file_buffer[..16]);

        self.count = u128::from_ne_bytes(count_buffer);
        
        Ok(())
    }// load_from_file
    
    // flush
    // 
    // Clears out the contents of the vector without affecting capacity, and 
    // resets the count and position to 0, essentially resetting this buffer 
    // back to a new instance
    pub fn flush(&mut self) {
        // Reset the count and position to 0 since all bits in this buffer will 
        // be deleted
        self.count = 0;
        
        // Clear out the vector's contents
        self.buffer.clear();
    }// flush
    
    // flush_to_file
    // 
    // Flushes this buffers bit count and bit buffer to a file for persistent 
    // storage
    pub fn flush_to_file(&mut self, file_name: &str) {
        // Collect the bit count and buffer contents into a single vector
        let mut file_vector: Vec<u8> = Vec::new();
        for byte in self.count.to_ne_bytes() {
            file_vector.push(byte);
        }
        file_vector.append(&mut self.buffer);
        
        // Create or truncate the specified file
        fs::write(file_name, file_vector).unwrap();
        
        // Flush this buffer to completely empty it
        self.flush();
    }// flush_to_file
    
    // get_bit
    // 
    // Returns the bit at the indicated position
    pub fn get_bit(&self, index: u128) -> u8 {
        // Extract the bit at the current position within the buffer
        let vector_index = (index / BITS_PER_BYTE) as usize;
        let bit_index = index % BITS_PER_BYTE;
        let bit = (self.buffer[vector_index] & (1 << bit_index)) as u8;
        
        // Return the bit
        bit >> bit_index
    }// get_bit
    
    // push_bit
    // 
    // Pushes the given bit to the end of the buffer
    pub fn push_bit(&mut self, bit: u8) {
        // If we hit a byte boundary, push another byte to the vector
        let bit_index = self.count % BITS_PER_BYTE;
        if bit_index == 0 {
            self.buffer.push(0);
        }
        
        // We need to set a bit in the last byte of the vector if the given 
        // bit is 1; otherwise, we just need to advance the count
        if bit == 1 {
            let mask = (1 << bit_index) as u8;
            let byte_index = self.buffer.len() - 1;
            self.buffer[byte_index] |= mask;
        }
        
        // Make sure we increment the number of bits written
        self.count += 1;
    }// push_bit
    
    // get_count
    // 
    // Get the current number of bits contained in this buffer
    pub fn get_count(&self) -> u128 { self.count }
    
}