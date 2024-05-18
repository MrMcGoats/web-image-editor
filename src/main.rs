use std::collections::HashMap;

use yew::prelude::*;
use web_sys::{console, HtmlInputElement, FileList};
use gloo::file::File;
use gloo::file::callbacks::FileReader;
use wasm_bindgen::prelude::*;

mod editable_canvas_div;
mod image_movable_div;
mod textbox_movable_div;

use editable_canvas_div::*;
use image_movable_div::*;
use textbox_movable_div::*;
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


pub enum Msg {
	Loaded(String, String, Vec<u8>, i32, i32),
	Files(Vec<File>),
	Text(TextDetails),
	FinishedLoading,
}

pub struct App {
	readers: HashMap<String, FileReader>,
	files: Vec<FileDetails>,
	text: Vec<TextDetails>,
	first_load: bool,
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			readers: HashMap::default(),
			files: Vec::default(),
			text: Vec::default(),
			first_load: true,
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Loaded(file_name, file_type, data, width, height) => {
				self.files.push(FileDetails {
					name: file_name.clone(),
					file_type,
					data,
					width,
					height,
				});
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
				self.text.push(text);
				true
			}
			Msg::FinishedLoading => {
				self.first_load = false;
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		if self.first_load {
			let items = parse_query();
			for item in items {
				if let Some(text) = item.text {
					ctx.link().send_message(Msg::Text(text));
				}
				if let Some(file) = item.file {
					ctx.link().send_message(Msg::Loaded(file.name, file.file_type, file.data, file.width, file.height));
				}
			}
			ctx.link().send_message(Msg::FinishedLoading);
		}

		html! {
			<div>
				<input id="file-upload" type="file" accept="image/*" multiple={true}
					onchange={ctx.link().callback(move |e: Event| {
						let input: HtmlInputElement = e.target_unchecked_into();
						Self::upload_files(input.files())
				})} />
					<button onclick={ctx.link().callback(|_| {
						Self::add_text(TextDetailsBuilder::default().text("Hello, World!".to_string()).font_size(16).build().unwrap())
					})}>{"Add Text"}</button>
				<button onclick={|_| capture_div("#photo-canvas")}>{"Save"}</button>
				<EditableCanvas id="photo-canvas">
					{ for self.files.iter().rev().map(App::view_file) }
					{ for self.text.iter().rev().map(App::view_text) }
				</EditableCanvas>
			</div>
		}
	}
}

impl App {
	fn view_file(file: &FileDetails) -> Html {
		html! {
			<MovableImageComponent file={file.clone()} id={ format!("phote-move-{}", file.name.clone()) } class="image" width=250 />
		}
	}

	fn view_text(text: &TextDetails) -> Html {
		html! {
			<MovableTextComponent text={text.clone()} id="text-move" class="text" width=250 style="border: 2px solid black;" />
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

#[derive(PartialEq, Clone)]
pub struct PageItems {
	pub text: Option<TextDetails>,
	pub file: Option<FileDetails>,
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
	pub movable: bool,
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
	// font_size (text only)
	// font_family (text only)
	// font_color (text only)
	// background_color (text only)
	// name (image only)
	
	// Each one will be an array. Corresponding values will be at the same index. If an attribute is
	// only for text or image, adjust the length of the array accordingly (add empty strings for the other type)
	let types = get_query_param("type");
	let values = get_query_param("value");
	let x_values = get_query_param("x");
	let y_values = get_query_param("y");
	let width_values = get_query_param("width");
	let height_values = get_query_param("height");
	let movable_values = get_query_param("movable");
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
		let width: i32;
		let height: i32;
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
			width = width_values[i].parse::<i32>().unwrap_or(100);
		} else {
			width = 100;
		}

		if height_values.len() > i {
			height = height_values[i].parse::<i32>().unwrap_or(100);
		} else {
			height = 100;
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

			text = Some(TextDetailsBuilder::default()
				.text(value)
				.font_size(font_size)
				.font_family(font_family)
				.font_color(font_color)
				.background_color(background_color)
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

			file = Some(FileDetails {
				name: name,
				file_type: "image/png".to_string(),
				data: base64::decode(value).unwrap_or(Vec::new()),
				width,
				height,
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


fn main() {
	yew::Renderer::<App>::new().render();
}
