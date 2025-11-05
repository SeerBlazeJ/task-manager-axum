use std::str::FromStr;

use chrono::{Datelike, Days, NaiveDate, Utc};
use dioxus::prelude::*;

mod backend_helper;
use backend_helper::{
    add_sched, add_todo, delete_todo, get_day_todos, get_todos, mark_done, mark_undone, Todo,
};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const HEADING_STYLE: &str = "heading font-black font-mono text-teal-300 text-5xl text-center";
const BUTTON_STYLE: &str =
    "border-0 rounded-full bg-blue-400 px-4 py-2 hover:scale-115 hover:transition hover:ease-in-out";
const TODO_LIST_STYLE: &str = "";
const TODO_ADD_STYLE: &str = "grid items-center";
const CHECKBOX_FORMATTING: &str =
    "hover:scale-125 hover:transition hove cd projects/rust/task-manager  r:ease-in-out checked:accent-teal-500  ";
const NAV_BTN_CLASS: &str =
    "hover:cursor-pointer hover:transition hover:ease-in-out hover:scale-130 ";

#[derive(Routable, Clone, PartialEq)]
enum Router {
    #[route("/")]
    App,
    #[route("/day/:date")]
    DateInfo { date: String },
}

// NEXT: Add a static schedule and make it reflect in the schedules

fn main() {
    dioxus::launch(RouteHandler);
}

#[component]
fn RouteHandler() -> Element {
    rsx!(Router::<Router> {})
}

// Base app interface
#[component]
fn App() -> Element {
    let is_add_task: Signal<bool> = use_signal(|| false);
    let open_sched_editor: Signal<bool> = use_signal(|| false);
    let todos = use_resource(get_todos);
    let current_year = use_signal(|| Utc::now().year());
    let current_month = use_signal(|| Utc::now().month());

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "h-screen text-white justify-center p-5 bg-slate-900",
            if is_add_task() {
                AddTodo { is_add_task, todos }
            } else if open_sched_editor() {
                SchedEditor { open_sched_editor }
            } else {
                div { class: "flex flex-row gap-8",
                    div { class: "w-1/2",
                        div { class: HEADING_STYLE,
                            h1 { "Tasks" }
                        }
                        Home {
                            is_add_task,
                            open_sched_editor,
                            todos,
                        }
                    }
                    div { class: "w-1/2",
                        div { class: HEADING_STYLE,
                            h1 { "Schedule" }
                        }
                        Calendar { current_month, current_year }
                    }
                }
            }
        }
    }
}

// Homepage - render a list of tasks
#[component]
fn Home(
    is_add_task: Signal<bool>,
    open_sched_editor: Signal<bool>,
    todos: Resource<Vec<Todo>>,
) -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-5",
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
                    is_add_task.set(true);
                },
                "Add Task"
            }
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {
                    open_sched_editor.set(true);
                },
                "Modify Schedule"
            }
        }
    }
}

// Form to add a new task
#[component]
fn AddTodo(is_add_task: Signal<bool>, todos: Resource<Vec<Todo>>) -> Element {
    let mut info = use_signal(String::new);
    let mut new_todo_name = use_signal(String::new);
    let mut new_todo_desc = use_signal(String::new);
    let mut new_todo_due = use_signal(String::new);
    let mut new_todo_imp = use_signal(String::new);
    let mut new_todo_req_time = use_signal(String::new);
    rsx! {
        div { class: "flex flex-col items-center gap-5",
            div { class: HEADING_STYLE,
                h1 { "Add a task" }
            }
            div { class: "info", "{info}" }
            form {
                class: TODO_ADD_STYLE,
                onsubmit: move |_| async move {
                    match add_todo(
                            new_todo_name.read().clone(),
                            new_todo_desc.read().clone(),
                            new_todo_due.read().clone(),
                            new_todo_req_time.read().clone(),
                            new_todo_imp.read().clone(),
                        )
                        .await
                    {
                        Ok(_) => info.set(format!("Task added with name {}", new_todo_name.read())),
                        Err(_) => info.set("An Error occured while adding the task".to_string()),
                    };
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
                button {
                    class: "bg-yellow-400 {BUTTON_STYLE} disabled:cursor-not-allowed disabled:bg-neutral-600",
                    disabled: "{ new_todo_name.read().is_empty() ||
                    new_todo_desc.read().is_empty() ||
                    new_todo_imp.read().is_empty()||
                    new_todo_req_time.read().is_empty()||
                    new_todo_due.read().is_empty() }",
                    r#type: "submit",
                    "Submit"
                }
            }
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {
                    is_add_task.set(false);
                    todos.restart();
                },
                "Go Back"
            }
        }
    }
}

