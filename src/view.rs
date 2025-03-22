use iced::widget::{button, column, row, text, text_editor, Column};
use std::path::Path;
use iced::highlighter::Theme;
use crate::style::{button_active_style, button_style, text_editor_style};
use crate::window::{Message, State};

pub fn view(state: &State) -> Column<Message>{

    column![
        row![
            button("Open").style(button_style).on_press(Message::OpenFile),
            button("Open Folder").style(button_style).on_press(Message::OpenFolder),
            button("New File").style(button_style).on_press(Message::NewFile),
            button("Save").style(button_style).on_press(Message::SaveFile),
            button("View Dir").style(button_style).on_press(Message::ViewDirectorys),
            button("").style(button_style).width(1000)
        ].spacing(0),
        row![
            if state.enable_directorys_view {
                column![
                    text("directory:"),
                    button("..").style(button_style).on_press(Message::SaveFile).width(200),
                    Column::with_children(
                        state.selected_folder_files.iter().map(|file| {
                            let file_name = Path::new(file)
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or(file);
                            button(file_name)
                                .style(button_style)
                                .width(200)
                                .on_press(Message::OpenFileString(file.clone()))
                                .into()
                        }).collect::<Vec<_>>()
                    )
                ].width(200).spacing(5)
            } else {
                Column::new().width(0)
            },


            column![
                text(state.message.clone()),
                row![
                    Column::with_children(
                        state.opened_files.iter().map(|(path, content)| {
                            let file_name = Path::new(path)
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Untitled");

                            let is_active = state.current_file_path.as_ref()
                                .map(|current| Path::new(current) != Path::new(path))
                                .unwrap_or(false);


                            button(file_name)
                                .style(if is_active {
                                        button_style
                                    }else{
                                        button_active_style
                                    })
                                .on_press(Message::ChangeMainFile(path.clone()))
                                .into()
                        }).collect::<Vec<_>>()
                    ),
                ].spacing(0),

                text_editor(&state.current_file_content).style(text_editor_style)
                    .on_action(Message::Edit).height(10000).highlight(state.current_file_path.as_ref()
                    .and_then(|path| Path::new(path).extension()?.to_str())
                    .unwrap_or("txt"), Theme::SolarizedDark)
            ]
        ]
    ].spacing(10).into()
}