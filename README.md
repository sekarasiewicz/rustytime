# Rustytime ⏱️

A fast, simple command-line time tracker built in Rust. Track time spent on projects and tasks with easy start/stop controls and comprehensive reporting.

## ✨ Features

- **Project Management**: Create, list, edit, archive, and delete projects
- **Task Management**: Organize tasks within projects with full CRUD operations
- **Time Tracking**: Simple start/stop timer with automatic time entry generation
- **Reporting**: Daily, project, and task-based time reports with date filtering
- **Data Export**: Export your time data to JSON or CSV formats
- **SQLite Storage**: Fast, reliable local database with automatic migrations
- **Cross-Platform**: Works on macOS, Linux, and Windows

## 🚀 Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/rustytime.git
cd rustytime

# Build and install
make install
# or
cargo install --path .
```

### Using Make (Recommended)
This project includes a comprehensive Makefile for easy development and deployment:

```bash
make help          # Show all available commands
make setup         # Initial development setup
make install       # Install the binary
make dev           # Run development checks
make build-release # Build optimized release binary
```

## 📖 Usage

### Project Management
```bash
# Create a new project
rustytime project add "Website Redesign" --desc "Client website redesign project"

# List all projects
rustytime project list

# Edit a project
rustytime project edit PROJECT_ID --name "New Name" --desc "Updated description"

# Archive a project
rustytime project archive PROJECT_ID

# Delete a project
rustytime project delete PROJECT_ID
```

### Task Management
```bash
# Add a task to a project
rustytime task add PROJECT_ID "UI Implementation" --desc "Implement the new UI design"

# List tasks for a project
rustytime task list PROJECT_ID

# Edit a task
rustytime task edit TASK_ID --name "Updated Task Name"

# Archive/delete tasks
rustytime task archive TASK_ID
rustytime task delete TASK_ID
```

### Time Tracking
```bash
# Start tracking time for a task
rustytime start TASK_ID

# Stop the current timer
rustytime stop
```

### Reporting
```bash
# Daily report (today by default)
rustytime report daily

# Daily report for specific date
rustytime report daily --date 2024-01-15

# Project time report
rustytime report project PROJECT_ID --from 2024-01-01 --to 2024-01-31

# Task time report
rustytime report task TASK_ID --from 2024-01-01 --to 2024-01-31
```

### Data Export
```bash
# Export to JSON
rustytime export json --out data.json --from 2024-01-01 --to 2024-12-31

# Export to CSV (coming soon)
rustytime export csv --out data.csv
```

## 🗃️ Database

Rustytime uses SQLite for local storage. The database is automatically created and migrated on first run. By default, the database file is stored at `~/.local/share/rustytime/rustytime.db` (on macOS: `~/Library/Application Support/rustytime/rustytime.db`).

### Custom Database Location

There are multiple ways to specify a custom database location, in order of priority:

1. **Environment Variable** (Recommended for deployment):
```bash
export RUSTYTIME_DATABASE_URL="sqlite:///path/to/custom/rustytime.db"
rustytime project list
```

2. **Command Line Option** (For one-off usage):
```bash
rustytime --db /path/to/custom.db project list
```

3. **Build-Time Configuration** (For packaged distributions):
```bash
# Set at build time to embed the path in the binary
RUSTYTIME_BUILD_DB_PATH="/opt/rustytime/data/rustytime.db" cargo build --release
```

The environment variable `RUSTYTIME_DATABASE_URL` is specific to Rustytime and won't conflict with other applications that might use the generic `DATABASE_URL`.

### Database Schema

#### Projects
- `id` (UUID) - Primary key
- `name` (TEXT) - Project name
- `description` (TEXT, optional) - Project description
- `archived` (BOOLEAN) - Archive status
- `created_at` (TIMESTAMP) - Creation timestamp

#### Tasks
- `id` (UUID) - Primary key
- `project_id` (UUID) - Foreign key to projects
- `name` (TEXT) - Task name
- `description` (TEXT, optional) - Task description
- `archived` (BOOLEAN) - Archive status
- `created_at` (TIMESTAMP) - Creation timestamp

#### Time Entries
- `id` (UUID) - Primary key
- `task_id` (UUID) - Foreign key to tasks
- `start_time` (TIMESTAMP) - When tracking started
- `end_time` (TIMESTAMP, optional) - When tracking stopped
- `duration_seconds` (INTEGER) - Calculated duration
- `date` (DATE) - Date of the entry (for grouping)

## 🛠️ Development

### Prerequisites
- Rust 1.70+ with 2024 edition support
- SQLite 3.x

### Building
```bash
# Development build
make build

# Release build
make build-release

# Run tests
make test

# Format code
make fmt

# Run clippy
make clippy

# Run all checks
make dev
```

### Development Tools
```bash
# Watch for changes and run tests
make watch

# Watch and run the application
make watch-run

# Generate documentation
make docs

# Security audit
make audit
```

## 🔧 Dependencies

- **clap** - Command-line argument parsing
- **sqlx** - Async SQL toolkit with compile-time checked queries
- **tokio** - Async runtime
- **time** & **time-tz** - Date and time handling with timezone support
- **uuid** - UUID generation
- **serde** & **serde_json** - Serialization for data export
- **anyhow** - Error handling

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 🚧 Roadmap

- [ ] CSV export functionality
- [ ] Tags for projects and tasks
- [ ] Multiple active timers support
- [ ] Terminal UI (TUI) interface
- [ ] Cloud synchronization
- [ ] Time tracking analytics and insights
- [ ] Integration with popular project management tools

## 💡 Tips

- Use `rustytime --help` to see all available commands
- The timer automatically stops when starting a new task
- Date formats should be in YYYY-MM-DD format
- UUIDs are used for all entity IDs for better uniqueness
- Use `make example-usage` to see more usage examples