// Calender UI
#[component]
fn Calendar(current_month: Signal<u32>, current_year: Signal<i32>) -> Element {
    let curr_month = *current_month.read();
    let curr_year = *current_year.read();
    let current_date = NaiveDate::from_ymd_opt(curr_year, curr_month, 1).unwrap();

    let year = current_date.year();
    let month = current_date.month();

    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let starting_weekday_offset = first_day_of_month.weekday().num_days_from_sunday();
    let days_in_month = first_day_of_month.num_days_in_month();
    let month_year_str = current_date.format("%B %Y").to_string();

    rsx! {
        div { class: "max-w-md mx-auto p-5 font-sans",
            div { class: "flex justify-between items-center text-center text-xl font-bold mb-5",
                button {
                    class: NAV_BTN_CLASS,
                    onclick: move |_| {
                        if curr_month == 1 {
                            current_year.set(curr_year - 1);
                            current_month.set(12);
                        } else {
                            current_month.set(curr_month - 1);
                        };
                    },
                    "←"
                }
                h2 { "{month_year_str}" }
                button {
                    class: NAV_BTN_CLASS,
                    onclick: move |_| {
                        if curr_month == 12 {
                            current_year.set(curr_year + 1);
                            current_month.set(1);
                        } else {
                            current_month.set(curr_month + 1);
                        };
                    },
                    "→"
                }
            }

            div { class: "grid grid-cols-7 gap-3",
                for day in ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] {
                    div { class: "text-center font-semibold text-md", "{day}" }
                }
                for _ in 0..starting_weekday_offset {
                    div {}
                }
                for day in 1..=days_in_month {
                    button {
                        onclick: move |_| {
                            let date_str = NaiveDate::from_ymd_opt(year, month, day as u32)
                                .unwrap()
                                .to_string();
                            let _ = navigator().push(format!("/day/{}", date_str));
                        },
                        div { class: "text-center p-3 border rounded-lg hover:bg-gray-700 hover:cursor-pointer",
                            "{day}"
                        }
                    }
                }
            }
        }
    }
}

