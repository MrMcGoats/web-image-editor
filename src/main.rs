use std::collections::HashMap;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use yew::prelude::*;
use web_sys::{MouseEvent, console, HtmlElement, HtmlInputElement, FileList};
use gloo::file::File;
use gloo::file::callbacks::FileReader;

#[derive(PartialEq, Properties)]
struct Props {
	id: AttrValue,
	style: AttrValue,
	children: Children,
}

struct FileDetails {
	name: String,
	file_type: String,
	data: Vec<u8>,
}

pub enum Msg {
	Loaded(String, String, Vec<u8>),
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
			Msg::Loaded(file_name, file_type, data) => {
				self.files.push(FileDetails {
					name: file_name.clone(),
					file_type,
					data,
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
							link.send_message(Msg::Loaded(
									file_name,
									file_type,
									res.expect("failed to read file"),
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
					{ for self.files.iter().map(App::view_file) }
			</div>
		}
	}
}

impl App {
	fn view_file(file: &FileDetails) -> Html {
		let style = format!("background: url({}); background-position: center; background-size: 100% 100%; background-repeat: no-repeat; width: 100%; height: 100%", format!("data:{};base64,{}", file.file_type, STANDARD.encode(&file.data)));

		html! {
			<MouseMoveComponent id={ format!("phote-move-{}", file.name.clone()) } style="width: 100px; height: 100px; border: 1px solid black;">
				<div class="preview-media" {style} >
				</div>
			</MouseMoveComponent>
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

	let mousex = use_state(|| 0);
	let mousey = use_state(|| 0);

	let dragging = use_state(|| false);

	let z_index = use_state(|| 1);
	let old_z_index = use_state(|| 1);

	let onmousemove = {
		let mousex = mousex.clone();
		let mousey = mousey.clone();
		let dragging = dragging.clone();
		let div_node_ref = div_node_ref.clone();
		move |event: MouseEvent| {
			if !*dragging {
				return;
			}
			let x = event.client_x();
			let y = event.client_y();

			// Cacluate mousex and mousey such that the mouse will be in the middle of the div
			let x1 = x - div_node_ref.cast::<HtmlElement>().unwrap().offset_width() as i32 / 2;
			let y1 = y - div_node_ref.cast::<HtmlElement>().unwrap().offset_height() as i32 / 2;

			mousex.set(x1);
			mousey.set(y1);
		}
	};

	let onmousedown = {
		let dragging = dragging.clone();
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		move |_: MouseEvent| {
			dragging.set(true);
			
			// TODO: Save previous z-index and set it back when mouseup
			// Can't figure out how to actually get the current z-index
			old_z_index.set(1);
			z_index.set(1000);
		}
	};

	let onmouseup = {
		let dragging = dragging.clone();
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		move |_: MouseEvent| {
			dragging.set(false);

			z_index.set(*old_z_index);
		}
	};

	let onmouseleave = {
		let dragging = dragging.clone();
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		move |_: MouseEvent| {
			dragging.set(false);

			z_index.set(*old_z_index);
		}
	};

	let onmouseenter = {
		let dragging = dragging.clone();
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		move |_: MouseEvent| {
			dragging.set(false);

			z_index.set(*old_z_index);
		}
	};

	html! {
		<div ref={div_node_ref} {onmousemove} {onmousedown} {onmouseup} {id} {onmouseenter} {onmouseleave} style={format!("position: absolute; left: {}px; top: {}px; z-index: {}; {}", *mousex, *mousey, *z_index, extra_style)}>
			{ props.children.clone() }
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
