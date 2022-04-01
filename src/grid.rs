use std::{
	io::{
		Read,
		BufReader,
		Result
	},
	fs::File
};

pub mod cell {
	pub type Cell = u32;

	pub fn get_tile_set_id(cell: Cell) -> u16 {
		(cell >> 16) as u16
	}

	pub fn get_tile_id(cell: Cell) -> u16 {
		cell as u16
	}
}

pub use cell::Cell;

pub type Grid = Vec<Vec<Cell>>;

pub fn from_bytes(bytes: Vec<u8>) {

}

pub fn load_from_file(
	source: &str,
	width: u16,
	height: u16
) -> Result<Grid> {
	let file = File::open(source)?;
	let mut reader = BufReader::new(file);
	let mut buffer: Vec<u8> = Vec::new();
	let mut grid: Grid = Vec::new();

	reader.read_to_end(&mut buffer)?;

	for x in 0..width {
		for y in 0..height {
			
		}
	}

	for i in (0..buffer.len()).step_by(2) {
		
	}

	return Ok(grid)
}
