#[path = "text_details.rs"]
pub mod text_details;
#[path = "generic_movable_div.rs"]
mod generic_movable_div;

use yew::prelude::*;
use text_details::TextDetails;
use generic_movable_div::MouseMoveComponent;
use web_sys::{HtmlInputElement,console};

#[derive(PartialEq, Properties)]
pub struct MovableTextProps {
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
	pub text: TextDetails,
}

#[function_component]
pub fn MovableTextComponent(props: &MovableTextProps) -> Html {
	let id = props.id.clone();
	let class = props.class.clone();
	let style = props.style.clone();
	let text_details = use_state(|| props.text.clone());
	let text = use_state(|| props.text.text.clone());

	let selected = use_state(|| false);

	let oninput = {
		let text = text.clone();
		move |e: InputEvent| {
			let input: HtmlInputElement = e.target_unchecked_into();
			text.set(input.value());
		}
	};

	let onmouseenter = {
		let selected = selected.clone();
		move |_: MouseEvent| {
			selected.set(true);
		}
	};

	let onmouseleave = {
		let selected = selected.clone();
		move |_: MouseEvent| {
			selected.set(false);
		}
	};

	let font_style = format!("background-color: {}; color: {}; font-size: {}px; font-family: {};",
		text_details.background_color,
		text_details.font_color,
		text_details.font_size,
		text_details.font_family,
	);

	html! {
		<MouseMoveComponent {id} {class} {style}>
			if *selected {
				<textarea value={ text.to_string() } style={format!("resize: none; overflow: hidden; width: 98%; height: 98%;{}", font_style)}
				{oninput} {onmouseenter} {onmouseleave} />
			} else {
				<div style={format!("width: 100%; height: 100%; background-color: {}", text_details.background_color)} {onmouseenter} {onmouseleave}>
					<span style={font_style}> { text.to_string() } </span>
				</div>
			}
			{ props.children.clone() }
		</MouseMoveComponent>
	}
}
