use yew::prelude::*;
use crate::text_details::TextDetails;
use web_sys::HtmlInputElement;

#[derive(PartialEq, Properties)]
pub struct StaticTextProps {
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
	#[prop_or(None)]
	pub x: Option<i32>,
	#[prop_or(None)]
	pub y: Option<i32>,
	pub text: TextDetails,
}

#[function_component(Text)]
pub fn static_text_component(props: &StaticTextProps) -> Html {
	let id = props.id.clone();
	let class = props.class.clone();
	let extra_style = props.style.clone();
	let width = props.width.clone();
	let height = props.height.clone();
	let left = props.x.clone();
	let top = props.y.clone();

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

	let style = format!("position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; {}",
		left.unwrap_or(0),
		top.unwrap_or(0),
		width.unwrap_or(100),
		height.unwrap_or(100),
		extra_style,
	);

	html! {
		<div {id} {class} {style}>
			if *selected && (*text_details).editable {
				<textarea value={ text.to_string() } style={format!("resize: none; overflow: hidden; width: 98%; height: 98%;{}", font_style)}
				{oninput} {onmouseenter} {onmouseleave} />
			} else {
				<div style={format!("width: 100%; height: 100%; background-color: {}", text_details.background_color)} {onmouseenter} {onmouseleave}>
					<span style={font_style}> { text.to_string() } </span>
				</div>
			}
			{ props.children.clone() }
		</div>
	}
}
