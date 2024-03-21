///
/// Padding! This is SHA-2 style padding
/// 

/**
 * Pads the remaining bits of a buffer given a start byte and bit (within that byte) and an ending byte
 * The end_byte is EXCLUDED
 * 
 * This treats each byte as being little endian in its bits
 */
pub fn pad(buf: &mut [u8], start_byte: usize, start_bit: usize, end_byte: usize) {
	buf[start_byte] |= 1 << start_bit;

	if start_byte == end_byte - 1 {
		return;
	}

	for i in (start_byte + 1)..end_byte {
		buf[i] = 0;
	}
}