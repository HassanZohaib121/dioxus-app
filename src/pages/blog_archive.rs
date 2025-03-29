use crate::Route;
use dioxus::prelude::*;

#[derive(Clone)]
struct BlogPost {
    id: i32,
    title: String,
    excerpt: String,
    date: String,
    category: String,
    image_url: String,
}

#[component]
pub fn BlogArchive() -> Element {
    // Sample blog posts data
    let posts = vec![
        BlogPost {
            id: 1,
            title: "Getting Started with Rust and Dioxus".to_string(),
            excerpt: "Learn how to build modern web applications using Rust and Dioxus framework.".to_string(),
            date: "March 15, 2024".to_string(),
            category: "Development".to_string(),
            image_url: "https://images.unsplash.com/photo-1518770660439-4636190af475?ixlib=rb-1.2.1&auto=format&fit=crop&w=800&q=80".to_string(),
        },
        BlogPost {
            id: 2,
            title: "The Future of Web Development".to_string(),
            excerpt: "Exploring the latest trends and technologies shaping the future of web development.".to_string(),
            date: "March 10, 2024".to_string(),
            category: "Technology".to_string(),
            image_url: "https://images.unsplash.com/photo-1518770660439-4636190af475?ixlib=rb-1.2.1&auto=format&fit=crop&w=800&q=80".to_string(),
        },
        BlogPost {
            id: 3,
            title: "Building Scalable Applications".to_string(),
            excerpt: "Best practices and patterns for building scalable web applications.".to_string(),
            date: "March 5, 2024".to_string(),
            category: "Architecture".to_string(),
            image_url: "https://images.unsplash.com/photo-1551288049-bebda4e38f71?ixlib=rb-1.2.1&auto=format&fit=crop&w=800&q=80".to_string(),
        },
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8",
            // Header Section
            div { class: "max-w-7xl mx-auto text-center my-18",
                h1 { class: "text-4xl font-bold text-gray-900 mb-4 animate-fade-in",
                    "Blog Archive"
                }
                p { class: "text-xl text-gray-600 max-w-2xl mx-auto animate-slide-up",
                    "Explore our latest articles, tutorials, and insights about technology, development, and more."
                }
            }

            // Blog Posts Grid
            div { class: "max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                {posts.into_iter().map(|post| {
                    rsx! {
                        Link {
                            to: Route::Blog { id: post.id },
                            class: "group bg-white rounded-lg shadow-lg overflow-hidden transform transition-all duration-300 hover:scale-105 hover:shadow-xl",
                            div { class: "relative h-48 overflow-hidden",
                                img {
                                    src: "{post.image_url}",
                                    class: "w-full h-full object-cover transform transition-transform duration-300 group-hover:scale-110"
                                }
                                div { class: "absolute top-4 right-4",
                                    span { class: "bg-blue-600 text-white px-3 py-1 rounded-full text-sm",
                                        "{post.category}"
                                    }
                                }
                            }
                            div { class: "p-6",
                                h2 { class: "text-xl font-semibold text-gray-900 mb-2 group-hover:text-blue-600 transition-colors duration-300",
                                    "{post.title}"
                                }
                                p { class: "text-gray-600 mb-4 line-clamp-3",
                                    "{post.excerpt}"
                                }
                                div { class: "flex items-center text-sm text-gray-500",
                                    span { "{post.date}" }
                                }
                            }
                        }
                    }
                })}
            }

            // Newsletter Section
            div { class: "max-w-3xl mx-auto mt-16 bg-white rounded-lg shadow-lg p-8 text-center",
                h2 { class: "text-2xl font-bold text-gray-900 mb-4",
                    "Subscribe to Our Newsletter"
                }
                p { class: "text-gray-600 mb-6",
                    "Stay updated with our latest articles and insights."
                }
                div { class: "flex gap-4 justify-center",
                    input {
                        r#type: "email",
                        placeholder: "Enter your email",
                        class: "flex-1 max-w-md px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    }
                    button {
                        class: "bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors duration-300",
                        "Subscribe"
                    }
                }
            }
        }
    }
}
