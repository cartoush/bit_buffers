use bit_buffers::BitReader;
use bit_buffers::BitWriter;

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
    bit_writer.flush_to_file("bit_file.bit");

    // Here, we will attempt to read bits from the file to see if they were 
    // read in the same order they were pushed (101001011)
    let mut bit_reader = BitReader::new();
    let _result = bit_reader.load_from_file("bit_file.bit");
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