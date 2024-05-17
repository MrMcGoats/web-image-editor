// Contains the base editable canvas div component
// This basically does nothing but contain other divs and not allow its children to overflow or change the size of the canvas div
// Note that this is just a div, not an actual HTML <cavas>
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CanvasProps {
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
}

#[function_component]
pub fn EditableCanvas(props: &CanvasProps) -> Html {
	let canvas_node_ref = use_node_ref();

	let id = props.id.clone();
	let extra_style = props.style.clone();

	let width = props.width.unwrap_or(800);
	let height = props.height.unwrap_or(800);

	html! {
		<div ref={canvas_node_ref} {id} style={format!("position: absolute; width: {}px; height: {}px; max-width: {}px; max-height: {}px; overflow: hidden; {}", width, height, width, height, extra_style)} >
			{ props.children.clone() }
		</div>
	}
}
