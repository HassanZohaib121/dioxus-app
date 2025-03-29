use dioxus::prelude::*;

use crate::admin::components::AdminLayout;
#[component]
pub fn BlogForm() -> Element {
    let mut title = use_signal(|| String::new());
    let mut content = use_signal(|| String::new());
    rsx! {
        AdminLayout {
            title: "Blog Form",
            form {
                div { class: "mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6",
                    div { class: "sm:col-span-4 space-y-4",
                        label { class:"block text-sm/6 text-gray-900 mb-2 text-left font-bold", "Title" }
                        input {
                        class: "border border-black border-1 rounded-md p-2 w-full focus:outline-black focus:ring-2 focus:ring-gray-300 focus:border-none focus:shadow-md focus:shadow-gray-300 focus:shadow-inner focus:shadow-gray-300 focus:shadow-md focus:shadow-gray-300 focus:shadow-inner focus:shadow-gray-300",
                        placeholder: "Title",
                        type: "text",
                        name: "title",
                        value: "{title}",
                        oninput: move |event| {
                            title.set(event.value());
                        }
                        }
                        label { class:"block text-sm/6 text-gray-900 mb-2 text-left font-bold", "Content" }
                        input {
                            class: "border border-black border-1 rounded-md p-2 w-full focus:outline-black focus:ring-1 focus:ring-gray-300 focus:border-none focus:shadow-md focus:shadow-gray-300 focus:shadow-inner focus:shadow-gray-300 focus:shadow-md focus:shadow-gray-300 focus:shadow-inner focus:shadow-gray-300",
                            placeholder: "Content",
                            type: "text",
                            name: "content",
                            value: "{content}",
                            oninput: move |event| {
                                content.set(event.value());
                            }
                        }

                    button { onclick: move |_| async move{
                        let result = blog_form_server(title.clone().to_string(), content.clone().to_string()).await;
                            if let Ok(result) = result {
                                println!("{}", result);
                            }
                        },
                        class: "bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-600",
                        "Submit"
                    }
                }
                }
            }
        }
    }
}

#[server(BlogFormServer)]
pub async fn blog_form_server(title: String, content: String) -> Result<String, ServerFnError> {
    Ok(format!("Title: {}, Content: {}", title, content))
}
