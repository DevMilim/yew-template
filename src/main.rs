mod components;
mod views;
pub use components::*;

use views::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
