use leptos::*;
mod app;
use app::App;
mod components;
mod  helpers;
mod  common;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}