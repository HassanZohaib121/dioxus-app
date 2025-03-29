use crate::Route;
use dioxus::prelude::*;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    let mut is_menu_open = use_signal(|| false);

    rsx! {
        nav { class: "fixed w-full bg-gray-900/80 backdrop-blur-md z-50 transition-all duration-300",
            div { class: "container mx-auto max-w-7xl px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center h-16",
                    // Logo
                    div { class: "flex items-center",
                        div { class: "shrink-0",
                            Link {
                                class: "text-2xl font-bold text-white hover:text-blue-400 transition-colors duration-300",
                                to: Route::Home {},
                                "TechStart"
                            }
                        }
                    }

                    // Desktop Navigation
                div { class: "hidden md:block",
                     div { class: "ml-10 flex items-baseline space-x-4",
                        Link {
                            class: "text-gray-300 hover:text-white transition-colors duration-300",
                            to: Route::Home {},
                            "Home"
                        }
                        Link {
                            class: "text-gray-300 hover:text-white transition-colors duration-300",
                            to: Route::Services {},
                            "Services"
                        }
                        Link {
                            class: "text-gray-300 hover:text-white transition-colors duration-300",
                            to: Route::BlogArchive {},
                            "Blog"
                        }
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-full transition-all duration-300 hover:scale-105",
                            "Get Started"
                        }
                    }
                }

                    // Mobile Menu Button
                    button {
                        class: "md:hidden text-gray-300 hover:text-white transition-colors duration-300",
                        onclick: move |_| {
                            is_menu_open.set(!is_menu_open());
                        },
                        if is_menu_open() {
                            "✕"
                        } else {
                            "☰"
                        }
                    }
                }

                // Mobile Navigation
                div {
                    class: "md:hidden transition-all duration-300 overflow-hidden",
                    class: if is_menu_open() { "max-h-64" } else { "max-h-0" },
                    div { class: "py-4 space-y-4",
                        Link {
                            class: "block text-gray-300 hover:text-white transition-colors duration-300",
                            to: Route::Home {},
                            "Home"
                        }
                        Link {
                            class: "block text-gray-300 hover:text-white transition-colors duration-300",
                            to: Route::Services {},
                            "Services"
                        }
                        Link {
                            class: "block text-gray-300 hover:text-white transition-colors duration-300",
                            to: Route::BlogArchive {},
                            "Blog"
                        }
                        button {
                            class: "w-full bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-full transition-all duration-300 hover:scale-105",
                            "Get Started"
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
