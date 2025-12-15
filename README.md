``` Under development ```
# Task Sorter

A full-stack Rust desktop application for managing and organizing tasks with an intuitive interface. Built with modern Rust technologies for performance and reliability.

## Overview

Task Sorter is a desktop task management application that helps you track todos with detailed information including due dates, importance levels, and estimated time requirements. The application features a clean interface powered by Dioxus and a robust backend using Axum and SurrealDB.

## Features

- ✅ Create, view, and manage tasks
- 📋 Track task details (name, description, due date, importance, required time)
- ✓ Mark tasks as complete/incomplete
- 💾 Persistent storage with SurrealDB (RocksDB engine)
- 🚀 Fast and responsive desktop interface
- 🦀 100% Rust implementation

## Tech Stack

### Backend
- **Axum** - Fast, ergonomic web framework
- **SurrealDB** - Modern, scalable database with RocksDB storage engine
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
- **Tower-HTTP** - CORS middleware

### Frontend
- **Dioxus 0.7.0** - Cross-platform UI framework
- **Dioxus Desktop** - Native desktop application wrapper
- **Reqwest** - HTTP client for backend communication

## Project Structure
```
task-manager/
├── backend/
│   ├── src/
│   │   ├── main.rs                 # Axum server entry point
│   │   ├── schedule_helper.rs      # Scheduling logic and algorithms
│   │   └── task_helper.rs          # Task operations and database handlers
│   ├── Cargo.lock
│   └── Cargo.toml
├── frontend/
│   ├── assets/
│   │   ├── main.css                # Main stylesheet
│   │   └── tailwind.css            # Tailwind CSS
│   ├── src/
│   │   ├── backend_helper.rs       # API client and backend communication
│   │   └── main.rs                 # Dioxus UI components
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dioxus.toml
│   ├── tailwind.css
├── .gitignore
├── LICENSE
└── README.md
```

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Dioxus CLI: `cargo install dioxus-cli`

## Installation

1. **Clone the repository**
```

git clone https://github.com/SeerBlazeJ/task-manager-axum.git
cd task-manager-axum

```

2. **Install dependencies**

Dependencies are automatically managed by Cargo when building.

## Running the Application

### Start the Backend Server

```

cd backend
cargo run

```

The server will start on `http://localhost:3000`

### Start the Frontend

In a new terminal:

```

cd frontend
dx serve --platform desktop

```

Or for development with hot reload:

```

dx serve --hot-reload

```

## API Endpoints

The backend exposes the following REST endpoints:

| Method | Endpoint                  | Description                          |
| ------ | ------------------------- | ------------------------------------ |
| GET    | `/get_tasks`              | Retrieve all tasks                   |
| GET    | `/get_task/{id}`          | Retrieve a single task by ID         |
| POST   | `/add_task`               | Create a new task                    |
| POST   | `/mark_done`              | Mark a task as completed             |
| POST   | `/mark_undone`            | Mark a task as not completed         |
| POST   | `/delete`                 | Delete a task                        |
| POST   | `/add_sched`              | Add a scheduled item                 |
| GET    | `/get_schedule/{day_str}` | Retrieve schedule for a specific day |


## Development

### Backend Development

```

cd backend
cargo watch -x run  \# Auto-reload on changes

```

### Frontend Development

```

cd frontend
dx serve --hot-reload  \# Live reload UI changes

```

### Database

The application uses SurrealDB with RocksDB storage. The database file (`TaskManagerApp`) is created automatically in the backend directory on first run.

**Database Details:**
- Namespace: `core`

## Building for Production

### Build Backend

```

cd backend
cargo build --release

```

The binary will be at `target/release/backend`

### Build Frontend Desktop App

```

cd frontend
dx build --release --platform desktop

```

Create distributable bundles:

```

dx bundle --release --platform desktop

```

Bundles will be in `target/dx/bundle/` directory:
- **Linux**: `.deb`, `.AppImage`
- **macOS**: `.app`, `.dmg`
- **Windows**: `.exe`, `.msi`

## Configuration

### Backend Port

To change the backend port, modify `main.rs`:

```

let addr = TcpListener::bind("localhost:3000")  // Change port here

```

### Frontend API URL

Update the base URL in `backend_connector.rs`:

```

const BASE_URL: \&str = "http://localhost:3000";  // Change if needed

```

## Troubleshooting

### Backend won't start
- Ensure port 3000 is available
- Check if the database directory has write permissions

### Frontend can't connect
- Verify backend is running on `localhost:3000`
- Check CORS is enabled (already configured)

### Build errors
- Run `cargo clean` in both directories
- Update Rust: `rustup update`
- Clear Dioxus cache: `dx clean`

### Goals

#### Short Term
- [ ] Integrate Tauri and its functionalities
- [ ] Enhance UI and funcitonalities for modifying static schedules
- [ ] Add suuoprt of file uploads for tasks, routines and events too ( When functionality is created )
- [ ] Add support for events
- [ ] Make databse for day schedules dynamically updatable to accomodate new entries

#### Long Term
- [ ] Task scheduling with ML-based optimization
- [X] Calendar view integration
- [x] Task priority sorting algorithms
- [ ] Export/import functionality
- [ ] AI-powered offset calculations
- [ ] Mobile app support
- [ ] Deployment ready app with all the features

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## License

[License](LICENSE) - This project is protected from trademarks or patent use under the Creative Commons Zero v1.0 Universal License.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) - Amazing Rust UI framework
- [Axum](https://github.com/tokio-rs/axum) - Ergonomic web framework
- [SurrealDB](https://surrealdb.com/) - Next-generation database

---

Built with 🦀 Rust
```
