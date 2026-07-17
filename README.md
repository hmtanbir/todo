# Todo - Full-Stack Rust Todo App

A full-stack Rust todo application built with **Leptos** (frontend) and **Axum** (backend), faithfully ported from the React/Express original.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | Leptos 0.7 (CSR via WASM) |
| Backend | Axum 0.8 |
| Styling | Tailwind CSS (CDN) |
| Persistence | SQLite database (`data/todos.db`) |
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

### Local Development

```bash
# From the todo-app-leptos directory
chmod +x start.sh
./start.sh
```

Or run manually in two terminals:

**Terminal 1 - Backend:**
```bash
cargo run --bin server
# Server runs on http://localhost:3001
```

**Terminal 2 - Frontend:**
```bash
cd frontend
trunk serve
# Frontend runs on http://localhost:8080 (proxies /api to http://localhost:3001)
```

Then open http://localhost:8080 in your browser.

### Production Deployment (Docker)

The application has been dockerized using a multi-stage Docker build that runs on a single container exposing the application on port `3001`. The Axum backend natively serves the compiled Leptos static frontend.

**1. Run with Docker Compose (Recommended):**
```bash
docker compose up -d --build
```
This builds the application image and starts it in the background. The app is accessible at **`http://localhost:3001`**.

**2. Persisted Data:**
The SQLite database file (`todos.db`) is persistently stored in a named Docker volume `todo-data`.

**3. Stop the services:**
```bash
docker compose down
```

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

## Features

- Full CRUD operations with optimistic updates
- Real-time polling (5s interval)
- Search, filter by priority/category, sort
- Custom category creation
- Overdue task detection
- Sync status indicator
- Responsive layout with sidebar
- Statistics dashboard
- Productivity progress bar
