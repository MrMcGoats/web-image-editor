#[derive(Clone, PartialEq)]
pub struct FileDetails {
	pub name: String,
	pub file_type: String,
	pub data: Vec<u8>,
	pub width: i32,
	pub height: i32,
}
