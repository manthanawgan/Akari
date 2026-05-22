use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::Mutex,
};
use tauri::{Manager, State};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    text: String,
    done: bool,
    created_at: String,
}

struct AppState {
    storage_lock: Mutex<()>,
}

fn storage_dir() -> Result<PathBuf, String> {
    let base = dirs::data_local_dir().ok_or("Could not find local data directory")?;
    Ok(base.join("akari"))
}

fn todos_path() -> Result<PathBuf, String> {
    Ok(storage_dir()?.join("todos.json"))
}

fn last_modified_path() -> Result<PathBuf, String> {
    Ok(storage_dir()?.join("last_modified"))
}

fn ensure_storage_dir() -> Result<(), String> {
    fs::create_dir_all(storage_dir()?).map_err(|err| format!("Could not create storage directory: {err}"))
}

fn read_todos_from_disk() -> Result<Vec<Todo>, String> {
    ensure_storage_dir()?;
    let path = todos_path()?;

    if !path.exists() {
        fs::write(&path, "[]\n").map_err(|err| format!("Could not initialize todos.json: {err}"))?;
        touch_last_modified()?;
        return Ok(Vec::new());
    }

    let raw = fs::read_to_string(&path).map_err(|err| format!("Could not read todos.json: {err}"))?;
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&raw).map_err(|err| format!("Could not parse todos.json: {err}"))
}

fn write_todos_to_disk(todos: &[Todo]) -> Result<(), String> {
    ensure_storage_dir()?;
    let json = serde_json::to_string_pretty(todos).map_err(|err| format!("Could not encode todos: {err}"))?;
    fs::write(todos_path()?, format!("{json}\n")).map_err(|err| format!("Could not write todos.json: {err}"))?;
    touch_last_modified()
}

fn touch_last_modified() -> Result<(), String> {
    ensure_storage_dir()?;
    fs::write(last_modified_path()?, format!("{}\n", Utc::now().to_rfc3339()))
        .map_err(|err| format!("Could not update last_modified: {err}"))
}

#[tauri::command]
fn get_todos(state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    let _guard = state.storage_lock.lock().map_err(|_| "Storage lock poisoned".to_string())?;
    read_todos_from_disk()
}

#[tauri::command]
fn add_todo(text: String, state: State<'_, AppState>) -> Result<Todo, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("Todo text cannot be empty".to_string());
    }

    let _guard = state.storage_lock.lock().map_err(|_| "Storage lock poisoned".to_string())?;
    let mut todos = read_todos_from_disk()?;
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        text: text.to_string(),
        done: false,
        created_at: Utc::now().to_rfc3339(),
    };

    todos.push(todo.clone());
    write_todos_to_disk(&todos)?;
    Ok(todo)
}

#[tauri::command]
fn toggle_todo(id: String, state: State<'_, AppState>) -> Result<Todo, String> {
    let _guard = state.storage_lock.lock().map_err(|_| "Storage lock poisoned".to_string())?;
    let mut todos = read_todos_from_disk()?;
    let todo = todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .ok_or_else(|| "Todo not found".to_string())?;

    todo.done = !todo.done;
    let updated = todo.clone();
    write_todos_to_disk(&todos)?;
    Ok(updated)
}

#[tauri::command]
fn delete_todo(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let _guard = state.storage_lock.lock().map_err(|_| "Storage lock poisoned".to_string())?;
    let mut todos = read_todos_from_disk()?;
    let original_len = todos.len();
    todos.retain(|todo| todo.id != id);

    if todos.len() == original_len {
        return Err("Todo not found".to_string());
    }

    write_todos_to_disk(&todos)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.close();
            } else {
                app.exit(0);
            }
        }))
        .manage(AppState {
            storage_lock: Mutex::new(()),
        })
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            toggle_todo,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running Akari");
}
