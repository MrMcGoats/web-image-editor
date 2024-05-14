use yew::prelude::*;
use web_sys::{MouseEvent, console, HtmlElement};
use gloo_events::EventListener;

#[derive(PartialEq, Properties)]
struct Props {
	id: AttrValue,
	style: AttrValue,
	children: Children,
}

#[function_component]
fn App() -> Html {
	let counter = use_state(|| 0);
	let onclick = {
		let counter = counter.clone();
		move |_| {
			let value = *counter + 1;
			counter.set(value);
		}
	};

	let onclickminus = {
		let counter = counter.clone();
		move |_| {
			let value = *counter - 1;
			counter.set(value);
		}
	};


	html! {
		<div>
			<MouseMoveComponent id="mousebox" style="background-color: red;">
				<button {onclick}>{ "+1" }</button>
				<button onclick={onclickminus}>{ "-1" }</button>
				<p>{ *counter }</p>
			</MouseMoveComponent>
		</div>
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
		move |_: MouseEvent| {
			dragging.set(true);
		}
	};

	let onmouseup = {
		let dragging = dragging.clone();
		move |_: MouseEvent| {
			dragging.set(false);
		}
	};

	let onmouseleave = {
		let dragging = dragging.clone();
		move |_: MouseEvent| {
			dragging.set(false);
		}
	};

	let onmouseenter = {
		let dragging = dragging.clone();
		move |_: MouseEvent| {
			dragging.set(false);
		}
	};

	html! {
		<div ref={div_node_ref} {onmousemove} {onmousedown} {onmouseup} {id} {onmouseenter} {onmouseleave} style={format!("position: absolute; left: {}px; top: {}px; {}", *mousex, *mousey, extra_style)}>
			{ props.children.clone() }
		</div>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
