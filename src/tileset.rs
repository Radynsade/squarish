use std::str;

pub struct TileSet {
	/// 256 bytes (64 unicode chars)
	pub name: String,
	pub frame_size: u8,
	pub image_width: u16,
	pub image_height: u16,
}

impl TileSet {
	pub fn from_bytes(bytes: [u8; 263]) -> Self {
		let name_bytes = &bytes[0..256];

		let name = match str::from_utf8(name_bytes) {
			Ok(value) => value.to_owned(),
			Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
		};

		let frame_size = bytes[256];

		let image_width_bytes = &bytes[257..259];
		let image_width = ((image_width_bytes[0] as u16) << 8) | image_width_bytes[1] as u16;

		let image_height_bytes = &bytes[259..261];
		let image_height = ((image_height_bytes[0] as u16) << 8) | image_height_bytes[1] as u16;

        TileSet{
            name,
            frame_size,
            image_width,
            image_height
        }
	}

    pub fn to_bytes(&self) -> [u8; 263] {
        let mut bytes: [u8; 263] = [0; 263];
        let name_bytes = self.name.as_bytes();
    }
}
