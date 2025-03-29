mod admin;
mod components;
mod pages;

use admin::*;
use components::*;
use dioxus::prelude::*;
use pages::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    BlogArchive {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/new")]
    New {},
    #[route("/services")]
    Services {},
    #[route("/admin")]
    AdminDashboard {},
    #[route("/admin/users")]
    UsersManagement {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
// const TAILWIND_CSS: Asset = asset!("/input.css");

#[component]
fn App() -> Element {
    rsx! {
        head {
            // document::Script {src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@3",}
            document::Link { rel: "icon", href: FAVICON }
            // document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        }
        Router::<Route> {}
    }
}
