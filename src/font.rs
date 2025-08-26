use std::sync::{Arc, Mutex};

use cosmic_text::{fontdb::Database, Buffer, FontSystem, SwashCache};

lazy_static::lazy_static! {
	pub static ref FONT_SYSTEM: Mutex<FontSystem> = Mutex::new(FontSystem::new_with_locale_and_db("en-US".into(), Database::new()));
	pub static ref SWASH_CACHE: Mutex<SwashCache> = Mutex::new(SwashCache::new());
	pub static ref BUFFER: Mutex<Buffer> = Mutex::new(Buffer::new(&mut FONT_SYSTEM.lock().unwrap(), cosmic_text::Metrics::new(14.0, 20.0)));
} // end lazy_static

pub struct Font {
	pub id: cosmic_text::fontdb::ID,
	pub name: String,
	pub font: vello::peniko::Font,
} // end struct Font

impl Font {
	pub fn new(font_name: impl Into<String>, font_data: Vec<u8>) -> Self {
		FONT_SYSTEM.lock().unwrap().db_mut().load_font_data(font_data.to_vec());
		let font_name = font_name.into();
		let query = cosmic_text::fontdb::Query {
			families: &[cosmic_text::Family::Name(&font_name)],
			weight: cosmic_text::Weight::NORMAL,
			stretch: cosmic_text::fontdb::Stretch::Normal,
			style: cosmic_text::Style::Normal,
		}; let id = FONT_SYSTEM.lock().unwrap().db().query(&query).expect("Wrong font name given");
		return Self { name: font_name, id, font: vello::peniko::Font::new(vello::peniko::Blob::new(Arc::new(font_data)), 0) }
	} // end fn new
} // end impl Font

pub struct TextFormatting {
	pub size: f32,
	pub width: f32,
} // end struct TextFormatting