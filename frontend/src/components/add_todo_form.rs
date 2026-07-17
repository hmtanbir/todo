use crate::components::icons::*;
use crate::types::*;
use leptos::prelude::*;

#[component]
pub fn AddTodoForm(
    on_add_todo: Callback<NewTodoData>,
    categories: Memo<Vec<String>>,
) -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (priority, set_priority) = signal(TodoPriority::Medium);
    let (category, set_category) = signal("Personal".to_string());
    let (new_category, set_new_category) = signal(String::new());
    let (due_date, set_due_date) = signal(String::new());
    let (show_details, set_show_details) = signal(false);
    let (error, set_error) = signal(String::new());

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error.set(String::new());

        let t = title.get();
        if t.trim().is_empty() {
            set_error.set("Please specify a task title".to_string());
            return;
        }

        let final_category = {
            let nc = new_category.get();
            if !nc.trim().is_empty() {
                nc.trim().to_string()
            } else {
                category.get()
            }
        };

        let desc = description.get();
        let dd = due_date.get();

        on_add_todo.run(NewTodoData {
            title: t.trim().to_string(),
            description: if desc.trim().is_empty() {
                None
            } else {
                Some(desc.trim().to_string())
            },
            priority: priority.get(),
            category: final_category,
            due_date: if dd.is_empty() { None } else { Some(dd) },
        });

        set_title.set(String::new());
        set_description.set(String::new());
        set_priority.set(TodoPriority::Medium);
        set_due_date.set(String::new());
        set_new_category.set(String::new());
        set_show_details.set(false);
    };

    view! {
        <form
            id="add-todo-form"
            on:submit=handle_submit
            class="bg-white rounded-xl border border-[#E5E7EB] p-4 mb-6 shadow-xs flex flex-col gap-3 transition-all"
        >
            <div class="flex gap-2.5 items-center">
                <input
                    id="add-todo-title"
                    type="text"
                    prop:value=title
                    on:input=move |ev| {
                        set_title.set(event_target_value(&ev));
                        if !error.get().is_empty() { set_error.set(String::new()); }
                    }
                    placeholder="Add a task to current sprint..."
                    class="flex-1 bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-4 py-2 text-[#111827] text-xs placeholder-[#9CA3AF] focus:outline-none focus:ring-2 focus:ring-[#2563EB]/15 focus:border-[#2563EB] transition-all"
                />
                <button type="button" on:click=move |_| set_show_details.update(|v| *v = !*v)
                    class={move || if show_details.get() {
                        "px-3 py-2 rounded-lg border text-xs font-semibold flex items-center gap-1 transition-all cursor-pointer bg-[#F3F4F6] border-[#D1D5DB] text-[#111827]"
                    } else {
                        "px-3 py-2 rounded-lg border text-xs font-semibold flex items-center gap-1 transition-all cursor-pointer bg-white hover:bg-[#F9FAFB] border-[#E5E7EB] text-[#4B5563]"
                    }}
                >
                    "Options"
                    {move || if show_details.get() { icon_chevron_up() } else { icon_chevron_down() }}
                </button>
                <button type="submit"
                    class="bg-[#2563EB] hover:bg-[#1D4ED8] text-white rounded-lg px-4 py-2 text-xs font-semibold shadow-xs flex items-center gap-1.5 transition-all active:scale-95 cursor-pointer"
                >
                    {move || icon_plus()}
                    " New Task"
                </button>
            </div>

            <Show when=move || !error.get().is_empty() fallback=|| ()>
                <div class="text-[#EF4444] text-[11px] font-semibold flex items-center gap-1 px-1">
                    {move || icon_alert_circle()}
                    " " {move || error.get()}
                </div>
            </Show>

            <Show when=move || show_details.get() fallback=|| ()>
                <div class="pt-3 grid grid-cols-1 md:grid-cols-3 gap-4 border-t border-[#E5E7EB] mt-2">
                    <div class="md:col-span-2 flex flex-col gap-1.5">
                        <label class="text-[10px] font-bold text-[#9CA3AF] uppercase tracking-wider">"Description"</label>
                        <textarea prop:value=description
                            on:input=move |ev| set_description.set(event_target_value(&ev))
                            placeholder="Task notes, reference info, or subtasks..." rows="3"
                            class="w-full bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-3 py-2 text-[#111827] text-xs placeholder-[#9CA3AF] focus:outline-none focus:ring-2 focus:ring-[#2563EB]/15 focus:border-[#2563EB] transition-all resize-none"
                        />
                    </div>
                    <div class="flex flex-col gap-3">
                        <div class="flex flex-col gap-1.5">
                            <label class="text-[10px] font-bold text-[#9CA3AF] uppercase tracking-wider">"Priority"</label>
                            <div class="grid grid-cols-3 gap-1 bg-[#F9FAFB] p-1 border border-[#E5E7EB] rounded-lg">
                                <For each=|| vec![TodoPriority::High, TodoPriority::Medium, TodoPriority::Low]
                                    key=|p| p.as_str().to_string()
                                    children=move |prio| {
                                        let p = prio;
                                        view! {
                                            <button type="button" on:click=move |_| set_priority.set(p)
                                                class={move || if priority.get() == p {
                                                    match p {
                                                        TodoPriority::High => "py-1 text-[9px] font-bold uppercase tracking-wider rounded-md transition-all cursor-pointer bg-[#EF4444] text-white shadow-xs",
                                                        TodoPriority::Medium => "py-1 text-[9px] font-bold uppercase tracking-wider rounded-md transition-all cursor-pointer bg-[#F59E0B] text-white shadow-xs",
                                                        TodoPriority::Low => "py-1 text-[9px] font-bold uppercase tracking-wider rounded-md transition-all cursor-pointer bg-[#3B82F6] text-white shadow-xs",
                                                    }
                                                } else {
                                                    "py-1 text-[9px] font-bold uppercase tracking-wider rounded-md transition-all cursor-pointer text-[#6B7280] hover:bg-[#E5E7EB]/50"
                                                }}
                                            >
                                                {prio.display()}
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        </div>
                        <div class="flex flex-col gap-1.5">
                            <label class="text-[10px] font-bold text-[#9CA3AF] uppercase tracking-wider">"Category"</label>
                            <div class="flex gap-2">
                                <select prop:value=category on:change=move |ev| set_category.set(event_target_value(&ev))
                                    class="flex-1 bg-[#F9FAFB] border border-[#E5E7EB] text-[#4B5563] rounded-lg px-2 py-1.5 text-xs font-medium focus:outline-none"
                                >
                                    <For each=move || categories.get() key=|c| c.clone() children=move |cat| view! { <option value={cat.clone()}>{cat.clone()}</option> } />
                                </select>
                                <input type="text" prop:value=new_category
                                    on:input=move |ev| set_new_category.set(event_target_value(&ev))
                                    placeholder="Add custom..."
                                    class="w-[100px] bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg px-2 py-1.5 text-[#111827] text-xs placeholder-[#9CA3AF] focus:outline-none"
                                />
                            </div>
                        </div>
                        <div class="flex flex-col gap-1.5">
                            <label class="text-[10px] font-bold text-[#9CA3AF] uppercase tracking-wider">"Due Date"</label>
                            <input type="date" prop:value=due_date
                                on:input=move |ev| set_due_date.set(event_target_value(&ev))
                                class="w-full bg-[#F9FAFB] border border-[#E5E7EB] text-[#4B5563] rounded-lg px-2 py-1.5 text-xs font-medium focus:outline-none"
                            />
                        </div>
                    </div>
                </div>
            </Show>
        </form>
    }
}
