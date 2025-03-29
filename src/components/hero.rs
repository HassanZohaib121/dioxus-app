use dioxus::prelude::*;

// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            // img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a {class:"text-red-600", href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a {class:"text-red-600", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a {class:"text-red-600", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a {class:"text-red-600", href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a {class:"text-red-600", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a {class:"text-red-600", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                // a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                // a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                // a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                // a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                // a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
