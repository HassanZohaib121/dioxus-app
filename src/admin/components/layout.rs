use crate::Route;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AdminLayout {
    children: Element,
    title: String,
}

#[component]
pub fn AdminLayout(props: AdminLayout) -> Element {
    let mut is_mobile_menu_open = use_signal(|| false);

    rsx! {
        div { class: "min-h-full",
            nav { class: "bg-gray-800",
                div { class: "mx-auto max-w-7xl px-4 sm:px-6 lg:px-8",
                    div { class: "flex h-16 items-center justify-between",
                        div { class: "flex items-center",
                            div { class: "shrink-0",
                                img {
                                    class: "size-8",
                                    src: "https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=500",
                                    alt: "Company Logo"
                                }
                            }
                            div { class: "hidden md:block",
                                div { class: "ml-10 flex items-baseline space-x-4",
                                    Link {
                                        to: Route::AdminDashboard {},
                                        class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white",
                                        "Dashboard"
                                    }
                                    Link {
                                        to: Route::UsersManagement {},
                                        class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white",
                                        "Users"
                                    }
                                }
                            }
                        }
                        div { class: "hidden md:block",
                            div { class: "ml-4 flex items-center md:ml-6",
                                button {
                                    class: "relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden",
                                    onclick: move |_| {},
                                    span { class: "absolute -inset-1.5" }
                                    span { class: "sr-only", "View notifications" }
                                }
                                div { class: "relative ml-3",
                                    button {
                                        class: "relative flex max-w-xs items-center rounded-full bg-gray-800 text-sm focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden",
                                        onclick: move |_| {},
                                        span { class: "absolute -inset-1.5" }
                                        span { class: "sr-only", "Open user menu" }
                                        img {
                                            class: "size-8 rounded-full",
                                            src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
                                            alt: "User avatar"
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "-mr-2 flex md:hidden",
                            button {
                                class: "relative inline-flex items-center justify-center rounded-md bg-gray-800 p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden",
                                onclick: move |_| is_mobile_menu_open.set(!is_mobile_menu_open()),
                                span { class: "absolute -inset-0.5" }
                                span { class: "sr-only", "Open main menu" }
                            }
                        }
                    }
                }
                // Mobile menu
                div {
                    class: "md:hidden",
                    hidden: !is_mobile_menu_open(),
                    div { class: "space-y-1 px-2 pt-2 pb-3 sm:px-3",
                        Link {
                            to: Route::AdminDashboard {},
                            class: "block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white",
                            "Dashboard"
                        }
                        Link {
                            to: Route::UsersManagement {},
                            class: "block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white",
                            "Users"
                        }
                    }
                }
            }

            header { class: "bg-white shadow-sm",
                div { class: "mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8",
                    h1 { class: "text-3xl font-bold tracking-tight text-gray-900",
                        "{props.title}"
                    }
                }
            }

            main {
                div { class: "mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8",
                    {props.children}
                }
            }
        }
    }
}
