#[path = "file_details.rs"]
pub mod file_details;
#[path = "generic_movable_div.rs"]
mod generic_movable_div;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use yew::prelude::*;
use file_details::FileDetails;
use generic_movable_div::MouseMoveComponent;

#[derive(PartialEq, Properties)]
pub struct MovableImageProps {
	#[prop_or_default]
	pub id: AttrValue,
	#[prop_or_default]
	pub class: AttrValue,
	#[prop_or_default]
	pub style: AttrValue,
	#[prop_or_default]
	pub children: Children,
	#[prop_or(None)]
	pub width: Option<i32>,
	#[prop_or(None)]
	pub height: Option<i32>,
	pub file: FileDetails,
}

#[function_component]
pub fn MovableImageComponent(props: &MovableImageProps) -> Html {
	let id = props.id.clone();
	let class = props.class.clone();
	let extra_style = props.style.clone();
	let file = props.file.clone();

	let first_load = use_state(|| true);

	let mut width = None;
	let mut height = None;

	// Calculate size of the div based on what was passed in and the image dimensions
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
			let new_height = (tmp_width as f32 * aspect_ratio) as i32;
			tmp_height = new_height;
		}

		width = Some(tmp_width);
		height = Some(tmp_height);

		first_load.set(false);
	}

	let style = format!(
		"background: url({}); background-position: center; background-size: 100% 100%; background-repeat: no-repeat; {}",
		format!(
			"data:{};base64,{}",
			file.clone().file_type,
			STANDARD.encode(&file.clone().data)
		),
		extra_style,
	);

	html! {
		<MouseMoveComponent {id} {class} {style} {width} {height}>
			{ props.children.clone() }
		</MouseMoveComponent>
	}
}
