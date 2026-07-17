use crate::components::icons::*;
use leptos::prelude::*;

#[component]
pub fn TaskFilter(
    search_query: (ReadSignal<String>, WriteSignal<String>),
    selected_priority: (ReadSignal<String>, WriteSignal<String>),
    sort_by: (ReadSignal<String>, WriteSignal<String>),
    on_clear_completed: Callback<()>,
    completed_count: Memo<usize>,
) -> impl IntoView {
    let (search, set_search) = search_query;
    let (priority, set_priority) = selected_priority;
    let (sort, set_sort) = sort_by;

    view! {
        <div id="filter-wrapper" class="bg-white rounded-xl border border-[#E5E7EB] p-4 mb-6 shadow-xs flex flex-col sm:flex-row items-center gap-4 justify-between">
            <div class="relative w-full sm:max-w-md">
                <span class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-[#9CA3AF]">
                    {move || icon_search()}
                </span>
                <input id="search-input" type="text" prop:value=search
                    on:input=move |ev| set_search.set(event_target_value(&ev))
                    placeholder="Search engineering sprint tasks..."
                    class="w-full pl-9 pr-4 py-1.5 bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg text-[#111827] text-xs placeholder-[#9CA3AF] focus:outline-none focus:ring-2 focus:ring-[#2563EB]/15 focus:border-[#2563EB] transition-all"
                />
                <Show when=move || !search.get().is_empty() fallback=|| ()>
                    <button on:click=move |_| set_search.set(String::new())
                        class="absolute inset-y-0 right-0 pr-3 flex items-center text-[#9CA3AF] hover:text-[#4B5563] text-xs font-semibold cursor-pointer"
                    >
                        "Clear"
                    </button>
                </Show>
            </div>

            <div id="filter-selectors" class="flex flex-wrap items-center gap-3 w-full sm:w-auto justify-end">
                <div class="flex items-center gap-2">
                    <span class="text-[11px] font-medium text-[#4B5563] flex items-center gap-1">
                        {move || icon_sliders_horizontal()} " Priority:"
                    </span>
                    <select id="priority-filter-select" prop:value=priority
                        on:change=move |ev| set_priority.set(event_target_value(&ev))
                        class="bg-white border border-[#E5E7EB] text-[#4B5563] rounded-lg px-2 py-1 text-xs font-medium focus:outline-none focus:ring-2 focus:ring-[#2563EB]/15 focus:border-[#2563EB] cursor-pointer"
                    >
                        <option value="all">"All Priorities"</option>
                        <option value="high">"High"</option>
                        <option value="medium">"Medium"</option>
                        <option value="low">"Low"</option>
                    </select>
                </div>

                <div class="flex items-center gap-2">
                    <span class="text-[11px] font-medium text-[#4B5563] flex items-center gap-1">
                        {move || icon_arrow_up_down()} " Sort:"
                    </span>
                    <select id="sort-select" prop:value=sort
                        on:change=move |ev| set_sort.set(event_target_value(&ev))
                        class="bg-white border border-[#E5E7EB] text-[#4B5563] rounded-lg px-2 py-1 text-xs font-medium focus:outline-none focus:ring-2 focus:ring-[#2563EB]/15 focus:border-[#2563EB] cursor-pointer"
                    >
                        <option value="newest">"Newest"</option>
                        <option value="oldest">"Oldest"</option>
                        <option value="dueDate">"Due Date"</option>
                        <option value="priority">"Priority"</option>
                        <option value="alphabetical">"A-Z"</option>
                    </select>
                </div>

                <Show when=move || completed_count.get() != 0 fallback=|| ()>
                    <button id="clear-completed-btn"
                        on:click=move |_| on_clear_completed.run(())
                        class="flex items-center gap-1 px-3 py-1 bg-red-50 hover:bg-red-100 text-red-600 border border-red-100 rounded-lg text-xs font-medium transition-all active:scale-95 cursor-pointer"
                    >
                        {move || icon_trash2()}
                        {move || format!(" Clear ({})", completed_count.get())}
                    </button>
                </Show>
            </div>
        </div>
    }
}