// Date-specific page loader
#[component]
fn DateInfo(date: String) -> Element {
    let attempted_to_date = NaiveDate::from_str(&date);
    let todos = use_resource(move || {
        let date = date.clone();
        async move { get_day_todos(&date).await }
    });
    if let Ok(parsed_date) = attempted_to_date {
        let date_string = parsed_date.format("%A, %B %-d, %Y").to_string();
        rsx! {
            div { class: "text-white justify-center p-5 bg-slate-900",
                h1 { class: "{HEADING_STYLE} text-2xl", "{date_string}" }
                table { class: " block text-xl border-b-2 border-b-slate-100",
                    tr { class: "block border-b-2",
                        th { class: " p-4 text-left border-r-2", "Time" }
                        th { class: " p-4 text-center", "Tasks" }
                    }
                    for x in 00..24 {
                        tr { class: "text-lg",
                            td { class: "p-2 border-r-2 text-right border-b-1 border-b-slate-500",
                                "{x} - {x+1}"
                            }
                            for vec_todo in todos.read().clone().into_iter() {
                                for todo in vec_todo {
                                    if x != 23 {
                                        if todo.due_by > parsed_date.and_hms_opt(x, 0, 0).expect("Couldn't parse into datetime")
                                            && todo.due_by
                                                < parsed_date
                                                    .and_hms_opt(x + 1, 0, 0)
                                                    .expect("Couldn't parse into datetime")
                                        {
                                            td { class: "p-2 text-left border-b-1 border-b-slate-500 border-r-1 border-r-slate-400 border-t-1 border-t-slate-400",
                                                {todo.name}
                                            }
                                        }
                                    } else {
                                        if todo.due_by
                                            > parsed_date
                                                .and_hms_opt(x, 0, 0)
                                                .expect("Failed to parse last hour into datetime")
                                            && todo.due_by
                                                < parsed_date
                                                    .checked_add_days(Days::new(1))
                                                    .unwrap()
                                                    .and_hms_opt(0, 0, 0)
                                                    .expect("Failed to parse to the start of next day")
                                        {
                                            td { class: "p-2 text-left border-b-1 border-b-slate-500 border-r-1 border-r-slate-400 border-u-1 border-u-slate-400",
                                                {todo.name}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                br {}
                button {
                    class: BUTTON_STYLE,
                    onclick: move |_| {
                        navigator().push("/");
                    },
                    "Go to homepage"
                }
            }
        }
    } else {
        navigator().go_back();
        rsx!("Invalid date format entered, redirecting you back...")
    }
}

// Editor interface for managing schedules, current implementation supports only adding and that too is not yet reflected in the server database

#[component]
fn SchedEditor(open_sched_editor: Signal<bool>) -> Element {
    let mut info = use_signal(String::new);
    let mut new_scheditem_title = use_signal(String::new);
    let mut new_scheditem_start_date = use_signal(String::new);
    let mut new_scheditem_end_date = use_signal(String::new);
    let mut new_scheditem_imp = use_signal(String::new);
    let mut new_scheditem_time_start = use_signal(String::new);
    let mut new_scheditem_time_end = use_signal(String::new);
    let mut new_scheditem_weekdays: Signal<Vec<String>> = use_signal(Vec::new);
    rsx! {
        div { class: "flex flex-col items-center gap-5",
            div { class: HEADING_STYLE,
                h1 { "Add a task" }
            }
            div { class: "info", "{info}" }
            form {
                class: TODO_ADD_STYLE,
                onsubmit: move |_| async move {
                    match add_sched(
                            new_scheditem_title.read().clone(),
                            new_scheditem_start_date.read().clone(),
                            new_scheditem_end_date.read().clone(),
                            new_scheditem_imp.read().clone(),
                            new_scheditem_time_start.read().clone(),
                            new_scheditem_time_end.read().clone(),
                            new_scheditem_weekdays.read().clone(),
                        )
                        .await
                    {
                        Ok(_) => {
                            info.set(format!("Task added with name {}", new_scheditem_title.read()))
                        }
                        Err(_) => info.set("An Error occured while adding the task".to_string()),
                    };
                    new_scheditem_title.set(String::new());
                    new_scheditem_start_date.set(String::new());
                    new_scheditem_end_date.set(String::new());
                    new_scheditem_imp.set(String::new());
                    new_scheditem_time_start.set(String::new());
                    new_scheditem_time_start.set(String::new());
                    new_scheditem_weekdays.set(Vec::new());
                },
                label {
                    "Title: "
                    input {
                        r#type: "text",
                        placeholder: "e.g. Office hours",
                        value: "{new_scheditem_title}",
                        oninput: move |e| new_scheditem_title.set(e.value()),
                    }
                }
                label {
                    "Start Date: "
                    input {
                        r#type: "date",
                        oninput: move |date| new_scheditem_start_date.set(date.value()),
                    }
                }
                label {
                    "End Date: "
                    input {
                        r#type: "date",
                        oninput: move |date| new_scheditem_end_date.set(date.value()),
                    }
                }
                label {
                    "Importance Level: "
                    input {
                        r#type: "range",
                        min: 1,
                        max: 10,
                        value: "{new_scheditem_imp}",
                        oninput: move |e| new_scheditem_imp.set(e.value()),
                    }
                }
                label {
                    "Duration required: "
                    input {
                        r#type: "text",
                        value: "{new_scheditem_time_start}",
                        oninput: move |e| new_scheditem_time_start.set(e.value()),
                    }
                }
                label {
                    "Due by: "
                    input {
                        r#type: "datetime-local",
                        value: "{new_scheditem_time_end}",
                        oninput: move |e| new_scheditem_time_end.set(e.value()),
                    }
                }
                button {
                    class: "bg-yellow-400 {BUTTON_STYLE} disabled:cursor-not-allowed disabled:bg-neutral-600",
                    disabled: "{ new_scheditem_title.read().is_empty() ||
                    new_scheditem_start_date.read().is_empty() ||
                    new_scheditem_end_date.read().is_empty()||
                    new_scheditem_imp.read().is_empty()||
                    new_scheditem_time_start.read().is_empty() || new_scheditem_time_start.read().is_empty() }",
                    r#type: "submit",
                    "Submit"
                }
            }
            button {
                class: BUTTON_STYLE,
                onclick: move |_| {
                    open_sched_editor.set(false);
                },
                "Go Back"
            }
        }
    }
}
