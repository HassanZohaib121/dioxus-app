use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    rsx! {
        FrontendLayout {
            // title: "Services".to_string(),
            div { class: "min-h-screen bg-gray-50 py-16 px-4 sm:px-6 lg:px-8",
            h1 { "Services" }
            }
        }
    }
}
