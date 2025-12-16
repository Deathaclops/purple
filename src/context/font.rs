
use vello::peniko::Blob;

pub struct FontContext {
	pub context: parley::FontContext,
	pub layout: parley::LayoutContext,
	pub fonts: Vec<String>,
} // end struct FontContext

impl FontContext {
	pub fn new() -> Self {
		let context = parley::FontContext {
			collection: parley::fontique::Collection::new(parley::fontique::CollectionOptions { shared: true, system_fonts: false }),
			source_cache: parley::fontique::SourceCache::new(parley::fontique::SourceCacheOptions { shared: true }),
		}; let layout = parley::LayoutContext::new();
		return Self { context, layout, fonts: Vec::new() };
	} // end fn new
	pub fn load(&mut self, data: Vec<u8>) -> String {
		let blob = Blob::new(std::sync::Arc::new(data));
		let id = self.context.collection
		.register_fonts(blob.clone(), None);
		self.fonts.push(self.context.collection.family_name(id[0].0).unwrap().to_string());
		return self.context.collection.family_name(id[0].0).unwrap().to_string();
	} // end fn load
} // end impl FontContext