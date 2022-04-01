pub enum Size {
	/// 2x2 bitmask
	Small,
	/// 3x3 bitmask
	Full,
}

pub const TOP_LEFT: u8 = 0;
pub const TOP: u8 = 1;
pub const TOP_RIGHT: u8 = 2;
pub const LEFT: u8 = 3;
pub const RIGHT: u8 = 4;
pub const BOTTOM_LEFT: u8 = 5;
pub const BOTTOM: u8 = 6;
pub const BOTTOM_RIGHT: u8 = 7;
pub const POSSIBLE_COMBINATIONS_2X2: usize = 16;
pub const POSSIBLE_COMBINATIONS_3X3: usize = 256;

pub fn fill_bit(
	byte: &mut u8,
	position: u8,
) {
	*byte |= 1 << position;
}

pub fn clear_bit(
	byte: &mut u8,
	position: u8,
) {
	*byte &= !(1 << position);
}

pub fn bit_at(
	byte: &u8,
	position: u8,
) -> bool {
	*byte & (1 << position) > 0
}
