mod app;
pub mod file_details;
pub mod text_details;
pub mod page_items;

use wasm_bindgen::prelude::*;
use web_sys::console;
use app::App;

fn main() {
	#[cfg(feature = "standalone")]
	yew::Renderer::<App>::new().render();
}

#[cfg(not(feature = "standalone"))]
#[wasm_bindgen(start)]
pub fn run_app() {
	let document = web_sys::window().unwrap().document().unwrap();
	let root = document.get_element_by_id("canvas-root").unwrap();
	console::log_1(&"Starting app".into());
	yew::Renderer::<App>::with_root(root).render();
}
