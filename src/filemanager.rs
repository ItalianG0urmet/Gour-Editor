use std::fs;
use iced::widget::text_editor;
use rfd::FileDialog;
use crate::window::State;

pub fn open_file(state: &mut State){
    let old_file =state.current_file_path.clone();

    if let Some(path) = FileDialog::new().pick_file() {
        state.current_file_path = Some(path.display().to_string());
    }

    if let Some(path) = &state.current_file_path {
        match fs::read_to_string(path) {
            Ok(content) => {
                state.current_file_content = text_editor::Content::with_text(&content);
                state.opened_files.insert(String::from(path), content);
                state.message = format!("Selected string: {}", path)
            },
            Err(_) => {
                state.message = "Can't open this file".to_string();
                state.current_file_path = old_file.clone();
            }
        }
    } else {
        state.message = "No file selected".to_string()
    }
}

pub fn open_directory(state: &mut State){
    let old_folder =  state.selected_folder.clone();
    state.selected_folder_files.clear();

    if let Some(path) = FileDialog::new().pick_folder() {
        let folder_path = path.display().to_string();
        state.selected_folder = Some(folder_path.clone());

        match fs::read_dir(&folder_path) {
            Ok(entries) => {
                let mut files = Vec::new();
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        let path = entry.path();
                        if file_type.is_file() {
                            files.push(path.display().to_string());
                        }
                    }
                }
                state.selected_folder_files = files;
                state.message = format!("Selected folder: {}", folder_path);
            },
            Err(_) => {
                state.message = "Can't open this folder".to_string();
                state.selected_folder = old_folder;
            }
        }
    } else {
        state.message = "No folder selected".to_string();
    }
}

pub fn save_file(state: &mut State){
    let path = if let Some(path) = &state.current_file_path {
        path.clone()
    } else if let Some(path) = FileDialog::new().save_file() {
        path.display().to_string()
    } else {
        state.message = "No file selected to save".to_string();
        return;
    };

    let content = state.current_file_content.text().to_owned();
    match fs::write(&path, content){
        Ok(_) => state.message = "File saved".to_string(),
        Err(_) => state.message = "Error saving file".to_string(),
    }
}

pub fn open_file_by_string(state: &mut State, path: String){
    let path_clone = path.clone();
    match fs::read_to_string(path) {
        Ok(content) => {
            state.current_file_content = text_editor::Content::with_text(&content);
            state.current_file_path = Some(path_clone);
        },
        Err(_) => {
            state.message = "Can't open this file".to_string();
        }
    }
}