// Contains functions to be called from JavaScript
use wasm_bindgen::prelude::*;
use crate::file_details::*;
use crate::text_details::*;
use crate::page_items::PageItemsBuilder;
use crate::console;

// Function to construct a PageItems struct from javascript with default values for values not provided
#[wasm_bindgen(js_name = build_item)]
pub fn js_build_item(text_js: JsValue, file_js: JsValue, x: Option<i32>, y: Option<i32>, width: Option<i32>, height: Option<i32>, movable: Option<bool>) -> Result<JsValue, JsValue> {
	// Check if text is undefined	
	let text: Option<TextDetails> = match text_js.is_undefined() {
		true => None,
		false => match serde_wasm_bindgen::from_value(text_js) {
			Ok(val) => Some(val),
			Err(_) => return Err("Failed to parse text".into()),
		},
	};

	// Check if file is undefined
	let file: Option<FileDetails> = match file_js.is_undefined() {
		true => None,
		false => match serde_wasm_bindgen::from_value(file_js) {
			Ok(val) => Some(val),
			Err(_) => return Err("Failed to parse file".into()),
		},
	};

	// If neither or both text or file is None throw an error
	if text.is_none() && file.is_none() {
		return Err("Both text and file are None".into());
	} else if text.is_some() && file.is_some() {
		return Err("Both text and file are defined".into());
	}

	let item = PageItemsBuilder::default()
		.text(text)
		.file(file)
		.x(x.unwrap_or(0))
		.y(y.unwrap_or(0))
		.width(width)
		.height(height)
		.movable(movable.unwrap_or(true))
		.build()
		.map_err(|_| "Failed to build item")?;
	Ok(serde_wasm_bindgen::to_value(&item).map_err(|_| "Failed to serialize item")?)
}

#[wasm_bindgen(js_name = build_text)]
pub fn js_build_text(text: Option<String>, font_size: Option<u16>, font_family: Option<String>, font_color: Option<String>, background_color: Option<String>, editable: Option<bool>) -> Result<JsValue, JsValue> {
	// Save default text so we don't need to generate it for each attribute
	let default_text = TextDetailsBuilder::default().build().unwrap();
	let text = TextDetailsBuilder::default()
		.text(text.unwrap_or(default_text.text))
		.font_size(font_size.unwrap_or(default_text.font_size))
		.font_family(font_family.unwrap_or(default_text.font_family))
		.font_color(font_color.unwrap_or(default_text.font_color))
		.background_color(background_color.unwrap_or(default_text.background_color))
		.editable(editable.unwrap_or(default_text.editable))
		.build()
		.map_err(|_| "Failed to build text")?;
	Ok(serde_wasm_bindgen::to_value(&text).map_err(|_| "Failed to serialize text")?)
}

#[wasm_bindgen(js_name = build_file)]
pub fn js_build_file(name: String, file_type: String, data: Vec<u8>) -> Result<JsValue, JsValue> {
	// Get image's dimensions
	let resolution_res = imagesize::blob_size(&data);
	let mut width = 100;
	let mut height = 100;
	if let Ok(resolution) = resolution_res {
		width = resolution.width as i32;
		height = resolution.height as i32;
	} else {
		console::error_1(&format!("Failed to get resolution of image: {}. Using default", name).into());
	}

	let file = FileDetails {
		name,
		file_type,
		data,
		width,
		height,
	};
	Ok(serde_wasm_bindgen::to_value(&file).map_err(|_| "Failed to serialize file")?)
}
