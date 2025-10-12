use dioxus::prelude::*;

mod backend_connector;
use backend_connector::{add_todo, delete_todo, get_todos, mark_done, mark_undone, Todo};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const HEADING_STYLE: &str = "heading font-black font-mono text-teal-300 text-5xl";
const BUTTON_STYLE: &str =
    "border-0 rounded-full bg-blue-400 px-4 py-2 hover:scale-115 hover:transition hover:ease-in-out";
const TODO_LIST_STYLE: &str = "";
const TODO_ADD_STYLE: &str = "grid items-center";
const CHECKBOX_FORMATTING: &str =
    "hover:scale-125 hover:transition hover:ease-in-out checked:accent-teal-500 ";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let is_add: Signal<bool> = use_signal(|| false);
    let todos: Resource<Vec<Todo>> = use_resource(move || get_todos());
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "h-screen text-white flex justify-center p-5 bg-slate-900",
            if !is_add() {
                Home { is_add, todos }
            } else {
                Add { is_add, todos }
            }
        }
    }
}

#[component]
fn Home(is_add: Signal<bool>, todos: Resource<Vec<Todo>>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-5",
            div { class: HEADING_STYLE,
                h1 { "To-Do" }
            }
            div { class: TODO_LIST_STYLE,
                ul {
                    match &*todos.read() {
                        Some(todos_vec) => {
                            if !todos_vec.is_empty() {
                                let todo_elements = todos_vec
                                    .iter()
                                    .map(|todo| {
                                        let id = match todo.id.clone() {
                                            Some(x) => x,
                                            None => panic!("ID not found for task {}", todo.name),
                                        };
                                        let del_id = id.clone();
                                        let is_done = todo.is_done;
                                        let name = todo.name.clone();
                                        rsx! {
                                            li {
                                                input {
                                                    class: CHECKBOX_FORMATTING,
                                                    r#type: "checkbox",
                                                    checked: is_done,
                                                    oninput: move |_| {
                                                        let id = id.clone();
                                                        async move {
                                                            if is_done {
                                                                mark_undone(id.clone()).await;
                                                            } else {
                                                                mark_done(id.clone()).await;
                                                            }
                                                            todos.restart();
                                                        }
                                                    },
                                                }
                                                "{name}"
                                                button {
                                                    class: "bg-red-500 m-10 max-w-3xs max-h-3xs {BUTTON_STYLE}",
                                                    onclick: move |_| {
                                                        let id = del_id.clone();
                                                        async move {
                                                            delete_todo(id.clone()).await;
                                                            todos.restart();
                                                        }
                                                    },
                                                    "Delete"
                                                }
                                            }
                                        }
                                    });
                                rsx! {
                                    {todo_elements}
                                }
                            } else {
                                rsx! {
                                    li { "No tasks found" }
                                }
                            }
                        }
                        None => rsx! {
                            li { "Loading" }
                        },
                    }
                }
            }
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {
                    is_add.set(true);
                },
                "Add"
            }
        }
    }
}

#[component]
fn Add(is_add: Signal<bool>, todos: Resource<Vec<Todo>>) -> Element {
    let mut info = use_signal(|| String::new());
    let mut new_todo_name = use_signal(|| String::new());
    let mut new_todo_desc = use_signal(|| String::new());
    let mut new_todo_due = use_signal(|| String::new());
    let mut new_todo_imp = use_signal(|| String::new());
    let mut new_todo_req_time = use_signal(|| String::new());
    rsx! {
        div { class: "flex flex-col items-center gap-5",
            div { class: HEADING_STYLE,
                h1 { "Add a task" }
            }
            div { class: "info", "{info}" }
            form {
                class: TODO_ADD_STYLE,
                onsubmit: move |_| async move {
                    add_todo(
                            new_todo_name.read().clone(),
                            new_todo_desc.read().clone(),
                            new_todo_due.read().clone(),
                            new_todo_req_time.read().clone(),
                            new_todo_imp.read().clone(),
                        )
                        .await;
                    info.set(format!("New task added with name {new_todo_name}"));
                    new_todo_name.set(String::new());
                    new_todo_desc.set(String::new());
                    new_todo_due.set(String::new());
                    new_todo_imp.set(String::new());
                    new_todo_req_time.set(String::new());
                    todos.restart();
                },
                label {
                    "Task Name: "
                    input {
                        r#type: "text",
                        placeholder: "Task Name",
                        value: "{new_todo_name}",
                        oninput: move |e| new_todo_name.set(e.value()),
                    }
                }
                label {
                    "Task Description: "
                    textarea {
                        placeholder: "Description",
                        value: "{new_todo_desc}",
                        oninput: move |e| new_todo_desc.set(e.value()),
                    }
                }
                label {
                    "Importance Level: "
                    input {
                        r#type: "range",
                        min: 1,
                        max: 10,
                        value: "{new_todo_imp}",
                        oninput: move |e| new_todo_imp.set(e.value()),
                    }
                }
                label {
                    "Duration required: "
                    input {
                        r#type: "text",
                        value: "{new_todo_req_time}",
                        oninput: move |e| new_todo_req_time.set(e.value()),
                    }
                }
                label {
                    "Due by: "
                    input {
                        r#type: "datetime-local",
                        value: "{new_todo_due}",
                        oninput: move |e| new_todo_due.set(e.value()),
                    }
                }
                button { class: "bg-yellow-400 {BUTTON_STYLE}", r#type: "submit", "Submit" }
            }
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {
                    is_add.set(false);
                    todos.restart();
                },
                "Go Back"
            }
        }
    }
}
