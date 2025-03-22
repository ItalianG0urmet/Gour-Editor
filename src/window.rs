use std::collections::HashMap;
use std::fs;
use rfd::FileDialog;
use iced::widget::text_editor;

#[derive(Debug, Default)]
pub struct State {
    pub(crate) message: String,
    pub(crate) current_file_content:  text_editor::Content,
    pub(crate) current_file_path: Option<String>,
    pub(crate) opened_files: HashMap<String, String>, //Path and Content

    pub(crate) selected_folder: Option<String>,
    pub(crate) selected_folder_files: Vec<String>,
    pub(crate) enable_directorys_view: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    OpenFolder,
    OpenFile,
    OpenFileString(String),
    SaveFile,
    NewFile,
    ViewDirectorys,
    ChangeMainFile(String),
    CloseFile(String)
}

pub fn update(state: &mut State, message: Message){
    match message{
        Message::CloseFile(path) => {
            state.opened_files.remove(&path);
        }
        Message::Edit(action) => {
            state.current_file_content.perform(action);
            if let Some(path) = &state.current_file_path {
                if let Some(value) = state.opened_files.get_mut(path) {
                    *value = state.current_file_content.text();
                };
                if !state.opened_files.contains_key(path){
                    state.opened_files.insert(String::from(path), state.current_file_content.text());
                }
            }
        }
        Message::OpenFile =>{

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
        Message::OpenFolder => {
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
        Message::SaveFile => {
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
        Message::NewFile => {
            state.current_file_content = text_editor::Content::with_text("");
            state.current_file_path = None;
            state.message = "New file created".to_string();
        }
        Message::OpenFileString(path) => {

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
        Message::ViewDirectorys => {

            state.enable_directorys_view = !state.enable_directorys_view;

        }
        Message::ChangeMainFile(path) => {
            if let Some(new_content) = state.opened_files.get(&path){
                if state.current_file_path == Some(path.clone()) {
                    state.message = "Already in file".to_string();
                    return;
                }
                state.current_file_content = text_editor::Content::with_text(&new_content);
                state.current_file_path = path.into();
            } else {
                state.message = "Error".to_string();
            }

        }
    }
}
