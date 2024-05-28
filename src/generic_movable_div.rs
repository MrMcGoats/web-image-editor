// Code for a generic div that can be moved around the screen by dragging it with the mouse
// Can also be resized

use web_sys::{MouseEvent, HtmlElement, console};
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_name = focusElement)]
	fn focus_element(element: &HtmlElement); // Need this because we can't set no scroll option in
														  // rust yet
}

#[derive(PartialEq, Properties)]
pub struct MouseMoveProps {
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
	pub start_x: Option<i32>,
	#[prop_or(None)]
	pub start_y: Option<i32>,
}


static DEFAULT_WIDTH: i32 = 250;
static DEFAULT_HEIGHT: i32 = 250;

#[function_component]
pub fn MouseMoveComponent(props: &MouseMoveProps) -> Html {
	let div_node_ref = use_node_ref();

	let id = props.id.clone();
	let class = props.class.clone();
	let extra_style = props.style.clone();

	let first_load = use_state(|| true);

	let width = use_state(|| 0);
	let height = use_state(|| 0);

	if *first_load {
		let mut tmp_width: i32 = DEFAULT_WIDTH;
		let mut tmp_height: i32 = DEFAULT_HEIGHT;
		// Calculate size of the div based on what was passed in
		if let Some(passed_width) = props.width {
			tmp_width = passed_width;
		}

		if let Some(passed_height) = props.height {
			tmp_height = passed_height;
		}

		width.set(tmp_width);
		height.set(tmp_height);

		first_load.set(false);
	}

	// Used to position the div
	let mousex = use_state(|| props.start_x.unwrap_or(0));
	let mousey = use_state(|| props.start_y.unwrap_or(0));

	// Saves where the mouse was clicked for resizing purposes
	let clickx = use_state(|| 0);
	let clicky = use_state(|| 0);

	// Bottom and right offsets to use with resizing, to keep the mouse under the edge
	let resize_start_x = use_state(|| 0);
	let resize_start_y = use_state(|| 0);
	let resize_start_width = use_state(|| 0);
	let resize_start_height = use_state(|| 0);

	// For dragging
	let drag_start_left = use_state(|| 0);
	let drag_start_top = use_state(|| 0);

	let dragging = use_state(|| false);
	let resizing = use_state(|| false);
	let on_edge = use_state(|| false);
	let on_corner = use_state(|| false);

	// This is used as basically a delete flag
	let hidden = use_state(|| false);

	let z_index = use_state(|| 1);
	let old_z_index = use_state(|| 1);

	let onkeydown = {
		let hidden = hidden.clone();
		move |event: KeyboardEvent| {
			if !*hidden && event.key() == "Delete" {
				hidden.set(true);
			}
		}
	};

	let onmousemove = {
		let mousex = mousex.clone();
		let mousey = mousey.clone();
		let clickx = clickx.clone();
		let clicky = clicky.clone();
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let on_edge = on_edge.clone();
		let drag_start_left = drag_start_left.clone();
		let drag_start_top = drag_start_top.clone();

		
		move |event: MouseEvent| {
			if !*dragging || *on_edge || *resizing {
				console::log_1(&format!("Mouse move: dragging:{}, on_edge:{}, resizing:{}", *dragging, *on_edge, *resizing).into());
				return;
			}
			
			let dx = event.client_x() - *clickx;
			let dy = event.client_y() - *clicky;

			let new_left = *drag_start_left + dx;
			let new_top = *drag_start_top + dy;

			console::log_1(&format!("top: {}, left: {}, dx: {}, dy: {}, startX: {}, startY: {}, startTop: {}, startLeft: {}", new_top, new_left, dx, dy, *clickx, *clicky, *drag_start_top, *drag_start_left).into());

			mousex.set(new_left);
			mousey.set(new_top);

		}
	};

	let onmousedown = {
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let on_edge = resizing.clone();
		
		let clickx = clickx.clone();
		let clicky = clicky.clone();

		let drag_start_left = drag_start_left.clone();
		let drag_start_top = drag_start_top.clone();

		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		let hidden = hidden.clone();

		let div_node_ref = div_node_ref.clone();
		move |event: MouseEvent| {
			if event.button() != 0 {
				return;
			}

			if *resizing || *on_edge || *dragging || *hidden {
				return;
			}

			clickx.set(event.client_x());
			clicky.set(event.client_y());

			let element = div_node_ref.cast::<HtmlElement>().unwrap();

			drag_start_left.set(element.offset_left());
			drag_start_top.set(element.offset_top());

			dragging.set(true);

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

		let div_node_ref = div_node_ref.clone();
		move |_: MouseEvent| {
			// Unfocus element
			div_node_ref.cast::<HtmlElement>().unwrap().blur().unwrap();

			dragging.set(false);
			resizing.set(false);

			z_index.set(*old_z_index);
		}
	};

	let onmouseenter = {
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		
		let div_node_ref = div_node_ref.clone();
		move |_: MouseEvent| {
			// Focus element
			focus_element(&div_node_ref.cast::<HtmlElement>().unwrap());
			
			dragging.set(false);
			resizing.set(false);

			z_index.set(*old_z_index);
		}
	};

	// Mouse events for each resizer
	let on_top_left_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_x = resize_start_x.clone();
		let resize_start_y = resize_start_y.clone();
		let drag_start_top = drag_start_top.clone();
		let drag_start_left = drag_start_left.clone();
		let mousex = mousex.clone();
		let mousey = mousex.clone();
		let width = width.clone();
		let height = height.clone();
		let resize_start_width = resize_start_width.clone();
		let resize_start_height = resize_start_height.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			let dx = event.client_x() - *resize_start_x;
			let dy = event.client_y() - *resize_start_y;
			let aspect_ratio = *resize_start_width as f64 / *resize_start_height as f64;

			let new_width = *resize_start_width - dx;
			let new_height = (new_width as f64 / aspect_ratio) as i32;
			let new_y = *drag_start_top + dy;
			let new_x = *drag_start_left + dx;

			width.set(new_width);
			height.set(new_height);

			mousex.set(new_x);
			mousey.set(new_y);
		}
	};

	let on_top_right_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_x = resize_start_x.clone();
		let resize_start_y = resize_start_y.clone();
		let drag_start_top = drag_start_top.clone();
		let width = width.clone();
		let height = height.clone();
		let resize_start_width = resize_start_width.clone();
		let resize_start_height = resize_start_height.clone();
		let mousey = mousey.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			let dx = event.client_x() - *resize_start_x;
			let dy = event.client_y() - *resize_start_y;
			let aspect_ratio = *resize_start_width as f64 / *resize_start_height as f64;

			let new_width = *resize_start_width + dx;
			let new_height = (new_width as f64 / aspect_ratio) as i32;

			let new_y = *drag_start_top + dy;

			width.set(new_width);
			height.set(new_height);

			mousey.set(new_y);
		}
	};

	let on_bottom_left_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_x = resize_start_x.clone();
		let drag_start_left = drag_start_left.clone();
		let mousex = mousex.clone();
		let width = width.clone();
		let height = height.clone();
		let resize_start_width = resize_start_width.clone();
		let resize_start_height = resize_start_height.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			let dx = event.client_x() - *resize_start_x;
			let aspect_ratio = *resize_start_width as f64 / *resize_start_height as f64;

			let new_width = *resize_start_width - dx;
			let new_height = (new_width as f64 / aspect_ratio) as i32;
			let new_x = *drag_start_left + dx;

			width.set(new_width);
			height.set(new_height);
			mousex.set(new_x);
		}
	};

	let on_bottom_right_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_x = resize_start_x.clone();
		let width = width.clone();
		let height = height.clone();
		let resize_start_width = resize_start_width.clone();
		let resize_start_height = resize_start_height.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			let dx = event.client_x() - *resize_start_x;
			let aspect_ratio = *resize_start_width as f64 / *resize_start_height as f64;

			let new_width = *resize_start_width + dx;
			let new_height = (new_width as f64 / aspect_ratio) as i32;

			width.set(new_width);
			height.set(new_height);
		}
	};

	let on_top_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_y = resize_start_y.clone();
		let height = height.clone();
		let mousey = mousey.clone();
		let drag_start_top = drag_start_top.clone();
		let on_corner = on_corner.clone();
		let resize_start_height = resize_start_height.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}

			let dy = event.client_y() - *resize_start_y;

			let new_height = *resize_start_height - dy;
			let new_y = *drag_start_top + dy;

			height.set(new_height);
			mousey.set(new_y);
		}
	};

	let on_bottom_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_y = resize_start_y.clone();
		let height = height.clone();
		let on_corner = on_corner.clone();
		let resize_start_height = resize_start_height.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}

			let dy = event.client_y() - *resize_start_y;

			let new_height = *resize_start_height + dy;

			height.set(new_height);
		}
	};

	let on_left_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_x = resize_start_x.clone();
		let drag_start_left = drag_start_left.clone();
		let mousex = mousex.clone();
		let width = width.clone();
		let on_corner = on_corner.clone();
		let resize_start_width = resize_start_width.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}
			
			let dx = event.client_x() - *resize_start_x;

			let new_width = *resize_start_width - dx;
			let new_x = *drag_start_left + dx;

			width.set(new_width);
			mousex.set(new_x);
		}
	};

	let on_right_resizer_move = {
		let resizing = resizing.clone();
		let resize_start_x = resize_start_x.clone();
		let width = width.clone();
		let on_corner = on_corner.clone();
		let resize_start_width = resize_start_width.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}

			let dx = event.client_x() - *resize_start_x;

			let new_width = *resize_start_width + dx;

			width.set(new_width);
		}
	};

	let on_resizer_click = {
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let resizer_start_x = resize_start_x.clone();
		let resizer_start_y = resize_start_y.clone();
		let drag_start_top = drag_start_top.clone();
		let drag_start_left = drag_start_left.clone();
		let resizer_start_width = resize_start_width.clone();
		let resizer_start_height = resize_start_height.clone();

		let div_node_ref = div_node_ref.clone();
		move |event: MouseEvent| {
			resizer_start_x.set(event.client_x());
			resizer_start_y.set(event.client_y());

			let element = div_node_ref.cast::<HtmlElement>().unwrap();

			drag_start_left.set(element.offset_left());
			drag_start_top.set(element.offset_top());
			resizer_start_width.set(element.offset_width());
			resizer_start_height.set(element.offset_height());

			dragging.set(false);
			resizing.set(true);
		}
	};

	let on_resizer_unclick = {
		let resizing = resizing.clone();
		move |_: MouseEvent| {
			resizing.set(false);
		}
	};

	let on_resizer_leave = {
		let resizing = resizing.clone();
		let on_edge = on_edge.clone();
		move |_: MouseEvent| {
			resizing.set(false);
			on_edge.set(false);
		}
	};

	let on_resizer_enter = {
		let resizing = resizing.clone();
		let on_edge = on_edge.clone();
		move |_: MouseEvent| {
			resizing.set(false);
			on_edge.set(true);
		}
	};

	// This is to deal with the fact that the edge and corner resizers overlap
	let on_resizer_corner_enter = {
		let on_corner = on_corner.clone();
		move |_: MouseEvent| {
			on_corner.set(true);
		}
	};

	let on_resizer_corner_leave = {
		let on_corner = on_corner.clone();
		move |_: MouseEvent| {
			on_corner.set(false);
		}
	};

	let style = match *hidden {
		true => "display: none;".to_string(),
		false => format!(
			"position: absolute; left: {}px; top: {}px; z-index: {}; width: {}px; height: {}px;{}",
			*mousex,
			*mousey,
			*z_index,
			*width,
			*height,
			extra_style,
		),
	};

	// Styles to keep the resizers in the corners and edges
	let resizer_size_pixels = 10;
	let edge_resizer_style = "position: absolute; z-index: 2;";
	let corner_resizer_style = format!("position: absolute; z-index: 3; width: {}px; height: {}px;", resizer_size_pixels, resizer_size_pixels);
	let edge_resizer_top_style = format!("{}top: 0; left: 0; right: 0; width: 100%; height: {}px", edge_resizer_style, resizer_size_pixels);
	let edge_resizer_bottom_style = format!("{}bottom: 0; left: 0; right: 0; width: 100%; height: {}px", edge_resizer_style, resizer_size_pixels);
	let edge_resizer_left_style = format!("{}top: 0; bottom: 0; left: 0; width: {}px; height: 100%", edge_resizer_style, resizer_size_pixels);
	let edge_resizer_right_style = format!("{}top: 0; bottom: 0; right: 0; width: {}px; height: 100%", edge_resizer_style, resizer_size_pixels);
	let corner_resizer_top_left_style = format!("{}top: 0; left: 0;", corner_resizer_style);
	let corner_resizer_top_right_style = format!("{}top: 0; right: 0;", corner_resizer_style);
	let corner_resizer_bottom_left_style = format!("{}bottom: 0; left: 0;", corner_resizer_style);
	let corner_resizer_bottom_right_style = format!("{}bottom: 0; right: 0;", corner_resizer_style);

	html! {
		<div ref={div_node_ref} {onkeydown} {onmousemove} {onmousedown} {onmouseup} id={id.clone()} {class} {onmouseenter} {onmouseleave} {style} tabindex="0">
			<div style={corner_resizer_top_left_style} onmousemove={on_top_left_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_corner_enter.clone()} onmouseleave={on_resizer_corner_leave.clone()} class="image-resize-div corner-resize-div top-left" id={format!("{}-top-left-resizer", id.clone())} />
			<div style={corner_resizer_top_right_style} onmousemove={on_top_right_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_corner_enter.clone()} onmouseleave={on_resizer_corner_leave.clone()} class="image-resize-div corner-resize-div top-right" id={format!("{}-top-right-resizer", id.clone())} />
			<div style={corner_resizer_bottom_left_style} onmousemove={on_bottom_left_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_corner_enter.clone()} onmouseleave={on_resizer_corner_leave.clone()} class="image-resize-div corner-resize-div bottom-left" id={format!("{}-bottom-left-resizer", id.clone())} />
			<div style={corner_resizer_bottom_right_style} onmousemove={on_bottom_right_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_corner_enter.clone()} onmouseleave={on_resizer_corner_leave.clone()} class="image-resize-div corner-resize-div bottom-right" id={format!("{}-bottom-right-resizer", id.clone())} />
			<div style={edge_resizer_top_style} onmousemove={on_top_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_enter.clone()} onmouseleave={on_resizer_leave.clone()} class="image-resize-div edge-resize-div top" id={format!("{}-top-resizer", id.clone())} />
			<div style={edge_resizer_left_style} onmousemove={on_left_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_enter.clone()} onmouseleave={on_resizer_leave.clone()} class="image-resize-div edge-resize-div left" id={format!("{}-bottom-resizer", id.clone())} />
			<div style={edge_resizer_right_style} onmousemove={on_right_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_enter.clone()} onmouseleave={on_resizer_leave.clone()} class="image-resize-div edge-resize-div right" id={format!("{}-left-resizer", id.clone())} />
			<div style={edge_resizer_bottom_style} onmousemove={on_bottom_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_enter.clone()} onmouseleave={on_resizer_leave.clone()} class="image-resize-div edge-resize-div bottom" id={format!("{}-right-resizer", id.clone())} />
			{ props.children.clone() }
		</div>
	}
}

