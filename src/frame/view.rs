use iced::widget::{button, Column, Row, Text, text_editor};
use iced::{Fill, Renderer, Theme};
use iced_aw::{menu::{Item, Menu}, MenuBar};
use std::path::Path;
use crate::frame::style::style::{button_active_style, button_style, transparent_button_style, text_editor_style};
use crate::window::{Message, State};



pub fn view(state: &State) -> Column<Message> {

    //Toolbar
    let file_menu = Item::<Message, Theme, Renderer>::with_menu(
        button(Text::new("File")).style(button_style),
        Menu::new(vec![
            Item::<Message, Theme, Renderer>::new(
                button(Text::new("New File")).on_press(Message::NewFile).style(transparent_button_style).width(Fill),
            ),
            Item::<Message, Theme, Renderer>::new(
                button(Text::new("Open file")).on_press(Message::OpenFile).style(transparent_button_style).width(Fill),
            ),
            Item::<Message, Theme, Renderer>::new(
                button(Text::new("Open folder")).on_press(Message::OpenFolder).style(transparent_button_style).width(Fill),
            ),
            Item::<Message, Theme, Renderer>::new(
                button(Text::new("Save all")).on_press(Message::SaveAll).style(transparent_button_style).width(Fill),
            ),
            Item::<Message, Theme, Renderer>::new(
                button(Text::new("Save")).on_press(Message::SaveFile).style(transparent_button_style).width(Fill),
            ),
        ]).max_width(150.0),
    );
    let view_menu = Item::<Message, Theme, Renderer>::with_menu(
        button(Text::new("View")).style(button_style),
        Menu::new(vec![
            Item::<Message, Theme, Renderer>::new(
                button(Text::new("Toggle side bar")).on_press(Message::ViewDirectorys).style(transparent_button_style).width(Fill),
            ),
        ]).max_width(150.0),
    );

    let menu_bar = MenuBar::new(vec![file_menu, view_menu]).width(Fill);

    //Directory
    let left_column = if state.enable_folder_tree {
        let directory_files = Column::with_children(
            state.current_folder_files
                .iter()
                .map(|file| {
                    let file_name = Path::new(file)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(file);

                    button(file_name)
                        .style(transparent_button_style)
                        .width(Fill)
                        .on_press(Message::OpenFileString(file.clone()))
                        .into()
                }).collect::<Vec<_>>(),
        );
        let directory_folders = Column::with_children(
            state.current_folder_folders
                .iter()
                .map(|dir| {
                    let dir_name = Path::new(dir)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(dir);
                         let display_name = format!("| {}", dir_name);
                    button(Text::new(display_name))
                        .style(transparent_button_style)
                        .width(Fill)
                        .on_press(Message::OpenFolderByString(dir.clone()))
                        .into()
                }).collect::<Vec<_>>(),
        );
        if state.current_folder != None {
            Column::new()
                .push(button("| ..").style(transparent_button_style).on_press(Message::OpenFolderByString(
                    Path::new(state.current_folder.as_deref().unwrap_or("/")).parent().unwrap_or_else(|| Path::new("/")).to_string_lossy().to_string()
                )).width(Fill))
                .push(directory_folders)
                .push(directory_files)
                .width(200)
                .spacing(5)
        } else {
            Column::new().push(button("").style(transparent_button_style)).width(200)
        }

    } else {
        Column::new().width(0)
    };

    // Opened file, editor, message
    let file_tabs = Row::with_children(
        state.opened_files
            .iter()
            .flat_map(|(path, _content)| {
                let file_name = Path::new(path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Untitled");
                let is_active = state.current_file_path.as_ref()
                    .map(|current| Path::new(current) != Path::new(path))
                    .unwrap_or(false);
                vec![
                    button(file_name)
                        .style(if is_active {
                            button_style
                        } else {
                            button_active_style
                        })
                        .on_press(Message::ChangeMainFile(path.clone()))
                        .into(),
                    button("x")
                        .style(if is_active {
                            button_style
                        } else {
                            button_active_style
                        })
                        .on_press(Message::CloseFile(path.clone()))
                        .into(),
                ]
            })
            .collect::<Vec<_>>(),
    );
    let editor = text_editor(&state.current_file_content)
        .style(text_editor_style)
        .on_action(Message::Edit)
        .height(Fill)
        .highlight(
            state.current_file_path
                .as_ref()
                .and_then(|path| Path::new(path).extension()?.to_str())
                .unwrap_or(""),
            iced::highlighter::Theme::Base16Ocean,
        );
    let right_column = Column::new()
        .push(file_tabs)
        .push(editor)
        .push(Text::new(state.message.clone()));

    let second_row = Row::new()
        .push(left_column)
        .push(right_column);

    Column::new()
        .push(menu_bar)
        .push(second_row)
        .spacing(14)
}
