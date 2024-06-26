use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Builder, Serialize, Deserialize, Debug)]
pub struct TextDetails {
	#[builder(default)]
	pub text: String,
	#[builder(default = "8")]
	pub font_size: u16,
	#[builder(default = "\"Arial\".to_string()")]
	pub font_family: String,
	#[builder(default = "\"black\".to_string()")]
	pub font_color: String,
	#[builder(default = "\"white\".to_string()")]
	pub background_color: String,
	#[builder(default = "true")]
	pub editable: bool,
}
