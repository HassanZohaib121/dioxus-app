use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn New() -> Element {
    rsx! {
        FrontendLayout {
            title: "New".to_string(),
            h1 { "New" }
        }
    }
}
