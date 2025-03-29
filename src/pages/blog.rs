use dioxus::prelude::*;

use crate::Route;

#[derive(Clone)]
struct BlogPost {
    id: i32,
    title: String,
    content: String,
    date: String,
    category: String,
    image_url: String,
    author: String,
}

#[component]
pub fn Blog(id: i32) -> Element {
    // Sample blog post data (in a real app, this would come from a database)
    let post = BlogPost {
        id,
        title: "Getting Started with Rust and Dioxus".to_string(),
        content: "Dioxus is a modern, fast, and powerful framework for building user interfaces in Rust. It provides a React-like experience while leveraging Rust's safety and performance.".to_string(),
        date: "March 15, 2024".to_string(),
        category: "Development".to_string(),
        image_url: "https://images.unsplash.com/photo-1518770660439-4636190af475?ixlib=rb-1.2.1&auto=format&fit=crop&w=800&q=80".to_string(),
        author: "John Doe".to_string(),
    };

    rsx! {
        article { class: "min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8",
            // Hero Section
            div { class: "max-w-4xl mx-auto my-12",
                // Header
                header { class: "mb-8",
                    div { class: "flex items-center justify-between mb-4",
                        Link {
                            to: Route::BlogArchive {},
                            class: "text-blue-600 hover:text-blue-800 flex items-center",
                            span { "← Back to Blog" }
                        }
                        span { class: "text-gray-500", "{post.date}" }
                    }
                    h1 { class: "text-4xl font-bold text-gray-900 mb-4",
                        "{post.title}"
                    }
                    div { class: "flex items-center gap-4",
                        span { class: "bg-blue-600 text-white px-3 py-1 rounded-full text-sm",
                            "{post.category}"
                        }
                        span { class: "text-gray-600",
                            "By {post.author}"
                        }
                    }
                }

                // Featured Image
                div { class: "relative h-[400px] rounded-lg overflow-hidden mb-8",
                    img {
                        src: "{post.image_url}",
                        class: "w-full h-full object-cover"
                    }
                }

                // Content
                div { class: "prose prose-lg max-w-none",
                    p { class: "text-gray-700 leading-relaxed mb-4",
                        "{post.content}"
                    }
                }

                // Navigation
                div { class: "flex justify-between items-center mt-12 pt-8 border-t",
                    match id > 1 {
                        true => rsx! {
                            Link {
                                to: Route::Blog { id: id - 1 },
                                class: "text-blue-600 hover:text-blue-800 flex items-center",
                                span { "← Previous Post" }
                            }
                        },
                        false => rsx! { div {} }
                    }
                    Link {
                        to: Route::BlogArchive {},
                        class: "text-blue-600 hover:text-blue-800",
                        "Back to Archive"
                    }
                    match id < 3 {
                        true => rsx! {
                            Link {
                                to: Route::Blog { id: id + 1 },
                                class: "text-blue-600 hover:text-blue-800 flex items-center",
                                span { "Next Post →" }
                            }
                        },
                        false => rsx! { div {} }
                    }
                }
            }
        }
    }
}
