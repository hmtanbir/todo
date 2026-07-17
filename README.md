# LeptosTask - Full-Stack Rust Todo App

A full-stack Rust todo application built with **Leptos** (frontend) and **Axum** (backend), faithfully ported from the React/Express original.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | Leptos 0.7 (CSR via WASM) |
| Backend | Axum 0.8 |
| Styling | Tailwind CSS (CDN) |
| Persistence | JSON file (`data/todos.json`) |
| Build | Trunk (frontend), Cargo (backend) |

## Prerequisites

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

## Quick Start

```bash
# From the todo-app-leptos directory
chmod +x start.sh
./start.sh
```

Or run manually in two terminals:

**Terminal 1 - Backend:**
```bash
cargo run --bin server
# Server runs on http://localhost:3000
```

**Terminal 2 - Frontend:**
```bash
cd frontend
trunk serve
# Frontend runs on http://localhost:8080
```

Then open http://localhost:8080 in your browser.

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/todos` | Fetch all todos |
| POST | `/api/todos` | Create new todo |
| PUT | `/api/todos/:id` | Update a todo |
| DELETE | `/api/todos/:id` | Delete a todo |
| POST | `/api/todos/clear-completed` | Clear completed todos |
| POST | `/api/todos/reorder` | Reorder todos |

## Project Structure

```
todo-app-leptos/
├── frontend/              # Leptos WASM frontend
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html
│   ├── style/main.css
│   └── src/
│       ├── main.rs        # Entry point
│       ├── app.rs         # Root App component
│       ├── types.rs       # Data models
│       └── components/
│           ├── mod.rs
│           ├── icons.rs           # SVG icon components
│           ├── add_todo_form.rs   # Add todo form
│           ├── task_filter.rs     # Search/filter/sort
│           ├── task_stats.rs      # Statistics cards
│           └── todo_item.rs       # Individual todo item
├── server/                # Axum API server
│   ├── Cargo.toml
│   └── src/main.rs
├── data/                  # Runtime data (auto-created)
├── Cargo.toml             # Workspace root
├── start.sh               # Convenience launcher
└── README.md
```

## Features (matching original React app)

- Full CRUD operations with optimistic updates
- Real-time polling (5s interval)
- Search, filter by priority/category, sort
- Custom category creation
- Overdue task detection
- Sync status indicator
- Responsive layout with sidebar
- Statistics dashboard
- Productivity progress bar
