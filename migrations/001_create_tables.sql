-- 001_create_tables.sql

-- 1. Projects Table
CREATE TABLE projects (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    status VARCHAR(20) DEFAULT 'Active'
) ENGINE=InnoDB;

-- 2. Tasks Table
CREATE TABLE tasks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    project_id INT,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    priority INT NOT NULL DEFAULT 3,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    due_date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_task_project FOREIGN KEY (project_id) 
        REFERENCES projects(id) ON DELETE SET NULL
) ENGINE=InnoDB;

-- 3. Subtasks Table
CREATE TABLE subtasks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    task_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    CONSTRAINT fk_subtask_task FOREIGN KEY (task_id) 
        REFERENCES tasks(id) ON DELETE CASCADE
) ENGINE=InnoDB;

-- 4. Comments (Unified Notes) Table
CREATE TABLE comments (
    id INT AUTO_INCREMENT PRIMARY KEY,
    task_id INT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_comment_task FOREIGN KEY (task_id) 
        REFERENCES tasks(id) ON DELETE CASCADE
) ENGINE=InnoDB;

-- 5. Tags Table
CREATE TABLE tags (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
) ENGINE=InnoDB;

-- 6. Task-Tags Junction Table (Many-to-Many)
CREATE TABLE task_tags (
    task_id INT NOT NULL,
    tag_id INT NOT NULL,
    PRIMARY KEY (task_id, tag_id),
    CONSTRAINT fk_tt_task FOREIGN KEY (task_id) 
        REFERENCES tasks(id) ON DELETE CASCADE,
    CONSTRAINT fk_tt_tag FOREIGN KEY (tag_id) 
        REFERENCES tags(id) ON DELETE CASCADE
) ENGINE=InnoDB;

-- 7. Task Logs (Activity Tracker)
CREATE TABLE task_logs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    task_id INT NOT NULL,
    message VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_log_task FOREIGN KEY (task_id) 
        REFERENCES tasks(id) ON DELETE CASCADE
) ENGINE=InnoDB;

-- 8. Final Project View
CREATE OR REPLACE VIEW task_summary_view AS
SELECT 
    t.id AS task_id,
    t.title,
    t.status AS task_status,
    p.name AS project_name,
    (SELECT COUNT(*) FROM subtasks WHERE task_id = t.id) AS subtask_count,
    (SELECT content FROM comments WHERE task_id = t.id ORDER BY created_at DESC LIMIT 1) AS latest_note
FROM tasks t
LEFT JOIN projects p ON t.project_id = p.id;
