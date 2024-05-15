use std::collections::HashMap;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use yew::prelude::*;
use web_sys::{MouseEvent, console, HtmlElement, HtmlInputElement, FileList};
use gloo::file::File;
use gloo::file::callbacks::FileReader;

#[derive(PartialEq, Properties)]
struct Props {
	#[prop_or_default]
	id: AttrValue,
	#[prop_or_default]
	class: AttrValue,
	#[prop_or_default]
	style: AttrValue,
	#[prop_or_default]
	children: Children,
	#[prop_or(None)]
	width: Option<i32>,
	#[prop_or(None)]
	height: Option<i32>,
	file: FileDetails,
}

#[derive(PartialEq, Properties)]
struct CanvasProps {
	#[prop_or_default]
	id: AttrValue,
	#[prop_or_default]
	class: AttrValue,
	#[prop_or_default]
	style: AttrValue,
	#[prop_or_default]
	children: Children,
	#[prop_or(None)]
	width: Option<i32>,
	#[prop_or(None)]
	height: Option<i32>,
}

#[derive(Clone, PartialEq)]
struct FileDetails {
	name: String,
	file_type: String,
	data: Vec<u8>,
	width: i32,
	height: i32,
}

pub enum Msg {
	Loaded(String, String, Vec<u8>, i32, i32),
	Files(Vec<File>)
}

pub struct App {
	readers: HashMap<String, FileReader>,
	files: Vec<FileDetails>,
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			readers: HashMap::default(),
			files: Vec::default(),
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
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
			<div>
				<input id="file-upload" type="file" accept="image/*" multiple={true}
					onchange={ctx.link().callback(move |e: Event| {
						let input: HtmlInputElement = e.target_unchecked_into();
						Self::upload_files(input.files())
					})} />
					<EditableCanvas id="photo-canvas" style="border: 1px solid black;" >
						{ for self.files.iter().rev().map(App::view_file) }
					</EditableCanvas>
			</div>
		}
	}
}

