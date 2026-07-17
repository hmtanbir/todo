use crate::components::add_todo_form::AddTodoForm;
use crate::components::icons::*;
use crate::components::task_filter::TaskFilter;
use crate::components::task_stats::TaskStats;
use crate::components::todo_item::TodoItem;
use crate::types::*;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

async fn fetch_todos_from_server() -> Result<Vec<Todo>, String> {
    let resp = gloo_net::http::Request::get("/api/todos")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if resp.ok() {
        resp.json::<Vec<Todo>>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

async fn create_todo_on_server(data: &NewTodoData) -> Result<Todo, String> {
    let body = serde_json::json!({
        "title": data.title,
        "description": data.description,
        "priority": data.priority.as_str(),
        "category": data.category,
        "dueDate": data.due_date,
    });
    let resp = gloo_net::http::Request::post("/api/todos")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .unwrap()
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if resp.ok() {
        resp.json::<Todo>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

async fn update_todo_on_server(id: &str, updates: &TodoUpdate) -> Result<Todo, String> {
    let mut body = serde_json::Map::new();
    if let Some(ref t) = updates.title {
        body.insert("title".into(), serde_json::Value::String(t.clone()));
    }
    if let Some(ref d) = updates.description {
        match d {
            Some(val) => {
                body.insert("description".into(), serde_json::Value::String(val.clone()));
            }
            None => {
                body.insert("description".into(), serde_json::Value::Null);
            }
        };
    }
    if let Some(c) = updates.completed {
        body.insert("completed".into(), serde_json::Value::Bool(c));
    }
    if let Some(ref p) = updates.priority {
        body.insert(
            "priority".into(),
            serde_json::Value::String(p.as_str().to_string()),
        );
    }
    if let Some(ref c) = updates.category {
        body.insert("category".into(), serde_json::Value::String(c.clone()));
    }
    if let Some(ref d) = updates.due_date {
        match d {
            Some(val) => {
                body.insert("dueDate".into(), serde_json::Value::String(val.clone()));
            }
            None => {
                body.insert("dueDate".into(), serde_json::Value::Null);
            }
        };
    }
    let resp = gloo_net::http::Request::put(&format!("/api/todos/{}", id))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&serde_json::Value::Object(body)).unwrap())
        .unwrap()
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if resp.ok() {
        resp.json::<Todo>()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

async fn delete_todo_on_server(id: &str) -> Result<(), String> {
    let resp = gloo_net::http::Request::delete(&format!("/api/todos/{}", id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

async fn clear_completed_on_server() -> Result<(), String> {
    let resp = gloo_net::http::Request::post("/api/todos/clear-completed")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (todos, set_todos) = signal(Vec::<Todo>::new());
    let (sync_status, set_sync_status) = signal(SyncStatus::default());
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal("all".to_string());
    let (selected_priority, set_selected_priority) = signal("all".to_string());
    let (sort_by, set_sort_by) = signal("newest".to_string());
    let (error_toast, set_error_toast) = signal(String::new());

    let categories = Memo::new(move |_| {
        let mut cats: Vec<String> = DEFAULT_CATEGORIES.iter().map(|s| s.to_string()).collect();
        for todo in todos.get() {
            if !cats.contains(&todo.category) {
                cats.push(todo.category.clone());
            }
        }
        cats
    });

    let filtered_todos = Memo::new(move |_| {
        let all = todos.get();
        let sq = search_query.get().to_lowercase();
        let sc = selected_category.get();
        let sp = selected_priority.get();
        let sb = sort_by.get();

        let mut result: Vec<Todo> = all
            .into_iter()
            .filter(|todo| {
                let matches_search = sq.is_empty()
                    || todo.title.to_lowercase().contains(&sq)
                    || todo
                        .description
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&sq);
                let matches_category = sc == "all" || todo.category == sc;
                let matches_priority = sp == "all" || todo.priority.as_str() == sp;
                matches_search && matches_category && matches_priority
            })
            .collect();

        result.sort_by(|a, b| match sb.as_str() {
            "oldest" => a.created_at.cmp(&b.created_at),
            "dueDate" => match (&a.due_date, &b.due_date) {
                (None, _) => std::cmp::Ordering::Greater,
                (_, None) => std::cmp::Ordering::Less,
                (Some(da), Some(db)) => da.cmp(db),
            },
            "priority" => {
                let weight = |p: &TodoPriority| match p {
                    TodoPriority::High => 3,
                    TodoPriority::Medium => 2,
                    TodoPriority::Low => 1,
                };
                weight(&b.priority).cmp(&weight(&a.priority))
            }
            "alphabetical" => a.title.cmp(&b.title),
            _ => b.created_at.cmp(&a.created_at),
        });
        result
    });

    let completed_count = Memo::new(move |_| todos.get().iter().filter(|t| t.completed).count());

    let trigger_error = {
        let set_error_toast = set_error_toast.clone();
        move |msg: String| {
            set_error_toast.set(msg);
            let set_error_toast = set_error_toast.clone();
            spawn_local(async move {
                gloo_timers::future::sleep(std::time::Duration::from_millis(4000)).await;
                set_error_toast.set(String::new());
            });
        }
    };

    // Initial fetch + polling
    {
        let set_todos = set_todos.clone();
        let set_sync_status = set_sync_status.clone();
        let trigger_error = trigger_error.clone();
        spawn_local(async move {
            set_sync_status.set(SyncStatus {
                status: SyncStatusState::Syncing,
                last_synced_at: None,
            });
            match fetch_todos_from_server().await {
                Ok(data) => {
                    set_todos.set(data);
                    set_sync_status.set(SyncStatus {
                        status: SyncStatusState::Success,
                        last_synced_at: Some(current_time_string()),
                    });
                }
                Err(_) => {
                    set_sync_status.set(SyncStatus {
                        status: SyncStatusState::Error,
                        last_synced_at: None,
                    });
                    trigger_error("Could not sync with cloud server. Retrying...".to_string());
                }
            }
            loop {
                gloo_timers::future::sleep(std::time::Duration::from_millis(5000)).await;
                match fetch_todos_from_server().await {
                    Ok(data) => {
                        set_todos.set(data);
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Success,
                            last_synced_at: Some(current_time_string()),
                        });
                    }
                    Err(_) => {
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Error,
                            last_synced_at: None,
                        });
                    }
                }
            }
        });
    }

    let force_refresh = {
        let set_todos = set_todos.clone();
        let set_sync_status = set_sync_status.clone();
        let trigger_error = trigger_error.clone();
        Callback::new(move |()| {
            let set_todos = set_todos.clone();
            let set_sync_status = set_sync_status.clone();
            let trigger_error = trigger_error.clone();
            spawn_local(async move {
                set_sync_status.set(SyncStatus {
                    status: SyncStatusState::Syncing,
                    last_synced_at: None,
                });
                match fetch_todos_from_server().await {
                    Ok(data) => {
                        set_todos.set(data);
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Success,
                            last_synced_at: Some(current_time_string()),
                        });
                    }
                    Err(_) => {
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Error,
                            last_synced_at: None,
                        });
                        trigger_error("Could not sync with cloud server. Retrying...".to_string());
                    }
                }
            });
        })
    };

    let handle_add_todo = {
        let set_todos = set_todos.clone();
        let set_sync_status = set_sync_status.clone();
        let trigger_error = trigger_error.clone();
        Callback::new(move |data: NewTodoData| {
            let set_todos = set_todos.clone();
            let set_sync_status = set_sync_status.clone();
            let trigger_error = trigger_error.clone();
            let temp_id = format!("opt-{}", &js_sys::Math::random().to_string()[2..9]);
            let now = js_sys::Date::new_0()
                .to_iso_string()
                .as_string()
                .unwrap_or_default();
            let optimistic = Todo {
                id: temp_id.clone(),
                title: data.title.clone(),
                description: data.description.clone(),
                completed: false,
                priority: data.priority,
                category: data.category.clone(),
                due_date: data.due_date.clone(),
                created_at: now.clone(),
                updated_at: now,
            };
            set_todos.update(|t| t.insert(0, optimistic));
            set_sync_status.set(SyncStatus {
                status: SyncStatusState::Syncing,
                last_synced_at: None,
            });
            spawn_local(async move {
                match create_todo_on_server(&data).await {
                    Ok(saved) => {
                        set_todos.update(|t| {
                            if let Some(pos) = t.iter().position(|x| x.id == temp_id) {
                                t[pos] = saved;
                            }
                        });
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Success,
                            last_synced_at: Some(current_time_string()),
                        });
                    }
                    Err(_) => {
                        set_todos.update(|t| t.retain(|x| x.id != temp_id));
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Error,
                            last_synced_at: None,
                        });
                        trigger_error("Add task failed. Reverted local change.".to_string());
                    }
                }
            });
        })
    };

    let handle_toggle = {
        let set_todos = set_todos.clone();
        let set_sync_status = set_sync_status.clone();
        let trigger_error = trigger_error.clone();
        Callback::new(move |(id, completed): (String, bool)| {
            let set_todos = set_todos.clone();
            let set_sync_status = set_sync_status.clone();
            let trigger_error = trigger_error.clone();
            let id_clone = id.clone();
            let now = js_sys::Date::new_0()
                .to_iso_string()
                .as_string()
                .unwrap_or_default();
            set_todos.update(|t| {
                if let Some(todo) = t.iter_mut().find(|x| x.id == id) {
                    todo.completed = completed;
                    todo.updated_at = now;
                }
            });
            set_sync_status.set(SyncStatus {
                status: SyncStatusState::Syncing,
                last_synced_at: None,
            });
            let updates = TodoUpdate {
                title: None,
                description: None,
                completed: Some(completed),
                priority: None,
                category: None,
                due_date: None,
            };
            spawn_local(async move {
                match update_todo_on_server(&id_clone, &updates).await {
                    Ok(_) => {
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Success,
                            last_synced_at: Some(current_time_string()),
                        });
                    }
                    Err(_) => {
                        set_todos.update(|t| {
                            if let Some(todo) = t.iter_mut().find(|x| x.id == id_clone) {
                                todo.completed = !completed;
                            }
                        });
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Error,
                            last_synced_at: None,
                        });
                        trigger_error("Action failed. Reverted state.".to_string());
                    }
                }
            });
        })
    };

    let handle_update = {
        let set_todos = set_todos.clone();
        let set_sync_status = set_sync_status.clone();
        let trigger_error = trigger_error.clone();
        Callback::new(move |(id, updates): (String, TodoUpdate)| {
            let set_todos = set_todos.clone();
            let set_sync_status = set_sync_status.clone();
            let trigger_error = trigger_error.clone();
            let id_clone = id.clone();
            let now = js_sys::Date::new_0()
                .to_iso_string()
                .as_string()
                .unwrap_or_default();
            set_todos.update(|t| {
                if let Some(todo) = t.iter_mut().find(|x| x.id == id) {
                    if let Some(ref title) = updates.title {
                        todo.title = title.clone();
                    }
                    if let Some(ref desc) = updates.description {
                        todo.description = desc.clone();
                    }
                    if let Some(p) = &updates.priority {
                        todo.priority = *p;
                    }
                    if let Some(ref cat) = updates.category {
                        todo.category = cat.clone();
                    }
                    if let Some(ref dd) = updates.due_date {
                        todo.due_date = dd.clone();
                    }
                    todo.updated_at = now;
                }
            });
            set_sync_status.set(SyncStatus {
                status: SyncStatusState::Syncing,
                last_synced_at: None,
            });
            spawn_local(async move {
                match update_todo_on_server(&id_clone, &updates).await {
                    Ok(saved) => {
                        set_todos.update(|t| {
                            if let Some(todo) = t.iter_mut().find(|x| x.id == id_clone) {
                                *todo = saved;
                            }
                        });
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Success,
                            last_synced_at: Some(current_time_string()),
                        });
                    }
                    Err(_) => {
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Error,
                            last_synced_at: None,
                        });
                        trigger_error("Task edit failed. Changes reverted.".to_string());
                    }
                }
            });
        })
    };

    let handle_delete = {
        let set_todos = set_todos.clone();
        let set_sync_status = set_sync_status.clone();
        let trigger_error = trigger_error.clone();
        Callback::new(move |id: String| {
            let set_todos = set_todos.clone();
            let set_sync_status = set_sync_status.clone();
            let trigger_error = trigger_error.clone();
            let id_clone = id.clone();
            let mut removed = None;
            set_todos.update(|t| {
                if let Some(pos) = t.iter().position(|x| x.id == id) {
                    removed = Some(t.remove(pos));
                }
            });
            set_sync_status.set(SyncStatus {
                status: SyncStatusState::Syncing,
                last_synced_at: None,
            });
            spawn_local(async move {
                match delete_todo_on_server(&id_clone).await {
                    Ok(_) => {
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Success,
                            last_synced_at: Some(current_time_string()),
                        });
                    }
                    Err(_) => {
                        if let Some(todo) = removed {
                            set_todos.update(|t| t.insert(0, todo));
                        }
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Error,
                            last_synced_at: None,
                        });
                        trigger_error("Delete failed. Restored task.".to_string());
                    }
                }
            });
        })
    };

    let handle_clear_completed = {
        let set_todos = set_todos.clone();
        let set_sync_status = set_sync_status.clone();
        let trigger_error = trigger_error.clone();
        let force_refresh = force_refresh.clone();
        Callback::new(move |()| {
            let set_todos = set_todos.clone();
            let set_sync_status = set_sync_status.clone();
            let trigger_error = trigger_error.clone();
            let force_refresh = force_refresh.clone();
            set_todos.update(|t| t.retain(|x| !x.completed));
            set_sync_status.set(SyncStatus {
                status: SyncStatusState::Syncing,
                last_synced_at: None,
            });
            spawn_local(async move {
                match clear_completed_on_server().await {
                    Ok(_) => {
                        set_sync_status.set(SyncStatus {
                            status: SyncStatusState::Success,
                            last_synced_at: Some(current_time_string()),
                        });
                    }
                    Err(_) => {
                        force_refresh.run(());
                        trigger_error("Clear completed failed.".to_string());
                    }
                }
            });
        })
    };

    let today_display = current_date_display();
    let todos_memo = Memo::new(move |_| todos.get());

    view! {
        <div id="todo-app-root" class="min-h-screen bg-[#F9FAFB] flex flex-col font-sans antialiased text-[#111827]">
            <header id="app-header" class="h-14 border-b border-[#E5E7EB] bg-white px-6 flex items-center justify-between z-10">
                <div class="flex items-center gap-2.5">
                    <div class="bg-[#2563EB] text-white p-1.5 rounded-lg">{move || icon_check_square()}</div>
                    <div>
                        <h1 class="text-sm font-bold tracking-tight text-[#111827] flex items-center gap-1.5 font-display">
                            "Todo " <span class="font-mono text-[10px] text-[#9CA3AF] font-medium">"v0.7.0"</span>
                        </h1>
                    </div>
                </div>
                <div class="flex items-center gap-4">
                    <div id="sync-status" class="flex items-center gap-2 px-2.5 py-1 bg-[#F9FAFB] border border-[#E5E7EB] rounded-lg">
                        <span class="relative flex h-1.5 w-1.5">
                            {move || match sync_status.get().status {
                                SyncStatusState::Syncing => view! {
                                    <>
                                        <span class="animate-ping-slow absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75" />
                                        <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-amber-500" />
                                    </>
                                }.into_any(),
                                SyncStatusState::Success => view! {
                                    <>
                                        <span class="animate-ping-slow absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-30" />
                                        <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-[#10B981]" />
                                    </>
                                }.into_any(),
                                SyncStatusState::Error => view! {
                                    <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-[#EF4444]" />
                                }.into_any(),
                                SyncStatusState::Idle => view! {
                                    <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-[#9CA3AF]" />
                                }.into_any(),
                            }}
                        </span>
                        <span class="text-[10px] font-bold text-[#4B5563] uppercase tracking-wider flex items-center gap-1">
                            {move || match sync_status.get().status {
                                SyncStatusState::Syncing => "Syncing".to_string(),
                                SyncStatusState::Success => "API Connected".to_string(),
                                SyncStatusState::Error => "Offline / Retry".to_string(),
                                SyncStatusState::Idle => "Idle".to_string(),
                            }}
                            {move || sync_status.get().last_synced_at.map(|t| format!("({})", t)).unwrap_or_default()}
                        </span>
                        <button id="force-sync-btn" on:click=move |_| force_refresh.run(())
                            class={move || if sync_status.get().status == SyncStatusState::Syncing {
                                "p-0.5 hover:bg-[#E5E7EB]/55 text-[#9CA3AF] hover:text-[#4B5563] rounded-md transition-all cursor-pointer ml-0.5 text-[#2563EB]"
                            } else {
                                "p-0.5 hover:bg-[#E5E7EB]/55 text-[#9CA3AF] hover:text-[#4B5563] rounded-md transition-all cursor-pointer ml-0.5"
                            }}
                            title="Force refresh tasks"
                        >
                            {move || icon_refresh_cw()}
                        </button>
                    </div>
                    <div class="h-7 w-px bg-[#E5E7EB]" />
                    <div class="flex items-center gap-2">
                        <div class="h-7 w-7 rounded-full bg-[#E5E7EB] border border-[#D1D5DB] flex items-center justify-center text-[10px] font-bold text-[#4B5563]">"SL"</div>
                        <span class="text-[11px] font-bold text-[#4B5563] hidden sm:inline">"Sprint Lead"</span>
                    </div>
                </div>
            </header>

            <div class="flex-1 flex overflow-hidden">
                <aside class="w-64 border-r border-[#E5E7EB] bg-white p-5 flex flex-col justify-between hidden md:flex">
                    <div class="flex flex-col gap-6">
                        <div>
                            <h2 class="text-[10px] font-bold text-[#9CA3AF] uppercase tracking-wider mb-2.5">"Sprint Views"</h2>
                            <div class="flex flex-col gap-1">
                                <button on:click=move |_| set_selected_category.set("all".to_string())
                                    class={move || if selected_category.get() == "all" {
                                        "flex items-center justify-between px-3 py-2 rounded-lg text-xs font-bold transition-all cursor-pointer bg-[#EFF6FF] text-[#2563EB]"
                                    } else {
                                        "flex items-center justify-between px-3 py-2 rounded-lg text-xs font-bold transition-all cursor-pointer text-[#4B5563] hover:bg-[#F9FAFB] hover:text-[#111827]"
                                    }}
                                >
                                    <span class="flex items-center gap-2">{move || icon_layout_dashboard()} " All Sprint Tasks"</span>
                                    <span class={move || if selected_category.get() == "all" {
                                        "text-[10px] font-mono px-1.5 py-0.5 rounded-md bg-[#2563EB]/10 text-[#2563EB]"
                                    } else {
                                        "text-[10px] font-mono px-1.5 py-0.5 rounded-md bg-[#F3F4F6] text-[#6B7280]"
                                    }}>
                                        {move || todos.get().len()}
                                    </span>
                                </button>
                            </div>
                        </div>

                        <div>
                            <h2 class="text-[10px] font-bold text-[#9CA3AF] uppercase tracking-wider mb-2.5">"Project Categories"</h2>
                            <div class="flex flex-col gap-1">
                                {move || {
                                    let cats = categories.get();
                                    let current = selected_category.get();
                                    cats.into_iter().enumerate().map(|(idx, cat)| {
                                        let is_active = current == cat;
                                        let colors = ["bg-[#3B82F6]", "bg-[#10B981]", "bg-[#F59E0B]", "bg-[#EC4899]", "bg-[#8B5CF6]"];
                                        let dot_color = colors[idx % colors.len()];
                                        let cat_clone = cat.clone();
                                        let cat_clone2 = cat.clone();
                                        view! {
                                            <button on:click=move |_| set_selected_category.set(cat_clone.clone())
                                                class={if is_active {
                                                    "flex items-center justify-between px-3 py-2 rounded-lg text-xs font-bold transition-all cursor-pointer bg-[#EFF6FF] text-[#2563EB]"
                                                } else {
                                                    "flex items-center justify-between px-3 py-2 rounded-lg text-xs font-bold transition-all cursor-pointer text-[#4B5563] hover:bg-[#F9FAFB] hover:text-[#111827]"
                                                }}
                                            >
                                                <span class="flex items-center gap-2">
                                                    <span class={format!("w-1.5 h-1.5 rounded-full {}", dot_color)} />
                                                    {cat.clone()}
                                                </span>
                                                <span class={if is_active {
                                                    "text-[10px] font-mono px-1.5 py-0.5 rounded-md bg-[#2563EB]/10 text-[#2563EB]"
                                                } else {
                                                    "text-[10px] font-mono px-1.5 py-0.5 rounded-md bg-[#F3F4F6] text-[#6B7280]"
                                                }}>
                                                    {move || todos.get().iter().filter(|t| t.category == cat_clone2).count()}
                                                </span>
                                            </button>
                                        }
                                    }).collect_view()
                                }}
                            </div>
                        </div>
                    </div>

                    <div class="p-3.5 bg-[#F9FAFB] border border-[#E5E7EB] rounded-xl flex flex-col gap-2">
                        <div class="flex items-center justify-between">
                            <span class="text-[10px] font-bold text-[#4B5563] uppercase tracking-wider flex items-center gap-1">
                                {move || icon_trending_up()} " Leptos Productivity"
                            </span>
                            <span class="text-[10px] font-bold text-[#2563EB]">
                                {move || { let t = todos.get(); let total = t.len(); let done = t.iter().filter(|x| x.completed).count(); if total > 0 { format!("{}%", (done as f64 / total as f64 * 100.0).round() as i32) } else { "0%".to_string() } }}
                            </span>
                        </div>
                        <div class="w-full bg-[#E5E7EB] h-1.5 rounded-full overflow-hidden">
                            <div class="bg-[#2563EB] h-full transition-all duration-300"
                                style=move || { let t = todos.get(); let total = t.len(); let done = t.iter().filter(|x| x.completed).count(); let pct = if total > 0 { done as f64 / total as f64 * 100.0 } else { 0.0 }; format!("width: {}%", pct) } />
                        </div>
                        <span class="text-[9px] font-semibold text-[#9CA3AF] uppercase tracking-wider">
                            {move || { let t = todos.get(); let done = t.iter().filter(|x| x.completed).count(); format!("{} of {} completed", done, t.len()) }}
                        </span>
                    </div>
                </aside>

                <main class="flex-1 overflow-y-auto p-6 md:p-8 flex flex-col gap-6">
                    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                        <div>
                            <h2 class="text-lg md:text-xl font-bold font-display tracking-tight text-[#111827]">"Today's Sprint Backlog"</h2>
                            <p class="text-xs text-[#6B7280] font-medium mt-0.5">
                                "Current backlog for the "
                                <span class="text-[#2563EB] font-bold">{move || if selected_category.get() == "all" { "Sprint".to_string() } else { selected_category.get() }}</span>
                                " project"
                            </p>
                        </div>
                        <div class="text-right flex sm:flex-col items-center sm:items-end justify-between sm:justify-center text-[11px] text-[#9CA3AF] font-bold uppercase tracking-wider gap-1.5">
                            <span>{today_display}</span>
                        </div>
                    </div>

                    <Show when=move || !error_toast.get().is_empty() fallback=|| ()>
                        <div id="error-banner" class="bg-red-50 border border-red-100 text-red-600 text-xs font-semibold rounded-lg p-3 flex items-center gap-2 shadow-xs">
                            {move || icon_cloud_lightning()} " " {move || error_toast.get()}
                        </div>
                    </Show>

                    <TaskStats todos=todos_memo />
                    <AddTodoForm on_add_todo=handle_add_todo categories=categories />
                    <TaskFilter
                        search_query=(search_query, set_search_query)
                        selected_priority=(selected_priority, set_selected_priority)
                        sort_by=(sort_by, set_sort_by)
                        on_clear_completed=handle_clear_completed
                        completed_count=completed_count
                    />

                    <div id="todo-items-list" class="flex flex-col gap-2.5">
                        {move || {
                            let items = filtered_todos.get();
                            if items.is_empty() {
                                view! {
                                    <div id="empty-state" class="text-center py-10 bg-white rounded-xl border border-[#E5E7EB]">
                                        <div class="mx-auto w-10 h-10 bg-[#F9FAFB] text-[#9CA3AF] rounded-lg flex items-center justify-center mb-3">
                                            {icon_layers()}
                                        </div>
                                        <p class="text-xs font-bold text-[#111827]">"No backlog items found"</p>
                                        <p class="text-[11px] text-[#6B7280] mt-0.5 font-medium">"Try tweaking your search input, categories, or filters"</p>
                                    </div>
                                }.into_any()
                            } else {
                                items.into_iter().map(|todo| {
                                    view! {
                                        <TodoItem
                                            todo=todo
                                            on_toggle=handle_toggle.clone()
                                            on_delete=handle_delete.clone()
                                            on_update=handle_update.clone()
                                            categories=categories
                                        />
                                    }
                                }).collect_view().into_any()
                            }
                        }}
                    </div>

                    <footer class="text-center mt-auto pt-8 text-[10px] text-[#9CA3AF] font-bold uppercase tracking-wider">
                        <p>"(c) 2026 Leptos DX Todo Engine. Secure file-backed JSON REST persistence."</p>
                    </footer>
                </main>
            </div>
        </div>
    }
}
