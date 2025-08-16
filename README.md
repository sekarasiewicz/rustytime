# Product Requirements Document — Rust Time Tracker

## 1. Overview

A command-line or minimal-GUI Rust application to track time spent on tasks within projects.
Each project contains multiple tasks, and each play/pause action creates a time entry.
Entries are grouped by date for reporting.

## 2. Objectives
	•	Enable developers/freelancers to quickly track working time on various projects and tasks.
	•	Provide simple start/stop controls without unnecessary overhead.
	•	Keep persistent, structured records for later review.

## 3. Core Features

3.1 Project Management
	•	Create a new project (name, optional description).
	•	List all projects.
	•	Edit project name/description.
	•	Archive/delete project.

3.2 Task Management
	•	Create a task under a specific project (name, optional description).
	•	List tasks per project.
	•	Edit task name/description.
	•	Archive/delete tasks.

3.3 Time Tracking
	•	Start (Play): Start tracking time for a task.
	•	Pause (Stop): Stop tracking time for the task.
	•	Each start/stop generates a time entry with:
	•	Task ID
	•	Start timestamp
	•	End timestamp
	•	Duration (calculated)
	•	Only one task can be active at a time (starting a new one pauses the current).

3.4 Data Grouping
	•	Group time entries by date (based on start time).
	•	Provide daily summaries per project and per task.

3.5 Reporting
	•	View total time spent per project over a given date range.
	•	View total time spent per task over a given date range.
	•	Export to CSV/JSON.


## 4. Non-Functional Requirements
	•	Language: Rust
	•	Persistence: SQLite (via sqlx or rusqlite)
	•	CLI Framework: clap or structopt
	•	Time Handling: chrono
	•	OS Compatibility: macOS, Linux, Windows
	•	Performance: Should handle thousands of time entries with minimal delay.

## 5. Example CLI Commands

# Projects
timey project add "Website Redesign" --desc "Client X redesign"
timey project list
timey project edit 1 --name "Web Redesign"

# Tasks
timey task add 1 "UI Implementation"
timey task list 1

# Tracking
timey start 3       # Start task ID 3
timey stop          # Stop active task

# Reports
timey report daily
timey report project 1 --from 2025-08-01 --to 2025-08-10


## 6. Data Model (Example)

Projects Table

id	name	description	archived	created_at


Tasks Table
| id | project_id | name         | description | archived | created_at |

Time Entries Table
| id | task_id | start_time         | end_time           | duration_seconds | date        |

## 7. Future Enhancements
	•	Tags for tasks and projects.
	•	Multiple active timers (optional mode).
	•	Sync with cloud storage (Supabase, Google Drive).
	•	Simple TUI (terminal UI) with ratatui crate.
