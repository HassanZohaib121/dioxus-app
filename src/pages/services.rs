use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    rsx! {
        FrontendLayout {
            title: "Services".to_string(),
            h1 { "Services" }
        }
    }
}
