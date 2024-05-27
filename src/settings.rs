// Functions/structs that configure/contain the settings
use derive_builder::Builder;
use wasm_bindgen::prelude::*;
use crate::console;
use crate::page_items::PageItems;
use crate::text_details::TextDetailsBuilder;
use crate::file_details::FileDetails;

// Javascript functions
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = getParam)]
	fn get_query_param(param: &str) -> Vec<String>;
}

#[derive(PartialEq, Clone, Builder)]
pub struct CanvasSettings {
	#[builder(default)]
	pub width: Option<i32>,
	#[builder(default)]
	pub height: Option<i32>,
}

pub fn parse_query() -> Vec<PageItems> {
	// This function will be used to parse the query string
	// It will return a vector of PageItems which contains any text or file details to add to the
	// page, as well as where to place them, their size, and if they are movable
	let mut items = Vec::new();

	// Possible options:
	// type
	// value (either text, or base64 encoded image)
	// x
	// y
	// width
	// height
	// movable
	// editable (text only)
	// font_size (text only)
	// font_family (text only)
	// font_color (text only)
	// background_color (text only)
	// name (image only)
	// real_width (image only)
	// real_height (image only)
	
	// Each one will be an array. Corresponding values will be at the same index. If an attribute is
	// only for text or image, adjust the length of the array accordingly (add empty strings for the other type)
	let types = get_query_param("type");
	let values = get_query_param("value");
	let x_values = get_query_param("x");
	let y_values = get_query_param("y");
	let width_values = get_query_param("width");
	let height_values = get_query_param("height");
	let movable_values = get_query_param("movable");
	let editable_values = get_query_param("editable");
	let font_size_values = get_query_param("font_size");
	let font_family_values = get_query_param("font_family");
	let font_color_values = get_query_param("font_color");
	let background_color_values = get_query_param("background_color");
	let name_values = get_query_param("name");

	let mut i = 0;
	let mut text_i = 0; // Used for text only attributes
	let mut image_i = 0; // Used for image only attributes
	
	while i < types.len() {
		let mut text = None;
		let mut file = None;
		
		let x: i32;
		let y: i32;
		let width: Option<i32>;
		let height: Option<i32>;
		let movable: bool;

		if x_values.len() > i {
			x = x_values[i].parse::<i32>().unwrap_or(0);
		} else {
			x = 0;
		}

		if y_values.len() > i {
			y = y_values[i].parse::<i32>().unwrap_or(0);
		} else {
			y = 0;
		}

		if width_values.len() > i {
			width = match width_values[i].parse::<i32>() {
				Ok(val) => Some(val),
				Err(_) => None,
			};
		} else {
			width = None;
		}

		if height_values.len() > i {
			height = match height_values[i].parse::<i32>() {
				Ok(val) => Some(val),
				Err(_) => None,
			};
		} else {
			height = None;
		}

		if movable_values.len() > i {
			movable = movable_values[i].parse::<bool>().unwrap_or(false);
		} else {
			movable = false;
		}

		if types[i] == "text" {
			let value: String;
			let font_size: u16;
			let font_family: String;
			let font_color: String;
			let background_color: String;
			let editable: bool;

			let default_text = TextDetailsBuilder::default().build().unwrap(); // To use as default values

			if values.len() > text_i {
				value = values[text_i].clone();
			} else {
				value = default_text.text;
			}

			if font_size_values.len() > text_i {
				font_size = font_size_values[text_i].parse::<u16>().unwrap_or(default_text.font_size);
			} else {
				font_size = default_text.font_size;
			}

			if font_family_values.len() > text_i {
				font_family = font_family_values[text_i].clone();
			} else {
				font_family = default_text.font_family;
			}

			if font_color_values.len() > text_i {
				font_color = font_color_values[text_i].clone();
			} else {
				font_color = default_text.font_color;
			}

			if background_color_values.len() > text_i {
				background_color = background_color_values[text_i].clone();
			} else {
				background_color = default_text.background_color;
			}

			if editable_values.len() > text_i {
				editable = editable_values[text_i].parse::<bool>().unwrap_or(false);
			} else {
				editable = false;
			}

			text = Some(TextDetailsBuilder::default()
				.text(value)
				.font_size(font_size)
				.font_family(font_family)
				.font_color(font_color)
				.background_color(background_color)
				.editable(editable)
				.build().unwrap()
			);

			text_i += 1;
		} else if types[i] == "image" {
			let value: String;
			let name: String;

			if values.len() > image_i {
				value = values[image_i].clone();
			} else {
				value = String::new();
			}

			if name_values.len() > image_i {
				name = name_values[image_i].clone();
			} else {
				name = String::new();
			}

			// Get width and height of image
			let resolution_res = imagesize::blob_size(&base64::decode(value.clone()).unwrap_or(Vec::new()));
			let mut real_width = 100;
			let mut real_height = 100;

			if let Ok(resolution) = resolution_res {
				real_width = resolution.width as i32;
				real_height = resolution.height as i32;
			} else {
				console::error_1(&format!("Failed to get resolution of image: {}. Using default", name).into());
			}

			file = Some(FileDetails {
				name,
				file_type: "image/png".to_string(),
				data: base64::decode(value).unwrap_or(Vec::new()),
				width: real_width,
				height: real_height,
			});
			image_i += 1;
		}

		items.push(PageItems {
			text,
			file,
			x,
			y,
			width,
			height,
			movable,
		});
		i += 1;
	}


	return items;
}

pub fn parse_settings_query() -> CanvasSettings {
	let width = get_query_param("canvas_width");
	let height = get_query_param("canvas_height");

	let width = match width.get(0).unwrap_or(&String::new()).parse::<i32>() {
		Ok(val) => Some(val),
		Err(_) => None,
	};
	let height = match height.get(0).unwrap_or(&String::new()).parse::<i32>() {
		Ok(val) => Some(val),
		Err(_) => None,
	};

	CanvasSettings {
		width,
		height,
	}
}
