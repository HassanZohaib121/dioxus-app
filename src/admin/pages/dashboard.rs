use crate::admin::components::AdminLayout;
use dioxus::prelude::*;

#[component]
pub fn AdminDashboard() -> Element {
    // Sample data (in a real app, this would come from your backend)
    let stats = vec![
        ("Total Users", "1,234", "↑ 12%", "bg-blue-500"),
        ("Active Sessions", "56", "↑ 8%", "bg-green-500"),
        ("Total Posts", "89", "↑ 5%", "bg-purple-500"),
        ("Comments", "456", "↑ 15%", "bg-yellow-500"),
    ];

    rsx! {
            AdminLayout {
                title: "Dashboard".to_string(),
                // Stats Grid
                div { class: "grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4",
                    {stats.into_iter().map(|(title, value, change, color)| {
                        rsx! {
                            div { class: "bg-white overflow-hidden shadow rounded-lg",
                                div { class: "p-5",
                                    div { class: "flex items-center",
                                        div { class: "flex-shrink-0",
                                            div { class: "rounded-md p-3 {color}",
                                                // You can add icons here
                                            }
                                        }
                                        div { class: "ml-5 w-0 flex-1",
                                            dl {
                                                dt { class: "text-sm font-medium text-gray-500 truncate",
                                                    "{title}"
                                                }
                                                dd { class: "flex items-baseline",
                                                    div { class: "text-2xl font-semibold text-gray-900",
                                                        "{value}"
                                                    }
                                                    div { class: "ml-2 flex items-baseline text-sm font-semibold text-green-600",
                                                        "{change}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })}
                }

                // Recent Activity
                div { class: "mt-8",
                    div { class: "bg-white shadow rounded-lg",
                        div { class: "px-4 py-5 sm:px-6",
                            h3 { class: "text-lg leading-6 font-medium text-gray-900",
                                "Recent Activity"
                            }
                        }
                        div { class: "border-t border-gray-200",
                            div { class: "px-4 py-5 sm:p-6",
                                // Sample activity items
                                div { class: "flow-root",
                                    ul { class: "-mb-8",
                                        {vec![
                                            ("New user registration", "2 minutes ago"),
                                            ("Blog post published", "1 hour ago"),
                                            ("Comment added", "3 hours ago"),
                                            ("User profile updated", "5 hours ago"),
                                        ].into_iter().map(|(action, time)| {
                                            rsx! {
                                                li {
                                                    div { class: "relative pb-8",
                                                        div { class: "relative flex space-x-3",
                                                            div {
                                                                div { class: "h-8 w-8 rounded-full bg-gray-400 flex items-center justify-center ring-8 ring-white" }
                                                            }
                                                            div { class: "min-w-0 flex-1 pt-1.5 flex justify-between space-x-4",
                                                                div {
                                                                    p { class: "text-sm text-gray-500",
                                                                        "{action}"
                                                                    }
                                                                    p { class: "text-sm text-gray-500",
                                                                        "{time}"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        })}
                                    }
                                }
                            }
                        }
                    }
                }
            }

    }
}
