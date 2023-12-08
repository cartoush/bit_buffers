use bit_buffers::BitReader;
use bit_buffers::BitWriter;
use bit_buffers::bit_reader;
use bit_buffers::bit_writer;

#[test]
fn write_and_read() {
    // Here, we attempt to write bits to a file
    let mut bit_writer = BitWriter::new();
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);

    // Here, we will attempt to read bits from the file to see if they were 
    // read in the same order they were pushed (101001011)
    let mut bit_reader = BitReader::new(bit_writer.get_buffer());
    let bit_string = format!("{}{}{}{}{}{}{}{}{}", 
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
    );
    assert_eq!(bit_string, "101001011");
}

#[test]
fn write_multiple_bits_and_read() {
    let mut bit_writer = BitWriter::new();

    bit_writer.write_bits(0b100111000011, 12);

    let mut bit_reader = BitReader::new(bit_writer.get_buffer());
    let bit_string = format!("{}{}{}{}{}{}{}{}{}{}{}{}",
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
            bit_reader.read_bit().unwrap(),
    );
    assert_eq!(bit_string, "100111000011");
}

#[test]
fn write_bits_and_read_multiple() {
    let mut bit_writer = BitWriter::new();

    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(0);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(0);

    let mut bit_reader = BitReader::new(bit_writer.get_buffer());
    let bits = bit_reader.read_bits(12).unwrap();
    assert_eq!(bits, 0b011000111100);
}

#[test]
fn read_overlapping() {
    let mut bit_writer = BitWriter::new();

    bit_writer.write_bits(0b0101, 4);
    bit_writer.write_bits(0b11001100110011, 14);
    let mut bit_reader = BitReader::new(bit_writer.get_buffer());
    let bits = bit_reader.read_bits(4).unwrap();
    assert_eq!(bits, 0b0101);
    let bits = bit_reader.read_bits(14).unwrap();
    assert_eq!(bits, 0b11001100110011);
}