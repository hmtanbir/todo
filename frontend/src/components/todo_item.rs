use crate::components::icons::*;
use crate::types::*;
use leptos::prelude::*;

#[component]
pub fn TodoItem(
    todo: Todo,
    on_toggle: Callback<(String, bool)>,
    on_delete: Callback<String>,
    on_update: Callback<(String, TodoUpdate)>,
    categories: Memo<Vec<String>>,
) -> impl IntoView {
    let (is_editing, set_is_editing) = signal(false);
    let (edit_title, set_edit_title) = signal(todo.title.clone());
    let (edit_desc, set_edit_desc) = signal(todo.description.clone().unwrap_or_default());
    let (edit_priority, set_edit_priority) = signal(todo.priority);
    let (edit_category, set_edit_category) = signal(todo.category.clone());
    let (edit_due_date, set_edit_due_date) = signal(todo.due_date.clone().unwrap_or_default());
    let (is_expanded, set_is_expanded) = signal(false);

    let is_overdue = todo
        .due_date
        .as_deref()
        .map(|d| is_date_overdue(d))
        .unwrap_or(false);

    let priority_border_class = match todo.priority {
        TodoPriority::High => "border-l-4 border-l-[#EF4444]",
        TodoPriority::Medium => "border-l-4 border-l-[#F59E0B]",
        TodoPriority::Low => "border-l-4 border-l-[#3B82F6]",
    };

    let priority_badge_class = match todo.priority {
        TodoPriority::High => "bg-[#FEE2E2] text-[#B91C1C] border-none",
        TodoPriority::Medium => "bg-[#FEF3C7] text-[#D97706] border-none",
        TodoPriority::Low => "bg-[#DBEAFE] text-[#1D4ED8] border-none",
    };

    let completed_class = if todo.completed {
        "opacity-60 bg-[#F9FAFB]/50"
    } else {
        ""
    };
    let title_class = if todo.completed {
        "text-xs font-bold tracking-tight text-[#9CA3AF] font-medium line-through truncate"
    } else {
        "text-xs font-bold tracking-tight text-[#111827] truncate"
    };

    let handle_save = {
        let todo_id = todo.id.clone();
        let on_update = on_update.clone();
        move |_| {
            let t = edit_title.get();
            if t.trim().is_empty() {
                return;
            }
            let desc = edit_desc.get();
            let dd = edit_due_date.get();
            on_update.run((
                todo_id.clone(),
                TodoUpdate {
                    title: Some(t.trim().to_string()),
                    description: Some(if desc.trim().is_empty() {
                        None
                    } else {
                        Some(desc.trim().to_string())
                    }),
                    completed: None,
                    priority: Some(edit_priority.get()),
                    category: Some(edit_category.get()),
                    due_date: Some(if dd.is_empty() { None } else { Some(dd) }),
                },
            ));
            set_is_editing.set(false);
        }
    };

    let handle_cancel = {
        let todo_title = todo.title.clone();
        let todo_description = todo.description.clone();
        let todo_priority = todo.priority;
        let todo_category = todo.category.clone();
        let todo_due_date = todo.due_date.clone();
        move |_| {
            set_edit_title.set(todo_title.clone());
            set_edit_desc.set(todo_description.clone().unwrap_or_default());
            set_edit_priority.set(todo_priority);
            set_edit_category.set(todo_category.clone());
            set_edit_due_date.set(todo_due_date.clone().unwrap_or_default());
            set_is_editing.set(false);
        }
    };

    let render_todo_id = todo.id.clone();

    // Captures for the reactive view closure
    let c_todo_id_edit = todo.id.clone();
    let c_todo_id_toggle = todo.id.clone();
    let c_todo_id_delete = todo.id.clone();
    let c_todo_title = todo.title.clone();
    let c_todo_category = todo.category.clone();
    let c_todo_priority = todo.priority;
    let c_todo_completed = todo.completed;
    let c_todo_due_date = todo.due_date.clone();
    let c_todo_description = todo.description.clone();
    let c_todo_created_at = todo.created_at.clone();
    let c_todo_updated_at = todo.updated_at.clone();

    view! {
        <div id={format!("todo-item-{}", render_todo_id)}
            class={format!("bg-white rounded-xl border border-[#E5E7EB] shadow-xs overflow-hidden transition-all duration-200 hover:shadow-sm {} {} todo-item-enter", priority_border_class, completed_class)}
        >
            {move || if is_editing.get() {
                let edit_id = c_todo_id_edit.clone();
                let save_fn = handle_save.clone();
                let cancel_fn = handle_cancel.clone();
                view! {
                    <div id={format!("editing-todo-item-{}", edit_id)} class="p-4 flex flex-col gap-3">
                        <div class="flex gap-2">
                            <input type="text" prop:value=edit_title
                                on:input=move |ev| set_edit_title.set(event_target_value(&ev))
                                placeholder="Task Title"
                                class="flex-1 bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-3 py-1.5 text-xs font-bold text-[#111827] focus:outline-none focus:ring-2 focus:ring-[#2563EB]/15 focus:border-[#2563EB]"
                            />
                            <div class="flex items-center gap-1">
                                <button on:click=save_fn
                                    class="p-1.5 bg-[#2563EB] text-white rounded-lg hover:bg-[#1D4ED8] transition-all shadow-xs cursor-pointer"
                                    title="Save changes"
                                >
                                    {icon_check_13()}
                                </button>
                                <button on:click=cancel_fn
                                    class="p-1.5 bg-white border border-[#E5E7EB] text-[#4B5563] rounded-lg hover:bg-[#F9FAFB] transition-all cursor-pointer"
                                    title="Cancel"
                                >
                                    {icon_x()}
                                </button>
                            </div>
                        </div>
                        <textarea prop:value=edit_desc
                            on:input=move |ev| set_edit_desc.set(event_target_value(&ev))
                            placeholder="Detailed description..." rows="2"
                            class="w-full bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-3 py-1.5 text-xs text-[#4B5563] focus:outline-none focus:ring-2 focus:ring-[#2563EB]/15 focus:border-[#2563EB] resize-none"
                        />
                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                            <div class="flex flex-col gap-1">
                                <span class="text-[9px] font-bold text-[#9CA3AF] uppercase tracking-wider">"Priority"</span>
                                <select prop:value=move || edit_priority.get().as_str().to_string()
                                    on:change=move |ev| set_priority_val(&set_edit_priority, &event_target_value(&ev))
                                    class="bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-2 py-1 text-xs text-[#4B5563] focus:outline-none"
                                >
                                    <option value="high">"High"</option>
                                    <option value="medium">"Medium"</option>
                                    <option value="low">"Low"</option>
                                </select>
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-[9px] font-bold text-[#9CA3AF] uppercase tracking-wider">"Category"</span>
                                <select prop:value=edit_category
                                    on:change=move |ev| set_edit_category.set(event_target_value(&ev))
                                    class="bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-2 py-1 text-xs text-[#4B5563] focus:outline-none"
                                >
                                    <For each=move || categories.get() key=|c| c.clone() children=move |cat| view! { <option value={cat.clone()}>{cat.clone()}</option> } />
                                </select>
                            </div>
                            <div class="flex flex-col gap-1">
                                <span class="text-[9px] font-bold text-[#9CA3AF] uppercase tracking-wider">"Due Date"</span>
                                <input type="date" prop:value=edit_due_date
                                    on:input=move |ev| set_edit_due_date.set(event_target_value(&ev))
                                    class="bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-2 py-1 text-xs text-[#4B5563] focus:outline-none"
                                />
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                let toggle_id = c_todo_id_toggle.clone();
                let toggle_fn = on_toggle.clone();
                let delete_id = c_todo_id_delete.clone();
                let delete_fn = on_delete.clone();

                let title = c_todo_title.clone();
                let category = c_todo_category.clone();
                let priority = c_todo_priority;
                let completed = c_todo_completed;
                let due_date = c_todo_due_date.clone();

                let desc_show = c_todo_description.is_some();
                let due_show = c_todo_due_date.is_some();
                let desc_render = c_todo_description.clone();

                let created_at = c_todo_created_at.clone();
                let updated_at = c_todo_updated_at.clone();

                view! {
                    <div class="p-4">
                        <div class="flex items-start gap-3">
                            <div class="flex items-center h-6">
                                <button on:click=move |_| toggle_fn.run((toggle_id.clone(), !completed))
                                    class={if completed {
                                        "w-5 h-5 rounded-full border flex items-center justify-center transition-all cursor-pointer bg-[#10B981] border-[#10B981] text-white"
                                    } else {
                                        "w-5 h-5 rounded-full border flex items-center justify-center transition-all cursor-pointer bg-white border-[#D1D5DB] hover:border-[#9CA3AF]"
                                    }}
                                >
                                    {if completed { icon_check() } else { ().into_any() }}
                                </button>
                            </div>

                            <div class="flex-1 min-w-0 cursor-pointer" on:click=move |_| set_is_expanded.update(|v| *v = !*v)>
                                <h3 class={title_class}>{title}</h3>
                                <div class="flex flex-wrap items-center gap-2 mt-1.5">
                                    <span class="flex items-center gap-1 text-[9px] font-bold text-[#6B7280] bg-[#F3F4F6] px-1.5 py-0.5 rounded-md uppercase tracking-wider">
                                        {icon_tag()} " " {category}
                                    </span>
                                    <span class={format!("text-[9px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded-md {}", priority_badge_class)}>
                                        {priority.display()}
                                    </span>
                                    {if let Some(dd) = due_date {
                                        let display_date = format_date_short(&dd);
                                        let due_badge = if is_overdue { "bg-[#FEE2E2] text-[#EF4444]" } else { "bg-[#F3F4F6] text-[#4B5563]" };
                                        view! {
                                            <span class={format!("flex items-center gap-1 text-[9px] font-bold px-1.5 py-0.5 rounded-md uppercase tracking-wider {}", due_badge)}>
                                                {icon_calendar()} " " {display_date}
                                                {if is_overdue { view! { <span class="text-[8px] uppercase tracking-wide ml-0.5 font-bold">" (Overdue)"</span> }.into_any() } else { ().into_any() }}
                                            </span>
                                        }.into_any()
                                    } else {
                                        ().into_any()
                                    }}
                                </div>
                            </div>

                            <div class="flex items-center gap-1">
                                <button on:click=move |_| set_is_expanded.update(|v| *v = !*v)
                                    class="p-1.5 text-[#9CA3AF] hover:text-[#4B5563] rounded-md hover:bg-[#F3F4F6] transition-all cursor-pointer">
                                    {move || if is_expanded.get() { icon_eye_off() } else { icon_eye() }}
                                </button>
                                <button on:click=move |_| { set_is_editing.set(true); set_is_expanded.set(true); }
                                    class="p-1.5 text-[#9CA3AF] hover:text-[#4B5563] rounded-md hover:bg-[#F3F4F6] transition-all cursor-pointer"
                                    title="Edit task">
                                    {icon_edit()}
                                </button>
                                <button on:click=move |_| delete_fn.run(delete_id.clone())
                                    class="p-1.5 text-[#9CA3AF] hover:text-[#EF4444] rounded-md hover:bg-red-50/60 transition-all cursor-pointer"
                                    title="Delete task">
                                    {icon_trash2_sm()}
                                </button>
                            </div>
                        </div>

                        <Show when=move || is_expanded.get() && (desc_show || due_show) fallback=|| ()>
                            <div class="mt-3 pt-3 border-t border-[#E5E7EB]">
                                {let desc = desc_render.clone(); if let Some(d) = desc {
                                    view! {
                                        <div class="text-[11px] text-[#4B5563] leading-relaxed bg-[#F9FAFB] p-2.5 rounded-lg border border-[#E5E7EB] font-medium">
                                            {d}
                                        </div>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}
                                <div class="flex justify-between items-center mt-2.5 text-[9px] text-[#9CA3AF] font-bold uppercase tracking-wider px-0.5">
                                    <span>{format!("Created: {}", format_full_datetime(&created_at))}</span>
                                    {
                                        if updated_at != created_at {
                                            view! { <span>{format!("Updated: {}", format_time_short(&updated_at))}</span> }.into_any()
                                        } else {
                                            ().into_any()
                                        }
                                    }
                                </div>
                            </div>
                        </Show>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

fn set_priority_val(setter: &WriteSignal<TodoPriority>, val: &str) {
    setter.set(TodoPriority::from_str(val));
}
