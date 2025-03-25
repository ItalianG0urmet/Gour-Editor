use std::collections::HashMap;
use iced::widget::text_editor;
use crate::filemanager::{open_file, open_file_by_string, save_file, open_directory, open_directory_by_string, save_all_file};

#[derive(Debug, Default)]
pub struct State {
    pub(crate) message: String,
    pub(crate) current_file_content:  text_editor::Content,
    pub(crate) current_file_path: Option<String>,
    pub(crate) opened_files: HashMap<String, String>, //Path and Content

    pub(crate) selected_folder: Option<String>,
    pub(crate) selected_folder_files: Vec<String>,
    pub(crate) selected_folder_folders: Vec<String>,
    pub(crate) enable_directorys_view: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    OpenFolder,
    OpenFolderByString(String),
    OpenFile,
    OpenFileString(String),
    SaveFile,
    NewFile,
    ViewDirectorys,
    ChangeMainFile(String),
    CloseFile(String),
    SaveAll,
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
        Message::OpenFolderByString(path) => {
            open_directory_by_string(state, path);

        }
        Message::OpenFile =>{
            open_file(state);
        }
        Message::OpenFolder => {
            open_directory(state);
        }
        Message::SaveFile => {
            save_file(state);
        }
        Message::NewFile => {
            state.current_file_content = text_editor::Content::with_text("");
            state.current_file_path = None;
            state.message = "New file created".to_string();
        }
        Message::OpenFileString(path) => {
            open_file_by_string(state, path);
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
                state.current_file_content = text_editor::Content::with_text(&new_content.clone());
                state.current_file_path = path.into();
            } else {
                state.message = "Error".to_string();
            }

        }
        Message::SaveAll => {
            save_all_file(state);
            save_file(state);
        }   
    }
}
