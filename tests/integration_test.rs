use bit_buffers::bit_reader;
use bit_buffers::bit_writer;
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

    // Here, we will attempt to read bits from the file to see if they were
    // read in the same order they were pushed (101001011)
    let mut bit_reader = BitReader::new(bit_writer.get_buffer());
    let bit_string = format!(
        "{}{}{}{}{}{}{}{}{}",
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
    let bit_string = format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}",
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

#[test]
fn read_middle_of_byte() {
    let mut bit_writer = BitWriter::new();

    bit_writer.write_bits(0b010100111000011111, 18);
    let mut bit_reader = BitReader::new(bit_writer.get_buffer());
    let bits = bit_reader.read_bits(10).unwrap();
    assert_eq!(bits, 0b0101001110);
    let bits = bit_reader.read_bits(4).unwrap();
    assert_eq!(bits, 0b0001);
    let bits = bit_reader.read_bits(4).unwrap();
    assert_eq!(bits, 0b1111);
}

#[test]
fn write_read_udp_packet() {
    let pkt: Vec<(u128, u8, &str)> = vec![
        (0x4, 4, "ip_version"), // 0100
        (0x5, 4, "header_len"), // 0101
        (0x00, 8, "service_type"), // 00000000
        (0x0048, 16, "total_len"), // 00000000 01110010
        (0x0ae8, 16, "identification"), // 00001010 11101000
        (0x2, 3, "flags"), // 010
        (0x0000, 13, "fragment_offset"), // 00000 00000000
        (0x40, 8, "ttl"), // 01000000
        (0x11, 8, "protocol"), // 00010001
        (0xacf1, 16, "header_checksum"), // 10101100 11110001
        (0xc0a8007c, 32, "src_addr"), // 11000000 10101000 00000000 01111100
        (0xc0a800ff, 32, "dest_addr"), // 11000000 10101000 00000000 11111111
        (0xe115, 16, "src_port"), // 11100001 00010101
        (0xe115, 16, "dst_port"), // 11100001 00010101
        (0x0034, 16, "udp_len"), // 00000000 00110100
        (0x0dfa, 16, "udp_checksum"), // 00001101 11111010
    ];

    let mut bit_writer = BitWriter::new();
    for field in &pkt {
        bit_writer.write_bits(field.0, field.1);
    }

    let mut bit_reader = BitReader::new(bit_writer.get_buffer());

    for field in &pkt {
        let bits = bit_reader.read_bits(field.1).unwrap();
        println!("field: {}, size: {} expected: {:#x}, read: {:#x}", field.2, field.1, field.0, bits);
        assert_eq!(bits, field.0);
    }
}
