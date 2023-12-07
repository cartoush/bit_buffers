use bit_buffers::BitWriter;

fn main() {
    let mut bit_writer = BitWriter::new();

    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(1);
    bit_writer.write_bit(0);
    bit_writer.write_bit(1);

}