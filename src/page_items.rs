use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::text_details::TextDetails;
use crate::file_details::FileDetails;

#[derive(PartialEq, Clone, Builder, Serialize, Deserialize)]
pub struct PageItems {
	#[builder(default)]
	pub text: Option<TextDetails>,
	#[builder(default)]
	pub file: Option<FileDetails>,
	#[builder(default = "0")]
	pub x: i32,
	#[builder(default = "0")]
	pub y: i32,
	#[builder(default)]
	pub width: Option<i32>,
	#[builder(default)]
	pub height: Option<i32>,
	#[builder(default = "true")]
	pub movable: bool,
}
