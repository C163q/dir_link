pub mod common;

use std::ops::Add;

use ratatui::prelude::*;
use ratatui::style::Styled;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, Clear, HighlightSpacing, List, ListItem, ListState,
    Paragraph, Row, Table, TableState, Wrap,
};
use ratatui::{buffer::Buffer, layout::Rect};
use tui_input::Input;
use unicode_width::UnicodeWidthStr;

use crate::app::App;
use crate::app::data::CursorCache;
use crate::app::float::confirm::{
    ConfirmChoice, FolderDeleteConfirmState, FolderSaveConfirmState, LinkDeleteConfirmState,
    LinkSaveConfirmState,
};
use crate::app::float::edit::{FolderEditState, LinkEditState};
use crate::app::float::warning::{CorruptDataWarningChoice, CorruptDataWarningState, WarningState};
use crate::app::normal::{FolderNormalState, InputMode, InputPart, LinkNormalState};
use crate::data::dir::LinkDir;
use crate::data::dirset::LinkDirSet;

pub fn render_main_border(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .title_top(Line::styled("Dir Link", Style::default().fg(Color::Yellow)).left_aligned())
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White));
    block.render(area, buf);
}

pub fn render_main_divider(area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .borders(Borders::RIGHT | Borders::LEFT)
        .border_style(Style::default().fg(Color::White));
    block.render(area, buf);
}

pub fn render_left_list<'a>(
    app: &'a mut App,
    area: Rect,
    buf: &mut Buffer,
    default: &'a mut ListState,
) {
    let list = List::new(
        app.data
            .map()
            .iter()
            .map(|dir| ListItem::new(dir.identifier().set_style(Color::Cyan))),
    )
    .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
    .highlight_spacing(HighlightSpacing::Always);

    let left_state = app.state.folder_list_state_mut().unwrap_or(default);
    <List as StatefulWidget>::render(list, area, buf, left_state);
}

pub fn render_right_list<'a>(
    app: &'a mut App,
    area: Rect,
    buf: &mut Buffer,
    default: &'a mut TableState,
) {
    let idx = app
        .state
        .folder_list_state()
        .and_then(|s| s.selected())
        .filter(|&idx| idx < app.data.len());

    match idx {
        Some(idx) if !app.data[idx].is_empty() => {
            let header_style = if app.state.is_link() {
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            };

            // TODO: style
            let header = ["Name", "Path"]
                .into_iter()
                .map(Cell::from)
                .collect::<Row>()
                .height(1)
                .style(header_style);

            let rows = app.data[idx].iter().map(|link| {
                let identifier = link.identifier();
                let path = link.path().to_string_lossy();
                let item = [identifier.to_string(), path.to_string()];
                item.into_iter().map(Cell::from).collect::<Row>().height(1)
            });
            let table = Table::new(
                rows,
                [Constraint::Percentage(30), Constraint::Percentage(70)],
            )
            .header(header)
            .row_highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
            .highlight_spacing(HighlightSpacing::Always);

            let select = app.state.link_table_state_mut().unwrap_or(default);

            <Table as StatefulWidget>::render(table, area, buf, select);
        }
        _ => {
            let area = common::center(area, Constraint::Length(5), Constraint::Length(1));
            let focused = app.state.is_link();
            render_right_list_empty(area, buf, focused);
        }
    };
}

pub fn render_right_list_empty(area: Rect, buf: &mut Buffer, focused: bool) {
    let style = if focused {
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    };
    Text::raw("Empty").style(style).centered().render(area, buf);
}

