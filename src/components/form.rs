use leptos::*;

// annotate the function with #[component]
#[component]
pub fn Form(cx: Scope, /* add any props you need */) -> impl IntoView {
    // create your UI and logic here
    view! {
        cx,
        // use HTML elements and other components here
        <form>
            // ...
        </form>
    }
}