use crate::Route;
use dioxus::prelude::*;

#[derive(Clone)]
struct User {
    id: i32,
    name: String,
    email: String,
    role: String,
    status: String,
    last_active: String,
}

#[component]
pub fn UsersManagement() -> Element {
    // Sample data (in a real app, this would come from your backend)
    let users = vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            role: "Admin".to_string(),
            status: "Active".to_string(),
            last_active: "2 minutes ago".to_string(),
        },
        User {
            id: 2,
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            role: "Editor".to_string(),
            status: "Active".to_string(),
            last_active: "1 hour ago".to_string(),
        },
        User {
            id: 3,
            name: "Bob Wilson".to_string(),
            email: "bob@example.com".to_string(),
            role: "User".to_string(),
            status: "Inactive".to_string(),
            last_active: "2 days ago".to_string(),
        },
    ];

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            // Top Navigation
            nav { class: "bg-white shadow-sm",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "flex justify-between h-16",
                        div { class: "flex",
                            div { class: "flex-shrink-0 flex items-center",
                                h1 { class: "text-xl font-bold text-gray-900", "User Management" }
                            }
                        }
                        div { class: "flex items-center space-x-4",
                            Link {
                                to: Route::AdminDashboard {},
                                class: "text-gray-600 hover:text-gray-900",
                                "Dashboard"
                            }
                            Link {
                                to: Route::Home {},
                                class: "text-gray-600 hover:text-gray-900",
                                "Back to Site"
                            }
                        }
                    }
                }
            }

            // Main Content
            main { class: "max-w-7xl mx-auto py-6 sm:px-6 lg:px-8",
                // Users Table
                div { class: "bg-white shadow overflow-hidden sm:rounded-lg",
                    div { class: "px-4 py-5 sm:px-6 flex justify-between items-center",
                        h3 { class: "text-lg leading-6 font-medium text-gray-900",
                            "Users"
                        }
                        button {
                            class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500",
                            "Add User"
                        }
                    }
                    div { class: "border-t border-gray-200",
                        table { class: "min-w-full divide-y divide-gray-200",
                            thead { class: "bg-gray-50",
                                tr {
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Name"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Email"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Role"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Status"
                                    }
                                    th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Last Active"
                                    }
                                    th { class: "px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider",
                                        "Actions"
                                    }
                                }
                            }
                            tbody { class: "bg-white divide-y divide-gray-200",
                                {users.into_iter().map(|user| {
                                    rsx! {
                                        tr {
                                            td { class: "px-6 py-4 whitespace-nowrap",
                                                div { class: "flex items-center",
                                                    div { class: "h-10 w-10 flex-shrink-0",
                                                        div { class: "h-10 w-10 rounded-full bg-gray-400" }
                                                    }
                                                    div { class: "ml-4",
                                                        div { class: "text-sm font-medium text-gray-900",
                                                            "{user.name}"
                                                        }
                                                    }
                                                }
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap",
                                                div { class: "text-sm text-gray-900",
                                                    "{user.email}"
                                                }
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap",
                                                div { class: "text-sm text-gray-900",
                                                    "{user.role}"
                                                }
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap",
                                                span {
                                                    class: format!("px-2 inline-flex text-xs leading-5 font-semibold rounded-full {}",
                                                        if user.status == "Active" { "bg-green-100 text-green-800" } else { "bg-red-100 text-red-800" }
                                                    ),
                                                    "{user.status}"
                                                }
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                                "{user.last_active}"
                                            }
                                            td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                                button { class: "text-blue-600 hover:text-blue-900 mr-4",
                                                    "Edit"
                                                }
                                                button { class: "text-red-600 hover:text-red-900",
                                                    "Delete"
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
