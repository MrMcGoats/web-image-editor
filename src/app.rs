#[path = "editable_canvas_div.rs"]
mod editable_canvas_div;
#[path = "image_movable_div.rs"]
mod image_movable_div;
#[path = "image_static_div.rs"]
mod image_static_div;
#[path = "textbox_movable_div.rs"]
mod textbox_movable_div;
#[path = "textbox_static_div.rs"]
mod textbox_static_div;
#[path = "settings.rs"]
mod settings;

use std::collections::HashMap;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use gloo::file::File;
use gloo::file::callbacks::FileReader;
use web_sys::{console, HtmlInputElement, FileList};
use editable_canvas_div::*;
use image_movable_div::*;
use image_static_div::*;
use textbox_movable_div::*;
use textbox_static_div::*;
use crate::file_details::*;
use crate::text_details::*;
use crate::page_items::*;
use settings::*;

// Javascript functions
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = capture)]
	fn capture_div(id: &str);
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
					<button id="add-text-button" onclick={ctx.link().callback(|_| {
						Self::add_text(TextDetailsBuilder::default().text("Hello, World!".to_string()).font_size(16).build().unwrap())
					})}>{"Add Text"}</button>
				<button onclick={|_| capture_div("#photo-canvas")} id="save-button">{"Save"}</button>
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
