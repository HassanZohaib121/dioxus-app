use crate::admin::pages::dashboard::AdminDashboard;
use dioxus::prelude::*;
// use gloo::console::log;
use serde::{Deserialize, Serialize};
#[warn(non_snake_case)]
// Our main App component
#[component]
pub fn Login() -> Element {
    let mut auth_state = use_signal(|| AuthState {
        is_authenticated: false,
        user: None,
    });

    rsx! {
        div { class: "min-h-screen bg-gray-100 flex flex-col",
            header { class: "bg-white shadow p-4",
                div { class: "container mx-auto flex justify-between items-center",
                    h1 { class: "text-xl font-bold", "Dioxus Admin Panel" }
                    if auth_state.read().is_authenticated {
                        button {
                            class: "px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600",
                            onclick: move |_| {
                                auth_state.write().is_authenticated = false;
                                auth_state.write().user = None;
                            },
                            "Logout"
                        }
                    }
                }
            }
            if auth_state.read().is_authenticated {
                AdminDashboard {}
            } else {
                main { class: "container mx-auto flex-grow flex items-center justify-center p-4",
                LoginForm { auth_state: auth_state }
                }
            }
            footer { class: "bg-white p-4 shadow-inner",
                div { class: "container mx-auto text-center text-gray-500",
                    "© 2025 Dioxus Admin Panel"
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

            // if !error.read().is_empty() {
            //     div { class: "mb-4 p-2 bg-red-100 text-red-700 rounded",
            //         "{error}"
            //     }
            // }

            div { class: "mb-4",
                label { class: "block text-gray-700 text-sm font-bold mb-2", r#for: "username",
                    "Username"
                }
                input {
                    class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                    id: "username",
                    r#type: "text",
                    placeholder: "Username",
                    value: "{username}", // Access the username field directly
                    oninput: move |event| {
                        username.set(event.value());
                    }
                }
                "Password"
            }
            input {
                class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                id: "password",
                r#type: "password",
                placeholder: "Password",
                value: "{password}", // Access the password field directly
                oninput: move |event| {
                    password.set(event.value());
                },
            }

            div { class: "flex items-center justify-between",
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
                a { class: "inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800", href: "#",
                    "Forgot Password?"
                }
            }

            div { class: "mt-6 text-center text-sm",
                p { "Demo credentials:" }
                p { class: "font-mono bg-gray-100 p-1 rounded", "Username: admin" }
                p { class: "font-mono bg-gray-100 p-1 rounded", "Password: password" }
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
