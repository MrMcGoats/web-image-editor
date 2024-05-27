// Code for a generic div that can be moved around the screen by dragging it with the mouse
// Can also be resized

use web_sys::{MouseEvent, HtmlElement};
use web_sys::console;
use yew::prelude::*;

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
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let on_edge = on_edge.clone();
		
		let div_node_ref = div_node_ref.clone();
		move |event: MouseEvent| {
			if !*dragging || *on_edge || *resizing {
				console::log_1(&format!("Mouse move: dragging:{}, on_edge:{}, resizing:{}", *dragging, *on_edge, *resizing).into());
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
		let resizing = resizing.clone();
		let on_edge = resizing.clone();
		
		let clickx = clickx.clone();
		let clicky = clicky.clone();

		let z_index = z_index.clone();
		let old_z_index = old_z_index.clone();
		let hidden = hidden.clone();
		move |event: MouseEvent| {
			if event.button() != 0 {
				return;
			}

			if *resizing || *on_edge || *dragging || *hidden {
				return;
			}

			clickx.set(event.client_x());
			clicky.set(event.client_y());

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
			div_node_ref.cast::<HtmlElement>().unwrap().focus().unwrap();
			
			dragging.set(false);
			resizing.set(false);

			z_index.set(*old_z_index);
		}
	};

	// Mouse events for each resizer
	let on_top_left_resizer_move = {
		let resizing = resizing.clone();
		let mousex = mousex.clone();
		let mousey = mousey.clone();
		let clickx = clickx.clone();
		let clicky = clicky.clone();
		let width = width.clone();
		let height = height.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			// Resize the div up and to the left
			// Maintain aspect ratio, and keep the cursor in the same spot relative to the div (move
			// the div to accomodate the cursor)
			let x = event.client_x();
			let y = event.client_y();

			let x1 = x - *clickx;
			let y1 = y - *clicky;

			let new_width = *width - x1;
			let new_height = *height - y1;

			let x2 = *mousex + x1;
			let y2 = *mousey + y1;

			mousex.set(x2);
			mousey.set(y2);

			width.set(new_width);
			height.set(new_height);
		}
	};

	let on_top_right_resizer_move = {
		let resizing = resizing.clone();
		let mousex = mousex.clone();
		let mousey = mousey.clone();
		let clickx = clickx.clone();
		let clicky = clicky.clone();
		let width = width.clone();
		let height = height.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			let x = event.client_x();
			let y = event.client_y();

			let x1 = x - *clickx;
			let y1 = y - *clicky;

			let new_width = *width + x1;
			let new_height = *height - y1;

			let x2 = *mousex;
			let y2 = *mousey + y1;

			mousex.set(x2);
			mousey.set(y2);
			width.set(new_width);
			height.set(new_height);
		}
	};

	let on_bottom_left_resizer_move = {
		let resizing = resizing.clone();
		let mousex = mousex.clone();
		let mousey = mousey.clone();
		let clickx = clickx.clone();
		let clicky = clicky.clone();
		let width = width.clone();
		let height = height.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			let x = event.client_x();
			let y = event.client_y();

			let x1 = x - *clickx;
			let y1 = y - *clicky;

			let new_width = *width - x1;
			let new_height = *height + y1;

			let x2 = *mousex + x1;
			let y2 = *mousey;

			mousex.set(x2);
			mousey.set(y2);
			width.set(new_width);
			height.set(new_height);
		}
	};

	let on_bottom_right_resizer_move = {
		let resizing = resizing.clone();
		let mousex = mousex.clone();
		let mousey = mousey.clone();
		let clickx = clickx.clone();
		let clicky = clicky.clone();
		let width = width.clone();
		let height = height.clone();
		move |event: MouseEvent| {
			if !*resizing {
				return;
			}

			let x = event.client_x();
			let y = event.client_y();

			let x1 = x - *clickx;
			let y1 = y - *clicky;

			let new_width = *width + x1;
			let new_height = *height + y1;

			let x2 = *mousex;
			let y2 = *mousey;

			mousex.set(x2);
			mousey.set(y2);
			width.set(new_width);
			height.set(new_height);
		}
	};

	let on_top_resizer_move = {
		let resizing = resizing.clone();
		let mousey = mousey.clone();
		let clicky = clicky.clone();
		let height = height.clone();
		let on_corner = on_corner.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}

			let y = event.client_y();

			let y1 = y - *clicky;

			let new_height = *height - y1;

			let y2 = *mousey + y1;

			mousey.set(y2);
			height.set(new_height);
		}
	};

	let on_bottom_resizer_move = {
		let resizing = resizing.clone();
		let mousey = mousey.clone();
		let clicky = clicky.clone();
		let height = height.clone();
		let on_corner = on_corner.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}

			let y = event.client_y();

			let y1 = y - *clicky;

			let new_height = *height + y1;

			let y2 = *mousey;

			mousey.set(y2);
			height.set(new_height);
		}
	};

	let on_left_resizer_move = {
		let resizing = resizing.clone();
		let mousex = mousex.clone();
		let clickx = clickx.clone();
		let width = width.clone();
		let on_corner = on_corner.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}

			let x = event.client_x();

			let x1 = x - *clickx;

			let new_width = *width + x1;

			let x2 = *mousex + x1;

			mousex.set(x2);
			width.set(new_width);
		}
	};

	let on_right_resizer_move = {
		let resizing = resizing.clone();
		let mousex = mousex.clone();
		let clickx = clickx.clone();
		let width = width.clone();
		let on_corner = on_corner.clone();
		move |event: MouseEvent| {
			if !*resizing || *on_corner {
				return;
			}

			let x = event.client_x();

			let x1 = x - *clickx;

			let new_width = *width + x1;

			let x2 = *mousex;

			mousex.set(x2);
			width.set(new_width);
		}
	};

	let on_resizer_click = {
		let dragging = dragging.clone();
		let resizing = resizing.clone();
		let clickx = clickx.clone();
		let clicky = clicky.clone();
		move |event: MouseEvent| {
			clickx.set(event.client_x());
			clicky.set(event.client_y());

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
			"position: absolute; left: {}px; top: {}px; z-index: {}; width: {}px; max-width: {}px; height: {}px; max-height: {}px; {}",
			*mousex,
			*mousey,
			*z_index,
			*width,
			*height,
			*width,
			*height,
			extra_style,
		),
	};

	// Styles to keep the resizers in the corners and edges
	let resizer_size_percent = 7;
	let edge_resizer_style = "position: absolute; z-index: 2;";
	let corner_resizer_style = format!("position: absolute; z-index: 3; width: {}%; height: {}%;", resizer_size_percent, resizer_size_percent);
	let edge_resizer_top_style = format!("{}top: 0; left: 0; right: 0; width: {}%; height: {}%", edge_resizer_style, 100 - resizer_size_percent, resizer_size_percent);
	let edge_resizer_bottom_style = format!("{}bottom: 0; left: 0; right: 0; width: {}%; height: {}%", edge_resizer_style, 100 - resizer_size_percent, resizer_size_percent);
	let edge_resizer_left_style = format!("{}top: 0; bottom: 0; left: 0; width: {}%; height: {}%", edge_resizer_style, resizer_size_percent, 100 - resizer_size_percent);
	let edge_resizer_right_style = format!("{}top: 0; bottom: 0; right: 0; width: {}%; height: {}%", edge_resizer_style, resizer_size_percent, 100 - resizer_size_percent);
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
			<div style={edge_resizer_bottom_style} onmousemove={on_left_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_enter.clone()} onmouseleave={on_resizer_leave.clone()} class="image-resize-div edge-resize-div left" id={format!("{}-bottom-resizer", id.clone())} />
			<div style={edge_resizer_left_style} onmousemove={on_right_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_enter.clone()} onmouseleave={on_resizer_leave.clone()} class="image-resize-div edge-resize-div right" id={format!("{}-left-resizer", id.clone())} />
			<div style={edge_resizer_right_style} onmousemove={on_bottom_resizer_move} onmousedown={on_resizer_click.clone()} onmouseup={on_resizer_unclick.clone()} onmouseenter={on_resizer_enter.clone()} onmouseleave={on_resizer_leave.clone()} class="image-resize-div edge-resize-div bottom" id={format!("{}-right-resizer", id.clone())} />
			{ props.children.clone() }
		</div>
	}
}

