use std::str::FromStr;

use chrono::{Datelike, NaiveDate, NaiveTime, Utc};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

mod backend_helper;
use backend_helper::{
    add_sched, add_todo, delete_todo, get_day_schedule, get_todo_by_id, get_todos, mark_done,
    mark_undone, Routine, Task,
};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// ===== DESIGN SYSTEM CONSTANTS =====

const HEADING_SECONDARY: &str = "font-bold text-teal-400 text-2xl mb-6 animate-fade-in";

const BUTTON_PRIMARY: &str = "
    px-6 py-3 rounded-xl font-semibold
    bg-gradient-to-r from-teal-500 to-teal-600
    hover:from-teal-400 hover:to-teal-500
    text-white shadow-lg hover:shadow-xl
    transform hover:-translate-y-0.5 transition-all duration-200
    focus:outline-none focus:ring-2 focus:ring-teal-500 focus:ring-offset-2 focus:ring-offset-slate-900
    disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:translate-y-0
";

const BUTTON_SECONDARY: &str = "
    px-6 py-3 rounded-xl font-semibold
    bg-slate-800 hover:bg-slate-700 border border-slate-600 hover:border-slate-500
    text-white shadow-md hover:shadow-lg
    transform hover:-translate-y-0.5 transition-all duration-200
    focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-2 focus:ring-offset-slate-900
";

const BUTTON_DANGER: &str = "
    px-4 py-2 rounded-lg font-medium text-sm
    bg-red-500 hover:bg-red-600
    text-white shadow-md hover:shadow-lg
    transform hover:scale-105 transition-all duration-200
    focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:ring-offset-slate-900
";

const BUTTON_ICON: &str = "
    p-2 rounded-lg
    hover:bg-slate-700 text-slate-400 hover:text-white
    transition-all duration-200
    focus:outline-none focus:ring-2 focus:ring-teal-500
";

const CARD_STYLE: &str = "
    bg-slate-800 rounded-xl shadow-xl border border-slate-700
    p-6 transition-all duration-300 hover:shadow-2xl hover:border-slate-600
    animate-fade-in-scale
";

const INPUT_STYLE: &str = "
    w-full px-4 py-2.5 bg-slate-800 border border-slate-700 rounded-lg
    text-white placeholder-slate-400
    focus:outline-none focus:ring-2 focus:ring-teal-500 focus:border-transparent
    transition-all duration-200 hover:border-slate-600
";

const CHECKBOX_STYLE: &str = "
    w-5 h-5 rounded border-2 border-slate-600
    checked:bg-teal-500 checked:border-teal-500
    hover:border-teal-400 hover:scale-110
    transition-all duration-200 cursor-pointer
    focus:outline-none focus:ring-2 focus:ring-teal-500 focus:ring-offset-2 focus:ring-offset-slate-900
";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Routable, Clone, PartialEq)]
enum Router {
    #[route("/")]
    App,
    #[route("/day/:date")]
    DateInfo { date: String },
}

// TODO: Update UI and backend to modify/delete routines

fn main() {
    dioxus::launch(RouteHandler);
}

#[component]
fn RouteHandler() -> Element {
    rsx!(Router::<Router> {})
}

