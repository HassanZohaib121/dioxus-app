mod admin;
mod components;
mod pages;

use admin::*;
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
    #[route("/admin/blog-form")]
    BlogForm {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
// const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    rsx! {
        head {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            // document::Link { rel: "stylesheet", href: MAIN_CSS }
        }
        Router::<Route> {}
    }
}
