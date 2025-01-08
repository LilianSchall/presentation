use leptos::mount::mount_to_body;

pub mod components;

fn main() {
    mount_to_body(|| components::App)
}
