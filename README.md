 # Taskr – CLI Task Manager (Rust + MariaDB)

Taskr is a command-line task management application built in Rust with a MariaDB backend. It allows users to create and manage tasks directly from the terminal, demonstrating practical database design, SQL operations, and a CLI-based workflow.

---

## 📌 Features

- Add tasks via command line
- View a list of all tasks
- Persistent storage using MariaDB
- Simple and fast CLI interface
- Demonstrates SQL queries and database interaction using Rust

---

## ⚙️ Setup Instructions

### 1. Install Dependencies

Make sure you have:

- Rust (via `rustup`)
- MariaDB installed and running

Check Rust:
```bash
rustc --version
cargo --version
```

---

### 2. Clone the Repository

```bash
git clone <https://github.com/brisbie/taskr>
cd taskr
```

---

### 3. Set Up the Database

Log into MariaDB:

```bash
sudo mysql
```

Create a database:

```sql
CREATE DATABASE taskr_db;
```

Create a user and grant permissions:

```sql
CREATE USER 'myuser'@'localhost' IDENTIFIED BY 'newpassword123';
GRANT ALL PRIVILEGES ON taskr_db.* TO 'myuser'@'localhost';
FLUSH PRIVILEGES;
```

---

### 4. Configure Environment Variables

Create a `.env` file in the project root:

```env
DATABASE_URL=mysql://myuser:newpassword123@localhost/taskr_db
```

---

### 5. Run the Project

To run the application:

```bash
cargo run
```

Or, if installed globally:

```bash
cargo install --path .
taskr --help
```

---

## 💻 Usage

### Add a Task

```bash
cargo run -- add-task "Project"
```

### List Tasks

```bash
cargo run -- list-tasks
```

---

## 📊 Example Output

```
Taskr CLI starting...
Task added: Project 
```

```
[1] Project (priority: 1, status: pending)
```

---

## 🧱 Project Structure

```
taskr/
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── db.rs
│   ├── models/
│   ├── queries/
│   ├── services/
│   └── utils/
├── migrations/
├── docs/
├── Cargo.toml
└── .env
```

---

## 🧠 Technologies Used

- Rust
- MariaDB
- SQL (via sqlx)
- Clap
- Tokio

---
