use std::fs;
use std::path::Path;
use iced::highlighter::{Theme};
use rfd::FileDialog;
use iced::widget::{column, Column, text_editor, button, text, row};
use crate::style::{button_style, text_editor_style};

#[derive(Debug, Default)]
pub struct State {
    message: String,
    content: text_editor::Content,
    file: Option<String>
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    OpenFile,
    SaveFile,
    NewFile,
}

pub fn update(state: &mut State, message: Message){
    match message{
        Message::Edit(action) => {
            state.content.perform(action)
        }
        Message::OpenFile =>{

            let old_file =state.file.clone();

            if let Some(path) = FileDialog::new().pick_file() {
                state.file = Some(path.display().to_string());
            }

            if let Some(path) = &state.file {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        state.content = text_editor::Content::with_text(&content);
                        state.message = format!("Selected string: {}", path)
                    },
                    Err(_) => {
                        state.message = "Can't open this file".to_string();
                        state.file = old_file.clone();
                    }
                }
            } else {
                state.message = "No file selected".to_string()
            }
        }
        Message::SaveFile => {
            let path = if let Some(path) = &state.file{
                path.clone()
            } else if let Some(path) = FileDialog::new().save_file() {
                path.display().to_string()
            } else {
                state.message = "No file selected to save".to_string();
                return;
            };

            let content = state.content.text().to_owned();
            match fs::write(&path, content){
                Ok(_) => state.message = "File saved".to_string(),
                Err(_) => state.message = "Error saving file".to_string(),
            }
        }
        Message::NewFile => {
            state.content = text_editor::Content::with_text("");
            state.file = None;
            state.message = "New file created".to_string();
        }
    }
}


pub fn view(state: &State) -> Column<Message>{

    column![
        row![
            button("Open").style(button_style).on_press(Message::OpenFile),
            button("Save").style(button_style).on_press(Message::SaveFile),
            button("New File").style(button_style).on_press(Message::NewFile),

        ].spacing(0),

        text(state.message.clone()),
        text_editor(&state.content).style(text_editor_style)
            .on_action(Message::Edit).height(10000).highlight(state.file.as_ref()
            .and_then(|path| Path::new(path).extension()?.to_str())
            .unwrap_or("txt"), Theme::SolarizedDark)
    ].into()
}