pub fn render_input(
    input: &mut Input,
    hint_message: &str,
    input_mode: InputMode,
    area: Rect,
    buf: &mut Buffer,
    ghost_text: Option<&str>,
    cursor_cache: &mut CursorCache,
) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Min(3)])
        .split(area);
    let text = Line::from(hint_message)
        .left_aligned()
        .style(Style::default().fg(Color::White));
    text.render(chunks[0], buf);

    let width = chunks[1].width.saturating_sub(3).max(1);
    let scroll = input.visual_scroll(width as usize);

    let style = match input_mode {
        InputMode::Normal => Style::reset().fg(Color::White),
        InputMode::Editing => Style::reset().fg(Color::Yellow),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(style)
        .title_top(Line::from("Input").centered().style(style))
        .style(style);

    let input_text = match ghost_text {
        None => Paragraph::new(input.value())
            .style(Style::default())
            .scroll((0, scroll as u16))
            .block(block)
            .wrap(Wrap { trim: false }),
        Some(text) if input.value().is_empty() => Paragraph::new(text)
            .style(Style::default().fg(Color::DarkGray).italic())
            .scroll((0, scroll as u16))
            .block(block)
            .wrap(Wrap { trim: false }),
        Some(_) => Paragraph::new(input.value())
            .style(Style::default())
            .scroll((0, scroll as u16))
            .block(block)
            .wrap(Wrap { trim: false }),
    };
    input_text.render(chunks[1], buf);

    if input_mode == InputMode::Editing && cursor_cache.is_outdated() {
        // FIXME:
        // 目前仍然存在多宽度字符时光标位置计算不准确的问题
        //
        // border                        一格位置显示不了一个完整字符
        //  ↓                                          ↓
        //  |字字字字字字字字字字字字字字字字字字字字字 |
        //  |字字█ ←该方块为光标应该在的位置            |
        //      ↑
        //  光标实际位置
        //
        //  计算的时候吧无法显示完整字符的部分也算进去了
        let text_idx = input
            .value()
            .char_indices()
            .nth(input.cursor())
            .map_or_else(|| input.value().len(), |(i, _)| i);
        let text = input.value().split_at(text_idx).0;

        let lines: Vec<_> = text.lines().collect();
        let mut lines_width: Vec<_> = lines
            .iter()
            .map(|&line| UnicodeWidthStr::width(line))
            .collect();
        if let Some(value) = lines_width.last_mut() {
            *value = value.add(1);
        }

        let text_area = Rect {
            x: chunks[1].x + 1,
            y: chunks[1].y + 1,
            width: chunks[1].width.saturating_sub(2),
            height: chunks[1].height.saturating_sub(2),
        };

        let lines_height: Vec<_> = lines_width
            .iter()
            .map(|&w| w.div_ceil(text_area.width as usize))
            .collect();
        let x_offset = (lines_width
            .last()
            .cloned()
            .unwrap_or_default()
            .saturating_sub(1)
            % text_area.width as usize) as u16;
        let y_offset = lines_height
            .iter()
            .sum::<usize>()
            .saturating_sub(1)
            .min(text_area.height as usize) as u16;

        cursor_cache.update(text_area.x + x_offset, text_area.y + y_offset);
    }
}

pub fn render_input_block(select: Option<usize>, mode: &InputMode, area: Rect, buf: &mut Buffer) {
    let edit_type = match select {
        Some(_) => "Rename".set_style(Color::Cyan),
        None => "Append".set_style(Color::Green),
    };
    let edit_state = match mode {
        InputMode::Editing => "E".set_style(Color::Yellow).bold(),
        InputMode::Normal => "N".set_style(Color::Yellow).bold(),
    };

    let block = Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .title_top(Line::from("Edit").centered())
        .title_bottom(Line::from(edit_type).left_aligned())
        .title_bottom(Line::from(edit_state).right_aligned())
        .border_type(BorderType::Thick);
    block.render(area, buf);
}

pub fn render_folder_edit(
    state: &mut FolderEditState,
    area: Rect,
    buf: &mut Buffer,
    cursor_cache: &mut CursorCache,
) {
    render_input_block(state.selected(), state.mode(), area, buf);

    let mode = state.mode().to_owned();
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .margin(1)
        .split(area)[0];
    render_input(
        state.input_mut(),
        "Input Folder Name:",
        mode,
        chunk,
        buf,
        None,
        cursor_cache,
    );
}

pub fn render_link_edit(
    state: &mut LinkEditState,
    area: Rect,
    buf: &mut Buffer,
    cursor_cache: &mut CursorCache,
) {
    render_input_block(state.selected(), state.mode(), area, buf);

    let mode = state.mode().to_owned();
    let key_mode = match state.part() {
        InputPart::Key => mode,
        InputPart::Value => InputMode::Normal,
    };
    let value_mode = match state.part() {
        InputPart::Key => InputMode::Normal,
        InputPart::Value => mode,
    };

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .margin(1)
        .split(area);
    render_input(
        state.key_input_mut(),
        "Input Link Name",
        key_mode,
        chunks[0],
        buf,
        None,
        cursor_cache,
    );
    render_input(
        state.value_input_mut(),
        "Input Link Path",
        value_mode,
        chunks[1],
        buf,
        Some("empty for current directory"),
        cursor_cache,
    );
}

pub fn render_confirm_yes_no_choice(area: Rect, buf: &mut Buffer, choice: ConfirmChoice) {
    let messages = ["Yes(Y)", "No(N)"];
    let choice = match choice {
        ConfirmChoice::Yes => 0,
        ConfirmChoice::No => 1,
    };
    common::render_comfirm_choice(area, buf, messages, choice, (1, 2));
}

