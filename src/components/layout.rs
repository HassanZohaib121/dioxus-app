// use crate::Route;
use dioxus::prelude::*;

use crate::components::Navbar;

#[derive(Props, PartialEq, Clone)]
pub struct FrontendLayout {
    children: Element,
    title: Option<String>,
}

#[component]
pub fn FrontendLayout(props: FrontendLayout) -> Element {
    let title = props.title.unwrap_or_default();
    rsx! {
        div { class: "min-h-full",
            Navbar {}
            if title != "" {
            header { class: "bg-white shadow-sm",
                div { class: "mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8",
                    h1 { class: "text-3xl mt-20 font-bold tracking-tight text-gray-900",
                        "{title}"
                    }
                }
            }
            }

            main {
                div { class: "mx-auto max-w-screen",
                    {props.children}
                }
            }
        }
    }
}