#[component]
fn App() -> Element {
    let is_add_task: Signal<bool> = use_signal(|| false);
    let open_sched_editor: Signal<bool> = use_signal(|| false);
    let curr_task_id: Signal<String> = use_signal(String::new);
    let todos = use_resource(get_todos);
    let current_year = use_signal(|| Utc::now().year());
    let current_month = use_signal(|| Utc::now().month());

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 text-white p-6",
            if is_add_task() {
                AddTodo { is_add_task, todos }
            } else if open_sched_editor() {
                SchedEditor { open_sched_editor }
            } else if !curr_task_id.read().is_empty() {
                Task_details { curr_task_id }
            } else {
                div { class: "max-w-7xl mx-auto",
                    // Main Header
                    div { class: "mb-12 text-center animate-fade-in",
                        h1 { class: "text-6xl font-black mb-3 text-gradient", "TaskFlow" }
                        p { class: "text-slate-400 text-lg", "Organize your life, one task at a time" }
                    }

                    div { class: "grid lg:grid-cols-2 gap-8",
                        // Tasks Section
                        div { class: "space-y-6 animate-slide-in-left",
                            div { class: CARD_STYLE,
                                div { class: "flex items-center justify-between mb-6",
                                    h2 { class: "text-2xl font-bold text-teal-400",
                                        "📋 Your Tasks"
                                    }
                                    div { class: "flex items-center gap-2 text-sm text-slate-400",
                                        span { "🎯" }
                                        match &*todos.read() {
                                            Some(todos_vec) => rsx! {
                                                span { "{todos_vec.len()} tasks" }
                                            },
                                            None => rsx! {
                                                span { "Loading..." }
                                            },
                                        }
                                    }
                                }
                                Home {
                                    is_add_task,
                                    open_sched_editor,
                                    todos,
                                    curr_task_id,
                                }
                            }
                        }

                        // Calendar Section
                        div { class: "space-y-6 animate-slide-in-right",
                            div { class: CARD_STYLE,
                                div { class: "flex items-center mb-6",
                                    h2 { class: "text-2xl font-bold text-teal-400",
                                        "📅 Schedule"
                                    }
                                }
                                Calendar { current_month, current_year }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Home(
    is_add_task: Signal<bool>,
    open_sched_editor: Signal<bool>,
    todos: Resource<Vec<Task>>,
    curr_task_id: Signal<String>,
) -> Element {
    rsx! {
        div { class: "space-y-6",
            // Task List
            div { class: "space-y-3 max-h-[500px] overflow-y-auto pr-2",
                match &*todos.read() {
                    Some(todos_vec) => {
                        if !todos_vec.is_empty() {
                            let todo_elements = todos_vec
                                .iter()
                                .enumerate()
                                .map(|(index, todo)| {
                                    let id = match todo.id.clone() {
                                        Some(x) => x,
                                        None => panic!("ID not found for task {}", todo.name),
                                    };
                                    let del_id = id.clone();
                                    let show_id = id.clone();
                                    let is_done = todo.is_done;
                                    let name = todo.name.clone();
                                    let imp = todo.imp_lvl;
                                    let stagger_class = format!("stagger-{}", (index % 5) + 1);
                                    rsx! {
                                        div {
                                            key: "{id}",
                                            class: "group bg-slate-900/50 hover:bg-slate-900 border border-slate-700 hover:border-teal-500/50 rounded-lg p-4 transition-all duration-200 animate-fade-in {stagger_class}",
                                            div { class: "flex items-center gap-4",
                                                // Checkbox
                                                input {
                                                    class: CHECKBOX_STYLE,
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
                                                // Task Info
                                                button {
                                                    class: "flex-1 text-left group",
                                                    onclick: move |_| {
                                                        let id = show_id.clone();
                                                        curr_task_id.set(id);
                                                    },
                                                    div { class: "flex items-center gap-3",
                                                        span { class: if is_done { "text-slate-500 line-through text-lg" } else { "text-white text-lg font-medium group-hover:text-teal-400 transition-colors duration-200" },
                                                            "{name}"
                                                        }
                                                        // Importance Badge
                                                        if imp >= 7 {
                                                            span { class: "px-2 py-0.5 text-xs font-semibold rounded-full bg-red-500/20 text-red-400 border border-red-500/30",
                                                                "High Priority"
                                                            }
                                                        } else if imp >= 4 {
                                                            span { class: "px-2 py-0.5 text-xs font-semibold rounded-full bg-yellow-500/20 text-yellow-400 border border-yellow-500/30",
                                                                "Medium"
                                                            }
                                                        }
                                                    }
                                                }
                                                // Delete Button
                                                button {
                                                    class: "opacity-0 group-hover:opacity-100 {BUTTON_DANGER}",
                                                    onclick: move |_| {
                                                        let id = del_id.clone();
                                                        async move {
                                                            delete_todo(id.clone()).await;
                                                            todos.restart();
                                                        }
                                                    },
                                                    "🗑️"
                                                }
                                            }
                                        }
                                    }
                                });
                            rsx! {
                                {todo_elements}
                            }
                        } else {
                            rsx! {
                                div { class: "text-center py-12 animate-fade-in",
                                    div { class: "text-6xl mb-4", "📭" }
                                    p { class: "text-slate-400 text-lg mb-2", "No tasks yet!" }
                                    p { class: "text-slate-500 text-sm", "Start by adding your first task below" }
                                }
                            }
                        }
                    }
                    None => rsx! {
                        div { class: "space-y-3",
                            for _ in 0..3 {
                                div { class: "loading-skeleton h-16 rounded-lg" }
                            }
                        }
                    },
                }
            }

            // Action Buttons
            div { class: "flex gap-3 pt-6 border-t border-slate-700",
                button {
                    class: "flex-1 {BUTTON_PRIMARY}",
                    onclick: move |_| {
                        is_add_task.set(true);
                    },
                    "➕ Add Task"
                }
                button {
                    class: "flex-1 {BUTTON_SECONDARY}",
                    onclick: move |_| {
                        open_sched_editor.set(true);
                    },
                    "⏰ Manage Routine"
                }
            }
        }
    }
}

#[component]
fn AddTodo(is_add_task: Signal<bool>, todos: Resource<Vec<Task>>) -> Element {
    let mut info = use_signal(String::new);
    let mut new_todo_name = use_signal(String::new);
    let mut new_todo_desc = use_signal(String::new);
    let mut new_todo_due = use_signal(String::new);
    let mut new_todo_imp = use_signal(|| "5".to_string());
    let mut new_todo_req_time_hours = use_signal(|| 0u8);
    let mut new_todo_req_time_mins = use_signal(|| 0u8);

    rsx! {
        div { class: "max-w-2xl mx-auto animate-fade-in-scale",
            div { class: CARD_STYLE,
                // Header
                div { class: "flex items-center justify-between mb-8",
                    h1 { class: HEADING_SECONDARY, "✨ Create New Task" }
                    button {
                        class: BUTTON_ICON,
                        onclick: move |_| {
                            is_add_task.set(false);
                            todos.restart();
                        },
                        "✕"
                    }
                }

                // Info Message
                if !info.read().is_empty() {
                    div { class: "mb-6 p-4 bg-teal-500/10 border border-teal-500/30 rounded-lg text-teal-400 animate-fade-in",
                        "{info}"
                    }
                }

                // Form
                form {
                    class: "space-y-6",
                    onsubmit: move |_| async move {
                        let formatted_time = format!(
                            "{:02}:{:02}",
                            new_todo_req_time_hours(),
                            new_todo_req_time_mins(),
                        );
                        match add_todo(
                                new_todo_name.read().clone(),
                                new_todo_desc.read().clone(),
                                new_todo_due.read().clone(),
                                formatted_time,
                                new_todo_imp.read().clone(),
                            )
                            .await
                        {
                            Ok(_) => {
                                info.set(
                                    format!("✅ Task '{}' added successfully!", new_todo_name.read()),
                                )
                            }
                            Err(_) => info.set("❌ An error occurred while adding the task".to_string()),
                        };
                        new_todo_name.set(String::new());
                        new_todo_desc.set(String::new());
                        new_todo_due.set(String::new());
                        new_todo_imp.set("5".to_string());
                        new_todo_req_time_hours.set(0);
                        new_todo_req_time_mins.set(0);
                        todos.restart();
                    },

                    // Task Name
                    div { class: "space-y-2",
                        label { class: "block text-sm font-semibold text-slate-300", "Task Name" }
                        input {
                            class: INPUT_STYLE,
                            r#type: "text",
                            placeholder: "e.g., Finish project proposal",
                            value: "{new_todo_name}",
                            oninput: move |e| new_todo_name.set(e.value()),
                        }
                    }

                    // Description
                    div { class: "space-y-2",
                        label { class: "block text-sm font-semibold text-slate-300", "Description" }
                        textarea {
                            class: INPUT_STYLE,
                            placeholder: "Add details about your task...",
                            rows: "4",
                            value: "{new_todo_desc}",
                            oninput: move |e| new_todo_desc.set(e.value()),
                        }
                    }

                    // Importance Level
                    div { class: "space-y-2",
                        label { class: "block text-sm font-semibold text-slate-300",
                            "Importance Level: "
                            span { class: "text-teal-400 font-bold", "{new_todo_imp}/10" }
                        }
                        input {
                            class: "w-full",
                            r#type: "range",
                            min: "1",
                            max: "10",
                            value: "{new_todo_imp}",
                            oninput: move |e| new_todo_imp.set(e.value()),
                        }
                        div { class: "flex justify-between text-xs text-slate-500",
                            span { "Low" }
                            span { "High" }
                        }
                    }

                    // Duration
                    div { class: "space-y-2",
                        label { class: "block text-sm font-semibold text-slate-300", "Estimated Duration" }
                        div { class: "flex items-center gap-3",
                            div { class: "flex-1",
                                input {
                                    class: INPUT_STYLE,
                                    r#type: "number",
                                    min: "0",
                                    max: "99",
                                    placeholder: "Hours",
                                    value: "{new_todo_req_time_hours}",
                                    oninput: move |e| {
                                        if let Ok(val) = e.value().parse::<u8>() {
                                            if val <= 99 {
                                                new_todo_req_time_hours.set(val);
                                            }
                                        }
                                    },
                                }
                            }
                            span { class: "text-slate-400 font-bold", ":" }
                            div { class: "flex-1",
                                input {
                                    class: INPUT_STYLE,
                                    r#type: "number",
                                    min: "0",
                                    max: "59",
                                    placeholder: "Minutes",
                                    value: "{new_todo_req_time_mins}",
                                    oninput: move |e| {
                                        if let Ok(val) = e.value().parse::<u8>() {
                                            if val <= 59 {
                                                new_todo_req_time_mins.set(val);
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }

                    // Due Date
                    div { class: "space-y-2",
                        label { class: "block text-sm font-semibold text-slate-300", "Due Date & Time" }
                        input {
                            class: INPUT_STYLE,
                            r#type: "datetime-local",
                            value: "{new_todo_due}",
                            min: "2024-01-01T00:00",
                            max: "3024-01-01T00:00",
                            oninput: move |e| new_todo_due.set(e.value()),
                        }
                    }

                    // Submit Buttons
                    div { class: "flex gap-3 pt-6",
                        button {
                            class: "flex-1 {BUTTON_PRIMARY}",
                            disabled: "{new_todo_name.read().is_empty() ||
                            new_todo_desc.read().is_empty() ||
                            new_todo_due.read().is_empty()}",
                            r#type: "submit",
                            "✅ Create Task"
                        }
                        button {
                            class: "flex-1 {BUTTON_SECONDARY}",
                            r#type: "button",
                            onclick: move |_| {
                                is_add_task.set(false);
                                todos.restart();
                            },
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Calendar(current_month: Signal<u32>, current_year: Signal<i32>) -> Element {
    let curr_month = *current_month.read();
    let curr_year = *current_year.read();
    let current_date = NaiveDate::from_ymd_opt(curr_year, curr_month, 1).unwrap();
    let today = Utc::now().naive_local().date();

    let year = current_date.year();
    let month = current_date.month();

    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let starting_weekday_offset = first_day_of_month.weekday().num_days_from_sunday();
    let days_in_month = first_day_of_month.num_days_in_month();
    let month_year_str = current_date.format("%B %Y").to_string();

    rsx! {
        div { class: "space-y-6",
            // Month Navigation
            div { class: "flex items-center justify-between",
                button {
                    class: "p-2 hover:bg-slate-700 rounded-lg transition-colors duration-200 text-slate-300 hover:text-white",
                    onclick: move |_| {
                        if curr_month == 1 {
                            current_year.set(curr_year - 1);
                            current_month.set(12);
                        } else {
                            current_month.set(curr_month - 1);
                        };
                    },
                    "◀"
                }
                h3 { class: "text-xl font-bold text-white", "{month_year_str}" }
                button {
                    class: "p-2 hover:bg-slate-700 rounded-lg transition-colors duration-200 text-slate-300 hover:text-white",
                    onclick: move |_| {
                        if curr_month == 12 {
                            current_year.set(curr_year + 1);
                            current_month.set(1);
                        } else {
                            current_month.set(curr_month + 1);
                        };
                    },
                    "▶"
                }
            }

            // Calendar Grid
            div { class: "grid grid-cols-7 gap-2",
                // Weekday Headers
                for day in ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"] {
                    div { class: "text-center font-semibold text-sm text-slate-400 py-2",
                        "{day}"
                    }
                }

                // Empty cells before month starts
                for _ in 0..starting_weekday_offset {
                    div {}
                }

                // Days of month
                for day in 1..=days_in_month {
                    {
                        let date = NaiveDate::from_ymd_opt(year, month, day as u32).unwrap();
                        let is_today = date == today;
                        rsx! {
                            button {
                                onclick: move |_| {
                                    let date_str = date.to_string();
                                    let _ = navigator().push(format!("/day/{}", date_str));
                                },
                                class: if is_today { "w-full aspect-square flex items-center justify-center rounded-lg font-semibold
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            bg-gradient-to-br from-teal-500 to-teal-600 text-white shadow-lg
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            hover:from-teal-400 hover:to-teal-500 hover:scale-110
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            transition-all duration-200 animate-pulse-glow" } else { "w-full aspect-square flex items-center justify-center rounded-lg font-medium
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            bg-slate-900/50 hover:bg-slate-700 text-slate-300 hover:text-white
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            border border-slate-700 hover:border-teal-500/50
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            hover:scale-105 transition-all duration-200" },
                                "{day}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DateInfo(date: String) -> Element {
    let attempted_to_date = NaiveDate::from_str(&date);
    let schedule = use_resource(use_reactive!(|(date,)| async move {
        get_day_schedule(&date).await
    }));

    if let Ok(parsed_date) = attempted_to_date {
        let date_string = parsed_date.format("%A, %B %-d, %Y").to_string();
        rsx! {
            div { class: "min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 text-white p-6",
                div { class: "max-w-5xl mx-auto animate-fade-in",
                    // Header
                    div { class: "mb-8",
                        button {
                            class: "mb-4 {BUTTON_SECONDARY}",
                            onclick: move |_| {
                                navigator().push("/");
                            },
                            "← Back to Home"
                        }
                        h1 { class: "text-4xl font-bold text-teal-400 mb-2", "{date_string}" }
                        p { class: "text-slate-400", "Your schedule for the day" }
                    }

                    // Schedule Table
                    div { class: CARD_STYLE,
                        div { class: "overflow-x-auto",
                            table { class: "w-full",
                                thead {
                                    tr { class: "border-b-2 border-slate-700",
                                        th { class: "p-4 text-left font-semibold text-teal-400 w-32",
                                            "Time"
                                        }
                                        th { class: "p-4 text-left font-semibold text-teal-400",
                                            "Activities"
                                        }
                                    }
                                }
                                tbody {
                                    for x in 0..24 {
                                        tr { class: "border-b border-slate-700 hover:bg-slate-900/50 transition-colors duration-200",
                                            td { class: "p-4 text-slate-400 font-medium",
                                                "{x:02}:00 - {x+1:02}:00"
                                            }
                                            td { class: "p-4",
                                                div { class: "flex flex-wrap gap-2",
                                                    for vec_sched_sub_items in schedule.read().clone().into_iter() {
                                                        for item in vec_sched_sub_items {
                                                            {
                                                                let should_show = item.represented_hour_start == x;
                                                                if should_show {
                                                                    rsx! {
                                                                        for title in item.title {
                                                                            div { class: "px-3 py-1.5 bg-teal-500/20 border border-teal-500/30 rounded-lg text-teal-400 font-medium text-sm",
                                                                                " {title} "
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    rsx! {}
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        navigator().go_back();
        rsx!("Invalid date format entered, redirecting you back...")
    }
}

#[component]
fn SchedEditor(open_sched_editor: Signal<bool>) -> Element {
    let mut info = use_signal(String::new);
    let mut new_scheditem_title = use_signal(String::new);
    let mut new_scheditem_start_date = use_signal(String::new);
    let mut new_scheditem_end_date = use_signal(String::new);
    let mut new_scheditem_imp = use_signal(|| "5".to_string());
    let mut new_scheditem_time_start = use_signal(String::new);
    let mut new_scheditem_time_end = use_signal(String::new);
    let mut new_scheditem_weekdays: Signal<Vec<String>> = use_signal(Vec::new);

    let mut toggle_weekday = move |day: String| {
        let mut weekdays = new_scheditem_weekdays.write();
        if let Some(pos) = weekdays.iter().position(|d| d == &day) {
            weekdays.remove(pos);
        } else {
            weekdays.push(day);
        }
    };

    rsx! {
        div { class: "max-w-2xl mx-auto animate-fade-in-scale",
            div { class: CARD_STYLE,
                // Header
                div { class: "flex items-center justify-between mb-8",
                    h1 { class: HEADING_SECONDARY, "⚡ Create Routine" }
                    button {
                        class: BUTTON_ICON,
                        onclick: move |_| {
                            open_sched_editor.set(false);
                        },
                        "✕"
                    }
                }

                // Info Message
                if !info.read().is_empty() {
                    div { class: "mb-6 p-4 bg-teal-500/10 border border-teal-500/30 rounded-lg text-teal-400 animate-fade-in",
                        "{info}"
                    }
                }

                // Form
                form {
                    class: "space-y-6",
                    onsubmit: move |_| async move {
                        let start_time_split: Vec<u32> = new_scheditem_time_start
                            .read()
                            .split(':')
                            .map(|string| string.parse::<u32>().unwrap())
                            .collect();
                        let end_time_split: Vec<u32> = new_scheditem_time_end
                            .read()
                            .split(':')
                            .map(|string| string.parse::<u32>().unwrap())
                            .collect();
                        let sched_item = Routine {
                            id: None,
                            title: new_scheditem_title.read().clone(),
                            start_date: NaiveDate::parse_from_str(
                                    &new_scheditem_start_date.read(),
                                    "%Y-%m-%d",
                                )
                                .unwrap(),
                            end_date: NaiveDate::parse_from_str(
                                    &new_scheditem_end_date.read(),
                                    "%Y-%m-%d",
                                )
                                .unwrap(),
                            imp: new_scheditem_imp.read().clone(),
                            start_time: NaiveTime::from_hms_opt(
                                    start_time_split[0],
                                    start_time_split[1],
                                    0,
                                )
                                .unwrap(),
                            end_time: NaiveTime::from_hms_opt(end_time_split[0], end_time_split[1], 0)
                                .unwrap(),
                            weekdays: new_scheditem_weekdays.read().clone(),
                        };
                        match add_sched(sched_item).await {
                            Ok(_) => {
                                info.set(
                                    format!(
                                        "✅ Routine '{}' created successfully!",
                                        new_scheditem_title.read(),
                                    ),
                                )
                            }
                            Err(_) => {
                                info.set("❌ An error occurred while creating the routine".to_string())
                            }
                        };
                        new_scheditem_title.set(String::new());
                        new_scheditem_start_date.set(String::new());
                        new_scheditem_end_date.set(String::new());
                        new_scheditem_imp.set("5".to_string());
                        new_scheditem_time_start.set(String::new());
                        new_scheditem_time_end.set(String::new());
                        new_scheditem_weekdays.set(Vec::new());
                    },

                    // Title
                    div { class: "space-y-2",
                        label { class: "block text-sm font-semibold text-slate-300", "Routine Title" }
                        input {
                            class: INPUT_STYLE,
                            r#type: "text",
                            placeholder: "e.g., Morning Workout, Office Hours",
                            value: "{new_scheditem_title}",
                            oninput: move |e| new_scheditem_title.set(e.value()),
                        }
                    }

                    // Date Range
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "space-y-2",
                            label { class: "block text-sm font-semibold text-slate-300",
                                "Start Date"
                            }
                            input {
                                class: INPUT_STYLE,
                                r#type: "date",
                                value: "{new_scheditem_start_date}",
                                oninput: move |date| new_scheditem_start_date.set(date.value()),
                            }
                        }
                        div { class: "space-y-2",
                            label { class: "block text-sm font-semibold text-slate-300",
                                "End Date"
                            }
                            input {
                                class: INPUT_STYLE,
                                r#type: "date",
                                value: "{new_scheditem_end_date}",
                                oninput: move |date| new_scheditem_end_date.set(date.value()),
                            }
                        }
                    }

                    // Time Range
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "space-y-2",
                            label { class: "block text-sm font-semibold text-slate-300",
                                "Start Time"
                            }
                            input {
                                class: INPUT_STYLE,
                                r#type: "time",
                                value: "{new_scheditem_time_start}",
                                oninput: move |e| new_scheditem_time_start.set(e.value()),
                            }
                        }
                        div { class: "space-y-2",
                            label { class: "block text-sm font-semibold text-slate-300",
                                "End Time"
                            }
                            input {
                                class: INPUT_STYLE,
                                r#type: "time",
                                value: "{new_scheditem_time_end}",
                                oninput: move |e| new_scheditem_time_end.set(e.value()),
                            }
                        }
                    }

                    // Importance Level
                    div { class: "space-y-2",
                        label { class: "block text-sm font-semibold text-slate-300",
                            "Importance: "
                            span { class: "text-teal-400 font-bold", "{new_scheditem_imp}/10" }
                        }
                        input {
                            class: "w-full",
                            r#type: "range",
                            min: "1",
                            max: "10",
                            value: "{new_scheditem_imp}",
                            oninput: move |e| new_scheditem_imp.set(e.value()),
                        }
                    }

                    // Weekdays Selection
                    div { class: "space-y-3",
                        label { class: "block text-sm font-semibold text-slate-300", "Repeat on Days" }
                        div { class: "flex flex-wrap gap-2",
                            for (day , label , color) in [
                                ("Sunday", "Sun", "red"),
                                ("Monday", "Mon", "blue"),
                                ("Tuesday", "Tue", "blue"),
                                ("Wednesday", "Wed", "blue"),
                                ("Thursday", "Thu", "blue"),
                                ("Friday", "Fri", "blue"),
                                ("Saturday", "Sat", "blue"),
                            ]
                            {
                                {
                                    let is_selected = new_scheditem_weekdays.read().contains(&day.to_string());
                                    let day_string = day.to_string();
                                    rsx! {
                                        button {
                                            key: "{day}",
                                            r#type: "button",
                                            class: if is_selected { format!(
                                                "px-4 py-2 rounded-lg font-semibold text-sm bg-{}-500 text-white border-2 border-{}-500 transform scale-105 transition-all duration-200 shadow-lg",
                                                color,
                                                color,
                                            ) } else { "px-4 py-2 rounded-lg font-medium text-sm bg-slate-900 text-slate-400 border-2 border-slate-700 hover:border-slate-600 hover:text-white transition-all duration-200"
                                                .to_string() },
                                            onclick: move |_| toggle_weekday(day_string.clone()),
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Submit Buttons
                    div { class: "flex gap-3 pt-6",
                        button {
                            class: "flex-1 {BUTTON_PRIMARY}",
                            disabled: "{ new_scheditem_title.read().is_empty() ||
                            new_scheditem_start_date.read().is_empty() ||
                            new_scheditem_end_date.read().is_empty() ||
                            new_scheditem_time_start.read().is_empty() ||
                            new_scheditem_time_end.read().is_empty() ||
                            new_scheditem_weekdays.read().is_empty() }",
                            r#type: "submit",
                            "✅ Create Routine"
                        }
                        button {
                            class: "flex-1 {BUTTON_SECONDARY}",
                            r#type: "button",
                            onclick: move |_| {
                                open_sched_editor.set(false);
                            },
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn Task_details(curr_task_id: Signal<String>) -> Element {
    let curr_task = use_resource(use_reactive!(|curr_task_id| async move {
        get_todo_by_id(curr_task_id()).await
    }));

    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 flex items-center justify-center px-4",
            div { class: "w-full max-w-2xl animate-fade-in-scale",
                div { class: CARD_STYLE,
                    match &*curr_task.read() {
                        Some(Some(task)) => {
                            let due_formatted = task.due_by.format("%d %b %Y, %H:%M").to_string();
                            let imp_level: u8 = task.imp_lvl;
                            rsx! {
                                // Header
                                div { class: "flex items-center justify-between mb-8",
                                    h1 { class: "text-2xl font-bold text-teal-400", "📋 Task Details" }
                                    button {
                                        class: BUTTON_SECONDARY,
                                        onclick: move |_| curr_task_id.set(String::new()),
                                        "← Back"
                                    }
                                }

                                // Task Name
                                div { class: "mb-6",
                                    div { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide",
                                        "Task"
                                    }
                                    h2 { class: "text-3xl font-bold text-white", "{task.name}" }
                                }

                                // Description
                                div { class: "mb-6 p-4 bg-slate-900/50 rounded-lg border border-slate-700",
                                    div { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide",
                                        "Description"
                                    }
                                    p { class: "text-slate-300 leading-relaxed", "{task.description}" }
                                }

                                // Details Grid
                                div { class: "grid md:grid-cols-2 gap-6 mb-6",
                                    // Due Date
                                    div { class: "p-4 bg-slate-900/50 rounded-lg border border-slate-700",
                                        div { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide",
                                            "📅 Due Date"
                                        }
                                        p { class: "text-white font-medium", "{due_formatted}" }
                                    }

                                    // Required Time
                                    div { class: "p-4 bg-slate-900/50 rounded-lg border border-slate-700",
                                        div { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide",
                                            "⏱️ Time Required"
                                        }
                                        p { class: "text-white font-medium", "{task.req_time}" }
                                    }

                                    // Time Alloted
                                    div { class: "p-4 bg-slate-900/50 rounded-lg border border-slate-700",
                                        div { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide",
                                            "⏳ Time Alloted"
                                        }
                                        p { class: "text-white font-medium", "{task.time_alloted}" }
                                    }

                                    // Importance
                                    div { class: "p-4 bg-slate-900/50 rounded-lg border border-slate-700",
                                        div { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide",
                                            "🎯 Importance"
                                        }
                                        div { class: "flex items-center gap-3",
                                            span { class: if imp_level >= 7 { "px-4 py-1.5 rounded-full text-sm font-semibold bg-red-500/20 text-red-400 border border-red-500/30" } else if imp_level >= 4 { "px-4 py-1.5 rounded-full text-sm font-semibold bg-yellow-500/20 text-yellow-400 border border-yellow-500/30" } else { "px-4 py-1.5 rounded-full text-sm font-semibold bg-green-500/20 text-green-400 border border-green-500/30" },
                                                "Level {imp_level}/10"
                                            }
                                        }
                                    }

                                    // Status
                                    div { class: "p-4 bg-slate-900/50 rounded-lg border border-slate-700",
                                        div { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide",
                                            "✓ Status"
                                        }
                                        span { class: if task.is_done { "px-4 py-1.5 rounded-full text-sm font-semibold bg-emerald-500/20 text-emerald-400 border border-emerald-500/30" } else { "px-4 py-1.5 rounded-full text-sm font-semibold bg-orange-500/20 text-orange-400 border border-orange-500/30" },
                                            if task.is_done {
                                                "✅ Completed"
                                            } else {
                                                "⏳ Pending"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(None) => rsx! {
                            div { class: "flex items-center justify-between mb-8",
                                h1 { class: "text-2xl font-bold text-teal-400", "Task not found for the given id" }
                            }
                            button {
                                class: BUTTON_PRIMARY,
                                onclick: move |_| {
                                    curr_task_id.set(String::new());
                                },
                                "Go Back"
                            }
                        },
                        None => rsx! {
                            div { class: "text-center py-12",
                                div { class: "loading-skeleton h-64 rounded-lg" }
                                p { class: "text-slate-400 mt-4", "Loading task details..." }
                            }
                        },
                    }
                }
            }
        }
    }
}
