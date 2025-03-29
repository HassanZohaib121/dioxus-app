use crate::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Props, PartialEq, Clone)]
pub struct AdminLayout {
    children: Element,
    title: String,
    // is_authenticated: bool,
}

#[component]
pub fn AdminLayout(props: AdminLayout) -> Element {
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut auth_state = use_signal(|| AuthState {
        is_authenticated: false,
        user: None,
    });

    rsx! {
        div { class: "min-h-full",
            nav { class: "bg-gray-800",
                div { class: "mx-auto max-w-7xl px-4 sm:px-6 lg:px-8",
                    div { class: "flex h-16 items-center justify-between",
                        div { class: "flex items-center",
                            div { class: "shrink-0",
                                // img {
                                //     class: "size-8",
                                //     src: "https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=500",
                                //     alt: "Company Logo"
                                // }
                                Link {
                                    class: "text-2xl font-bold text-white hover:text-blue-400 transition-colors duration-300",
                                    to: Route::Home {},
                                    "TechStart"
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
                                    Link {
                                        to: Route::BlogForm {},
                                        class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white",
                                        "Blog Form"
                                    }
                                }
                            }
                        }
                        div { class: "hidden md:block",
                            div { class: "ml-4 flex items-center md:ml-6",
                                if auth_state.read().is_authenticated {
                                button {
                                    class: "relative rounded-full bg-gray-800 p-1 text-gray-400 hover:text-white focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800 focus:outline-hidden",
                                    onclick: move |_| {},
                                    span { class: "absolute -inset-1.5" }
                                    span { class: "sr-only", "View notifications" }
                                }
                                button {
                                    class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                                    onclick: move |_| {
                                        auth_state.write().is_authenticated = false;
                                        auth_state.write().user = None;
                                    },
                                    "Logout"
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
                        Link {
                            to: Route::BlogForm {},
                            class: "block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white",
                            "Blog Form"
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
                if auth_state.read().is_authenticated {
                div { class: "mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8",
                    {props.children}
                }
                } else {
                    main { class: "container mx-auto flex-grow flex items-center justify-center p-4",
                        LoginForm { auth_state: auth_state }
                    }
                }
            }
        }
    }
}

// Login form component
#[component]
pub fn LoginForm(auth_state: Signal<AuthState>) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut loading = use_signal(|| false);

    let handle_submit = move |_| {
        error.set(String::new());
        loading.set(true);

        // Simple validation
        if username.read().is_empty() || password.read().is_empty() {
            error.set("Username and password are required".to_string());
            loading.set(false);
            return;
        }

        // Simulate API call with a timeout
        // In a real app, you would make an actual API request here
        let username_clone = username.read().clone();
        let password_clone = password.read().clone();

        // Simulate async authentication
        // In a real app, you would use proper async/await with a real API call
        // log!("Username: {}, Password: {}", username_clone, password_clone);
        spawn(async move {
            let success = username_clone == "admin" && password_clone == "password";

            // Update state based on authentication result
            if success {
                auth_state.write().is_authenticated = true;
                auth_state.write().user = Some(User {
                    username: username_clone,
                    display_name: "Admin User".to_string(),
                });
            } else {
                error.set("Invalid username or password".to_string());
            }

            loading.set(false);
        });
    };

    rsx! {
    div { class: "w-full max-w-md",
        div { class: "bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4",
            h2 { class: "text-2xl font-bold mb-6 text-center", "Admin Login" }

            if !error.read().is_empty() {
                div { class: "mb-4 p-2 bg-red-100 text-red-700 rounded",
                    "{error}"
                }
            }

            div { class: "mb-4 flex flex-col gap-2",
                input {
                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                    id: "username",
                    r#type: "text",
                    placeholder: "Username",
                    value: "{username}",
                    required: true,
                    oninput: move |event| {
                        username.set(event.value());
                    }
                }
                input {
                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                    id: "password",
                    r#type: "password",
                    placeholder: "Password",
                    value: "{password}",
                    required: true,
                    oninput: move |event| {
                        password.set(event.value());
                    },
                }

                div { class: "flex items-center justify-between mt-2",
                    a { class: "inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800", href: "#",
                        "Forgot Password?"
                    }
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:opacity-50",
                    r#type: "button",
                    onclick: handle_submit,
                    disabled: loading,

                    if loading() {
                        "Logging in..."
                    } else {
                        "Sign In"
                    }
                }
            }

                    // div { class: "mt-6 text-center text-sm",
                    //     p { "Demo credentials:" }
                    //     p { class: "font-mono bg-gray-100 p-1 rounded", "Username: admin" }
                    //     p { class: "font-mono bg-gray-100 p-1 rounded", "Password: password" }
                    // }
                }
             }
        }
    }
}

// Auth state structure
#[derive(Clone)]
pub struct AuthState {
    is_authenticated: bool,
    user: Option<User>,
}

// User structure
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct User {
    username: String,
    display_name: String,
}
