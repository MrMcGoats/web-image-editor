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
}


pub enum Msg {
	Loaded(String, String, Vec<u8>, i32, i32),
	Files(Vec<File>),
	Text(TextDetails),
}

pub struct App {
	readers: HashMap<String, FileReader>,
	files: Vec<FileDetails>,
	text: Vec<TextDetails>,
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			readers: HashMap::default(),
			files: Vec::default(),
			text: Vec::default(),
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
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		// Add some fake text for now
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


fn main() {
	yew::Renderer::<App>::new().render();
}
