use std::fs;
use iced::{Theme};
use rfd::FileDialog;
use iced::widget::{column, Column, text_editor, button, text, row, Row, Button, Text};
use crate::style::transparent_style;

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
}

pub fn update(state: &mut State, message: Message){
    match message{
        Message::Edit(action) => {
            state.content.perform(action)
        }
        Message::OpenFile =>{
            if let Some(path) = FileDialog::new().pick_file() {
                state.file = Some(path.display().to_string());
            }
            
            if let Some(path) = &state.file {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        state.content = text_editor::Content::with_text(&content);
                        state.message = format!("Selected string: {}", path)
                    },
                    Err(_) => state.message = format!("Can't open this file")
                }
            } else {
                state.message = format!("No file selected")
            }
        }
        Message::SaveFile => {
            let path = if let Some(path) = &state.file{
                path.clone()
            } else if let Some(path) = FileDialog::new().save_file() {
                path.display().to_string()
            } else {
                state.message = format!("No file selected to save");
                return;
            };

            let content = state.content.text().to_owned();
            match fs::write(&path, content){
                Ok(_) => state.message = format!("File saved"),
                Err(_) => state.message = format!("Error saving file"),
            }
        }
    }
}

pub fn view(state: &State) -> Column<Message>{

    column![
        row![
            button("Open").style(transparent_style).on_press(Message::OpenFile),
            button("Save").style(transparent_style).on_press(Message::SaveFile),

        ].spacing(0),

        text(state.message.clone()),
        text_editor::<Message, Theme, iced::Renderer>(&state.content)
            .on_action(Message::Edit)
    ].into()
}