impl App {
	fn view_file(file: &FileDetails) -> Html {
		

		html! {
			<MouseMoveComponent file={file.clone()} id={ format!("phote-move-{}", file.name.clone()) } width=250 style="border: 1px solid black;" />
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
}


#[function_component]
fn MouseMoveComponent(props: &Props) -> Html {
	let div_node_ref = use_node_ref();

	let id = props.id.clone();
	let extra_style = props.style.clone();
	let file = props.file.clone();

	let first_load = use_state(|| true);

	let bgstyle = format!(
		"background: url({}); background-position: center; background-size: 100% 100%; background-repeat: no-repeat",
		format!(
			"data:{};base64,{}",
			file.clone().file_type,
			STANDARD.encode(&file.clone().data)
		)
	);
	
	let width = use_state(|| 0);
	let height = use_state(|| 0);

	if *first_load {
		// Need these because we can't check the new value of width and height. They'll always read
		// as 0
		let mut tmp_width: i32 = 0;
		let mut tmp_height: i32 = 0;
		// Calculate size of the div based on what was passed in
		if let Some(passed_width) = props.width {
			tmp_width = passed_width;
		}

		if let Some(passed_height) = props.height {
			tmp_height = passed_height;
		}


		// Now if the width and height are 0, set them to the actual image size
		if tmp_width == 0 && tmp_height == 0 {
			tmp_width = file.width;
			tmp_height = file.height;
		} else if tmp_width == 0 {
			// We know tmp_height is not 0 because if it was the above if statement would have been true
			// Set width to maintain aspect ratio
			let aspect_ratio = file.width as f32 / file.height as f32;
			let new_width = (tmp_height as f32 * aspect_ratio) as i32;
			tmp_width = new_width;
		} else if tmp_height == 0 {
			// Set height to maintain aspect ratio
			let aspect_ratio = file.width as f32 / file.height as f32;
			let new_height = (tmp_width as f32 / aspect_ratio) as i32;
			tmp_height = new_height;
		}


		console::log_1(&format!("Got dimensions {}x{}", tmp_width, tmp_height).into());
		width.set(tmp_width);
		height.set(tmp_height);

		first_load.set(false);
	}

	// Used to position the div
	let mousex = use_state(|| 0);
	let mousey = use_state(|| 0);

	// Saves where the mouse was clicked for resizing purposes
	let clickx = use_state(|| 0);
	let clicky = use_state(|| 0);

	let dragging = use_state(|| false);
	let resizing = use_state(|| false);

	let z_index = use_state(|| 1);
	let old_z_index = use_state(|| 1);

	let onmousemove = {
		let mousex = mousex.clone();
		let mousey = mousey.clone();

		let clickx = clickx.clone();
		let clicky = clicky.clone();

		let dragging = dragging.clone();
		let resizing = resizing.clone();

		let width = width.clone();
		let height = height.clone();

		let div_node_ref = div_node_ref.clone();
		move |event: MouseEvent| {
			if *dragging {
				let x = event.client_x();
				let y = event.client_y();

				// Cacluate mousex and mousey such that the mouse will be in the middle of the div
				let x1 = x - div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 / 2;

				let y1 = y - div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 / 2;

				mousex.set(x1);
				mousey.set(y1);
			} else if *resizing {
				// If we're in the corner, resize the div and maintain aspect ratio
				// If we're on an edge, resize in that direction
				let x = event.client_x();
				let y = event.client_y();

				// Calculate 2% of the width and height
				let width_2 = div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 * 2 / 100;
				let height_2 = div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 * 2 / 100;

				// A corner is a location within 10 pixels of two edges
				let mut corner = false;
				// Check all 4 corners
				if x > div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 - width_2 && y > div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 - height_2 {
					corner = true;
				} else if x < width_2 && y < height_2 {
					corner = true;
				} else if x > div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 - width_2 && y < height_2 {
					corner = true;
				} else if x < width_2 && y > div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 - height_2 {
					corner = true;
				}

				if corner {
					// Maintian aspect ratio
					
					let x_diff = x - *clickx;
					let y_diff = y - *clicky;

					let new_width = div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 + x_diff;
					let new_height = div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 + y_diff;

					let aspect_ratio = div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as f32 / div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as f32;

					if x_diff.abs() > y_diff.abs() {
						let new_height = (new_width as f32 / aspect_ratio) as i32;
						height.set(new_height);
						width.set(new_width);
					} else {
						let new_width = (new_height as f32 * aspect_ratio) as i32;
						width.set(new_width);
						height.set(new_height);
					}
				} else {
					// If we're near a top or bottom edge, only adjust height
					// If we're near a left or right edge, only adjust width
					if x < width_2  {
						let x_diff = x - *clickx;
						let new_width = div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 - x_diff;
						width.set(new_width);

						// Move the div to the left
						let x1 = div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 + x_diff;
						mousex.set(x1);
					} else if x > div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 - width_2 {
						let x_diff = x - *clickx;
						let new_width = div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 + x_diff;
						width.set(new_width);
					} else if y < height_2 {
						let y_diff = y - *clicky;
						let new_height = div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 - y_diff;
						height.set(new_height);

						// Move the div up
						let y1 = div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 + y_diff;
						mousey.set(y1);
					} else if y > div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 - height_2 {
						let y_diff = y - *clicky;
						let new_height = div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 + y_diff;
						height.set(new_height);
					}
				}
			}
		}
	};

	let onmousedown = {
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		
		let clickx = clickx.clone();
		let clicky = clicky.clone();

		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		let div_node_ref = div_node_ref.clone();
		move |event: MouseEvent| {
			// Calculate 2% of the width and height
			let width_2 = div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 * 2 / 100;
			let height_2 = div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 * 2 / 100;

			// If mouse is near the side of the div resize, otherwise drag
			if false && (event.client_x() < width_2 || event.client_x() > div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 - width_2 || event.client_y() < height_2 || event.client_y() > div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 - height_2) { // TODO Remove first condition to reenable
				resizing.set(true);
				dragging.set(false);
			} else {
				dragging.set(true);
				resizing.set(false);
			}
			

			clickx.set(event.client_x());
			clicky.set(event.client_y());

			// TODO: Save previous z-index and set it back when mouseup
			// Can't figure out how to actually get the current z-index
			old_z_index.set(1);
			z_index.set(1000);
		}
	};

	let onmouseup = {
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		move |_: MouseEvent| {
			dragging.set(false);
			resizing.set(false);

			z_index.set(*old_z_index);
		}
	};

	let onmouseleave = {
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		move |_: MouseEvent| {
			dragging.set(false);
			resizing.set(false);

			z_index.set(*old_z_index);
		}
	};

	let onmouseenter = {
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		move |_: MouseEvent| {
			dragging.set(false);
			resizing.set(false);

			z_index.set(*old_z_index);
		}
	};

	html! {
		<div ref={div_node_ref} {onmousemove} {onmousedown} {onmouseup} {id} {onmouseenter} {onmouseleave} style={format!("position: relative; left: {}px; top: {}px; z-index: {}; width: {}px; max-width: {}px; height: {}px; max-height: {}px; {}; {}", *mousex, *mousey, *z_index, *width, *height, *width, *height, bgstyle, extra_style)} >
			{ props.children.clone() }
		</div>
	}
}

#[function_component]
fn EditableCanvas(props: &CanvasProps) -> Html {
	let canvas_node_ref = use_node_ref();

	let id = props.id.clone();
	let extra_style = props.style.clone();

	let width = props.width.unwrap_or(800);
	let height = props.height.unwrap_or(800);

	html! {
		<div ref={canvas_node_ref} {id} style={format!("width: {}px; height: {}px; max-width: {}px; max-height: {}px; overflow: hidden; {}", width, height, width, height, extra_style)} >
			{ props.children.clone() }
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
