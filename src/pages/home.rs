use dioxus::prelude::*;

use crate::components::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 text-white",
            // Hero Section
            section { class: "relative h-screen flex items-center justify-center overflow-hidden",
                div { class: "absolute inset-0 bg-gradient-to-r from-blue-600/20 to-purple-600/20 animate-pulse" }
                div { class: "container mx-auto px-4 z-10",
                    div { class: "text-center",
                        h1 { class: "text-5xl md:text-7xl font-bold mb-6 animate-fade-in",
                            "Transform Your Digital Future"
                        }
                        p { class: "text-xl md:text-2xl text-gray-300 mb-8 animate-slide-up",
                            "Innovative solutions for modern businesses"
                        }
                        button {
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-full transform transition-all duration-300 hover:scale-105 animate-bounce",
                            "Get Started"
                        }
                    }
                }
            }

            // Features Section
            section { class: "py-20 bg-gray-800",
                div { class: "container mx-auto px-4",
                    h2 { class: "text-4xl font-bold text-center mb-16", "Why Choose Us" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                        // Feature 1
                        div { class: "bg-gray-700 p-6 rounded-lg transform transition-all duration-300 hover:scale-105",
                            div { class: "text-blue-500 text-4xl mb-4", "🚀" }
                            h3 { class: "text-xl font-bold mb-2", "Lightning Fast" }
                            p { class: "text-gray-300", "Experience blazing-fast performance with our optimized solutions." }
                        }
                        // Feature 2
                        div { class: "bg-gray-700 p-6 rounded-lg transform transition-all duration-300 hover:scale-105",
                            div { class: "text-blue-500 text-4xl mb-4", "🛡️" }
                            h3 { class: "text-xl font-bold mb-2", "Secure" }
                            p { class: "text-gray-300", "Enterprise-grade security to protect your valuable data." }
                        }
                        // Feature 3
                        div { class: "bg-gray-700 p-6 rounded-lg transform transition-all duration-300 hover:scale-105",
                            div { class: "text-blue-500 text-4xl mb-4", "💡" }
                            h3 { class: "text-xl font-bold mb-2", "Innovative" }
                            p { class: "text-gray-300", "Cutting-edge technology to stay ahead of the competition." }
                        }
                    }
                }
            }

            // CTA Section
            section { class: "py-20 bg-gradient-to-r from-blue-600 to-purple-600",
                div { class: "container mx-auto px-4 text-center",
                    h2 { class: "text-4xl font-bold mb-8", "Ready to Get Started?" }
                    p { class: "text-xl mb-8 text-gray-100", "Join thousands of satisfied customers today." }
                    button {
                        class: "bg-white text-blue-600 font-bold py-3 px-8 rounded-full transform transition-all duration-300 hover:scale-105",
                        "Contact Us"
                    }
                }
            }
        }
    }
}