pub fn render_folder_delete_confirm_float<F>(
    state: &FolderDeleteConfirmState<F>,
    area: Rect,
    buf: &mut Buffer,
) where
    F: FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet),
{
    render_confirm_border(area, buf);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let hint_message = "Are you sure to DELETE this folder?";
    render_confirm_message(chunks[0], buf, hint_message);

    render_confirm_yes_no_choice(chunks[1], buf, state.choice());
}

pub fn render_link_delete_confirm_float<F>(
    state: &LinkDeleteConfirmState<F>,
    area: Rect,
    buf: &mut Buffer,
) where
    F: FnOnce(ConfirmChoice, &mut LinkNormalState, &mut LinkDir),
{
    render_confirm_border(area, buf);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let hint_message = "Are you sure to DELETE this link?";
    render_confirm_message(chunks[0], buf, hint_message);

    render_confirm_yes_no_choice(chunks[1], buf, state.choice());
}

pub fn render_confirm_border(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .title_top(
            Line::from("Confirm")
                .style(Style::default().fg(Color::Yellow))
                .centered(),
        );
    block.render(area, buf);
}

pub fn render_confirm_message(area: Rect, buf: &mut Buffer, message: &str) {
    let text = Text::from(
        message
            .lines()
            .map(|line| Line::from(line.trim_end()))
            .collect::<Vec<_>>(),
    );

    let paragraph = Paragraph::new(text)
        .centered()
        .wrap(Wrap { trim: false })
        .style(Color::LightRed)
        .add_modifier(Modifier::BOLD);
    paragraph.render(common::centered_text(message, area, 0, 0), buf);
}

pub fn render_warning_float(state: &mut WarningState, area: Rect, buf: &mut Buffer) {
    let hint_message = "Press <Esc>/<Q> to Quit";

    let chunk = common::render_border(
        Some(Line::from("Warning").style(Style::default().fg(Color::Yellow))),
        Some(Line::from(hint_message).style(Style::default().fg(Color::LightGreen))),
        Style::default().fg(Color::White),
        area,
        buf,
    );

    let message = state.message();
    let paragraph = Paragraph::new(message)
        .centered()
        .wrap(Wrap { trim: false })
        .style(Color::LightRed)
        .add_modifier(Modifier::BOLD);
    paragraph.render(common::centered_text(message, chunk, 0, 0), buf);
}

pub fn render_folder_save_confirm_float(
    state: &mut FolderSaveConfirmState,
    edit_area: Rect,
    area: Rect,
    buf: &mut Buffer,
    cursor_cache: &mut CursorCache,
) {
    render_folder_edit(state.last_state_mut(), edit_area, buf, cursor_cache);

    Clear.render(area, buf);
    render_confirm_border(area, buf);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let hint_message = "Are you sure to quit without saving?";
    render_confirm_message(chunks[0], buf, hint_message);

    render_confirm_yes_no_choice(chunks[1], buf, state.choice());
}

pub fn render_link_save_confirm_float(
    state: &mut LinkSaveConfirmState,
    edit_area: Rect,
    area: Rect,
    buf: &mut Buffer,
    cursor_cache: &mut CursorCache,
) {
    render_link_edit(state.last_state_mut(), edit_area, buf, cursor_cache);

    Clear.render(area, buf);
    render_confirm_border(area, buf);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let hint_message = "Are you sure to quit without saving?";
    render_confirm_message(chunks[0], buf, hint_message);

    render_confirm_yes_no_choice(chunks[1], buf, state.choice());
}

pub fn render_corrupt_data_warning_float(
    state: &mut CorruptDataWarningState,
    area: Rect,
    buf: &mut Buffer,
) {
    let hint_message = "Choose an option and press <Enter>";

    let chunk = common::render_border(
        Some(Line::from("Corrupt Data Error").style(Style::default().fg(Color::Red))),
        Some(Line::from(hint_message).style(Style::default().fg(Color::LightGreen))),
        Style::default().fg(Color::White),
        area,
        buf,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk);

    let hint_message = String::from("Data corrupt!\nError message: ") + state.message();
    render_confirm_message(area, buf, &hint_message);

    let messages = ["Exit", "Create New Data"];
    let choice = match state.choice() {
        CorruptDataWarningChoice::Exit => 0,
        CorruptDataWarningChoice::NewData => 1,
    };
    common::render_comfirm_choice(chunks[1], buf, messages, choice, (1, 2));
}
