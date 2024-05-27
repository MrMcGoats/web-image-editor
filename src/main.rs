mod app;
pub mod file_details;
pub mod text_details;
pub mod page_items;

use web_sys::console;
use app::App;

fn main() {
	yew::Renderer::<App>::new().render();
}
