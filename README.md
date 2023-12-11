# bit_buffers

The `bit_buffers` library allows for the reading and writing of individual bits to and from bit buffers.

[![doc tests](https://img.shields.io/badge/doc%20tests-passing-green.svg)](https://crates.io/crates/bit_buffers)
[![integration tests](https://img.shields.io/badge/integration%20tests-passing-green.svg)](https://crates.io/crates/bit_buffers)

This library offers prototype versions of a bit reading entity and a bit writing entity that both keep track of how many bits are contained within them, and at which bit they are currently using.

## **Example Usage** (writing bits to a file)
If you want to save a sequence of bits to a file, use a BitWriter.

```
// main.rs
use bit_buffers::BitWriter;

fn main() {

    // Create the writer
    let mut bit_writer = BitWriter::new();

    // Write out the desired bits
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
}
```

## **Future Goals**

### - Thread Safety
As of now, this library has not been tested in a multi-threaded application. Neither BitReader nor BitWriter are guaranteed to be thread safe at this time.

### - Sharability
When reading and writing bits from and to files, the user's system determines the endianness. This may lead to problems when trying to share bit files across a network. A future goal is to standardize how the number of bits are written to and read from such files.

### - Optimization
When bits are written to a BitWriter, they are written directly to an internal vector of bytes. The code as of now is messy and error-prone. Adding an additional buffer may simplify the underlying logic.

Additionally, for every 8 bits of data written, a re-allocation may need to take place. Adding features to allow users to reserve an expected amount of space may amortize the cost of reading and writing bits over time.

### - Additional Features
Currently, there is no way for a user to index into the buffer to read specific bits or overwrite bits.