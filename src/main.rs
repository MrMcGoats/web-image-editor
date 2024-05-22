use std::collections::HashMap;

use yew::prelude::*;
use web_sys::{console, HtmlInputElement, FileList};
use gloo::file::File;
use gloo::file::callbacks::FileReader;
use wasm_bindgen::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;
use yew::context::ContextHandle;
use std::sync::Mutex;

mod editable_canvas_div;
mod image_movable_div;
mod image_static_div;
mod textbox_movable_div;
mod textbox_static_div;
pub mod file_details;
pub mod text_details;

use editable_canvas_div::*;
use image_movable_div::*;
use image_static_div::*;
use textbox_movable_div::*;
use textbox_static_div::*;
use file_details::*;
use text_details::*;	

// Javascript functions
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = capture)]
	fn capture_div(id: &str);
	#[wasm_bindgen(js_name = getParam)]
	fn get_query_param(param: &str) -> Vec<String>;
}


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

#[derive(PartialEq, Clone, Builder)]
pub struct CanvasSettings {
	#[builder(default)]
	pub width: Option<i32>,
	#[builder(default)]
	pub height: Option<i32>,
}

pub enum Msg {
	Loaded(String, String, Vec<u8>, i32, i32),
	Files(Vec<File>),
	Text(TextDetails),
	Item(PageItems),
	FinishedLoading,
	SetupCanvas(CanvasSettings),
}

pub struct App {
	readers: HashMap<String, FileReader>,
	items: Vec<PageItems>,
	first_load: bool,
	canvas_settings: CanvasSettings,
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			readers: HashMap::default(),
			items: Vec::default(),
			first_load: true,
			canvas_settings: CanvasSettingsBuilder::default().build().unwrap(),
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Loaded(file_name, file_type, data, width, height) => {
				let file_details = FileDetails {
					name: file_name.clone(),
					file_type,
					data,
					width,
					height,
				};

				self.items.push(PageItemsBuilder::default().file(Some(file_details)).width(Some(250)).build().unwrap());
				self.readers.remove(&file_name);
				true
			}
			Msg::Files(files) => {
				for file in files.into_iter() {
					let file_name = file.name();
					let file_type = file.raw_mime_type();

					let task = {
						let link = ctx.link().clone();
						let file_name = file_name.clone();

						gloo::file::callbacks::read_as_bytes(&file, move |res| {
							// Get width and height of image
							let data = res.expect("failed to read file");
							let resolution_res = imagesize::blob_size(&data);
							let mut width = 100;
							let mut height = 100;
							if let Ok(resolution) = resolution_res {
								width = resolution.width as i32;
								height = resolution.height as i32;
							} else {
								console::error_1(&format!("Failed to get resolution of image: {}. Using default", file_name).into());
							}

							console::log_1(&format!("Image size ({}): {}x{}", file_name, width, height).into());

							link.send_message(Msg::Loaded(
									file_name,
									file_type,
									data,
									width,
									height,
							  ))
						})
					};
					self.readers.insert(file_name, task);
				}
				true
			}
			Msg::Text(text) => {
				self.items.push(PageItemsBuilder::default().text(Some(text)).build().unwrap());
				true
			}
			Msg::Item(item) => {
				self.items.push(item);
				true
			}
			Msg::FinishedLoading => {
				self.first_load = false;
				true
			}
			Msg::SetupCanvas(settings) => {
				self.canvas_settings = settings;
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		if self.first_load {
			let items = parse_query();
			for item in items {
				ctx.link().send_message(Msg::Item(item));
			}

			// Parse settings query
			ctx.link().send_message(Msg::SetupCanvas(parse_settings_query()));
			ctx.link().send_message(Msg::FinishedLoading);
		}

		// Check if we have any new items to add to the page
		match web_sys::window().unwrap().document().unwrap().get_element_by_id("extra-data-div") {
			Some(extra_data_div) => {
				match extra_data_div.get_attribute("data-extra-data") {
					Some(data_str) => {
						if data_str != "" {
							// data_str is a comma separated list of items
							let data_arr_str = format!("[{}]", data_str);
							console::log_1(&data_arr_str.clone().into());
							let data: Vec<PageItems> = serde_json::from_str(&data_arr_str).unwrap();
							for item in data {
								ctx.link().send_message(Msg::Item(item));
							}
							extra_data_div.set_attribute("data-extra-data", "").unwrap();
						}
					}
					None => { console::log_1(&"No extra data".into()); }
				}
			}
			None => { console::log_1(&"No extra data div".into()); }
		}

		html! {
			<div>
				<div id="extra-data-div" style="display: none;"><button id="canvas-update-trigger" onclick={ctx.link().callback(|_| {
					// Trigger page redraw
					Msg::FinishedLoading
				})}>{"Update"}</button></div> // Used to store new items as
																						 // JsValues
				<input id="file-upload" type="file" accept="image/*" multiple={true}
					onchange={ctx.link().callback(move |e: Event| {
						let input: HtmlInputElement = e.target_unchecked_into();
						Self::upload_files(input.files())
				})} />
					<button onclick={ctx.link().callback(|_| {
						Self::add_text(TextDetailsBuilder::default().text("Hello, World!".to_string()).font_size(16).build().unwrap())
					})}>{"Add Text"}</button>
				<button onclick={|_| capture_div("#photo-canvas")}>{"Save"}</button>
				<EditableCanvas id="photo-canvas" width={self.canvas_settings.width} height={self.canvas_settings.height}>
					{ for self.items.iter().rev().map(App::view_item) }
				</EditableCanvas>
			</div>
		}
	}
}

impl App {
	fn view_file(file: &FileDetails, width: Option<i32>, height: Option<i32>, start_x: i32, start_y: i32, movable: bool) -> Html {
		html! {
			if movable {
				<MovableImageComponent file={file.clone()} id={ format!("phote-move-{}", file.name.clone()) } class="image" {width} {height} {start_x} {start_y} />
			} else {
				<Image file={file.clone()} id={ format!("phote-static-{}", file.name.clone()) } class="image" {width} {height} x={start_x} y={start_y} />
			}
		}
	}

	fn view_text(text: &TextDetails, width: Option<i32>, height: Option<i32>, start_x: i32, start_y: i32, movable: bool) -> Html {
		html! {
			if movable {
				<MovableTextComponent text={text.clone()} id="text-move" class="text" {width} {height} {start_x} {start_y} />
			} else {
				<Text text={text.clone()} id="text-static" class="text" {width} {height} x={start_x} y={start_y} />
			}
		}
	}

	fn view_item(item: &PageItems) -> Html {
		if let Some(file) = &item.file {
			Self::view_file(file, item.width, item.height, item.x, item.y, item.movable)
		} else if let Some(text) = &item.text {
			Self::view_text(text, item.width, item.height, item.x, item.y, item.movable)
		} else {
			html! {}
		}
	}

	fn upload_files(files: Option<FileList>) -> Msg {
		let mut result = Vec::new();

		if let Some(files) = files {
			let files = js_sys::try_iter(&files)
				.unwrap()
				.unwrap()
				.map(|v| web_sys::File::from(v.unwrap()))
				.map(File::from);
			result.extend(files);
		}
		Msg::Files(result)
	}

	fn add_text(text: TextDetails) -> Msg {
		Msg::Text(text)
	}
}

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

fn parse_settings_query() -> CanvasSettings {
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


fn main() {
	yew::Renderer::<App>::new().render();
}
