use crate::components::icons::*;
use crate::types::*;
use leptos::prelude::*;

#[component]
pub fn TaskStats(todos: Memo<Vec<Todo>>) -> impl IntoView {
    let total = Memo::new(move |_| todos.get().len());
    let completed = Memo::new(move |_| todos.get().iter().filter(|t| t.completed).count());
    let active = Memo::new(move |_| total.get() - completed.get());
    let percent = Memo::new(move |_| {
        let t = total.get();
        if t > 0 {
            (completed.get() as f64 / t as f64 * 100.0).round() as i32
        } else {
            0
        }
    });
    let high_priority = Memo::new(move |_| {
        todos
            .get()
            .iter()
            .filter(|t| !t.completed && t.priority == TodoPriority::High)
            .count()
    });
    let medium_priority = Memo::new(move |_| {
        todos
            .get()
            .iter()
            .filter(|t| !t.completed && t.priority == TodoPriority::Medium)
            .count()
    });
    let low_priority = Memo::new(move |_| {
        todos
            .get()
            .iter()
            .filter(|t| !t.completed && t.priority == TodoPriority::Low)
            .count()
    });
    let upcoming_overdue = Memo::new(move |_| {
        todos
            .get()
            .iter()
            .filter(|t| {
                !t.completed
                    && t.due_date
                        .as_deref()
                        .map(is_date_overdue)
                        .unwrap_or(false)
            })
            .count()
    });

    view! {
        <div id="stats-container" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
            <div id="progress-card" class="bg-white p-4 rounded-xl border border-[#E5E7EB] shadow-xs flex flex-col justify-between">
                <div>
                    <div class="flex items-center justify-between text-[#9CA3AF] mb-1">
                        <span class="text-[10px] font-bold uppercase tracking-wider">"Completion Progress"</span>
                        {move || icon_bar_chart()}
                    </div>
                    <div class="flex items-baseline gap-2 mt-1">
                        <span class="text-2xl font-bold text-[#111827] tracking-tight">{move || format!("{}%", percent.get())}</span>
                        <span class="text-[11px] text-[#6B7280]">{move || format!("of {} tasks", total.get())}</span>
                    </div>
                </div>
                <div class="mt-3">
                    <div class="w-full bg-[#E5E7EB] rounded-full h-1.5 overflow-hidden">
                        <div class="bg-[#2563EB] h-1.5 rounded-full transition-all duration-500 ease-out"
                             style=move || format!("width: {}%", percent.get()) />
                    </div>
                </div>
            </div>

            <div id="summary-card" class="bg-white p-4 rounded-xl border border-[#E5E7EB] shadow-xs flex flex-col justify-between">
                <div>
                    <div class="flex items-center justify-between text-[#9CA3AF] mb-1">
                        <span class="text-[10px] font-bold uppercase tracking-wider">"Sprint Capacity"</span>
                        {move || icon_check_circle()}
                    </div>
                    <div class="grid grid-cols-2 gap-2 mt-1.5">
                        <div>
                            <span class="text-xl font-bold text-[#111827] block">{move || active.get()}</span>
                            <span class="text-[9px] text-[#6B7280] font-bold uppercase tracking-wider flex items-center gap-1">
                                <span class="w-1.5 h-1.5 rounded-full bg-[#2563EB] inline-block" />
                                " Active"
                            </span>
                        </div>
                        <div>
                            <span class="text-xl font-bold text-[#111827] block">{move || completed.get()}</span>
                            <span class="text-[9px] text-[#6B7280] font-bold uppercase tracking-wider flex items-center gap-1">
                                <span class="w-1.5 h-1.5 rounded-full bg-[#10B981] inline-block" />
                                " Done"
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            <div id="priorities-card" class="bg-white p-4 rounded-xl border border-[#E5E7EB] shadow-xs flex flex-col justify-between">
                <div>
                    <div class="flex items-center justify-between text-[#9CA3AF] mb-1">
                        <span class="text-[10px] font-bold uppercase tracking-wider">"High Risk"</span>
                        {move || icon_alert_triangle()}
                    </div>
                    <div class="flex items-baseline gap-2 mt-1">
                        <span class="text-2xl font-bold text-[#111827] tracking-tight">{move || high_priority.get()}</span>
                        <span class="text-[10px] text-[#EF4444] font-bold uppercase tracking-wider">"Critical"</span>
                    </div>
                </div>
                <div class="flex items-center gap-3 mt-2.5 text-[10px] text-[#6B7280] font-medium">
                    <span class="flex items-center gap-1">
                        <span class="w-1.5 h-1.5 rounded-full bg-[#F59E0B] inline-block" />
                        {move || format!(" {} Medium", medium_priority.get())}
                    </span>
                    <span class="flex items-center gap-1">
                        <span class="w-1.5 h-1.5 rounded-full bg-[#3B82F6] inline-block" />
                        {move || format!(" {} Low", low_priority.get())}
                    </span>
                </div>
            </div>

            <div id="due-card" class="bg-white p-4 rounded-xl border border-[#E5E7EB] shadow-xs flex flex-col justify-between">
                <div>
                    <div class="flex items-center justify-between text-[#9CA3AF] mb-1">
                        <span class="text-[10px] font-bold uppercase tracking-wider">"Timeline Alerts"</span>
                        {move || icon_clock()}
                    </div>
                    <div class="flex items-baseline gap-2 mt-1">
                        <span class={move || if upcoming_overdue.get() > 0 { "text-2xl font-bold tracking-tight text-[#EF4444]" } else { "text-2xl font-bold tracking-tight text-[#111827]" }}>
                            {move || upcoming_overdue.get()}
                        </span>
                        <span class="text-[11px] text-[#6B7280]">"overdue tasks"</span>
                    </div>
                </div>
                <div class="mt-2.5 flex items-center gap-1 text-[10px] text-[#9CA3AF] font-medium">
                    {move || icon_calendar_11()}
                    <span>"Real-time milestone tracker"</span>
                </div>
            </div>
        </div>
    }
}
