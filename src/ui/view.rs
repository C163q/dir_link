use ratatui::prelude::*;
use ratatui::style::Styled;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, HighlightSpacing, List, ListItem, ListState, Paragraph, Row,
    Table, TableState, Wrap,
};
use ratatui::{buffer::Buffer, layout::Rect};
use tui_input::Input;

use crate::data::dir::LinkDir;
use crate::data::dirset::LinkDirSet;
use crate::ui::App;
use crate::ui::float::confirm::{ConfirmChoice, FolderDeleteConfirmState, LinkDeleteConfirmState};
use crate::ui::state::{
    AppState, FolderEditState, FolderNormalState, InputMode, InputPart, LinkEditState, LinkNormalState,
};

pub mod common;

pub fn render_border(app: &App, area: Rect, buf: &mut Buffer) {
    let state_hint = match &app.state {
        AppState::Normal(_) => "normal",
        AppState::Edit(_) => "edit",
        AppState::Quit(_) => "quit",
    };

    let block = Block::bordered()
        .title_top(Line::styled("Dir Link", Style::default().fg(Color::Yellow)).left_aligned())
        .title_bottom(Line::styled(state_hint, Style::default().fg(Color::Green)).right_aligned())
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White));
    block.render(area, buf);
}

pub fn render_divider(area: Rect, buf: &mut Buffer) {
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
) -> Option<(u16, u16)> {
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
        InputMode::Normal => Style::default().fg(Color::White),
        InputMode::Editing => Style::default().fg(Color::Yellow),
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

    if input_mode == InputMode::Editing {
        let x = input.visual_cursor().saturating_sub(scroll) + 1;
        let (x_offset, y_offset) = ((x % width as usize) as u16, (x / width as usize) as u16);
        Some((chunks[1].x + x_offset, chunks[1].y + 1 + y_offset))
    } else {
        None
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
) -> Option<(u16, u16)> {
    render_input_block(state.list_state().selected(), state.mode(), area, buf);

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
    )
}

pub fn render_link_edit(
    state: &mut LinkEditState,
    area: Rect,
    buf: &mut Buffer,
) -> Option<(u16, u16)> {
    render_input_block(state.table_state().selected(), state.mode(), area, buf);

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
    let key_pos = render_input(
        state.key_input_mut(),
        "Input Link Name",
        key_mode,
        chunks[0],
        buf,
        None,
    );
    let value_pos = render_input(
        state.value_input_mut(),
        "Input Link Path",
        value_mode,
        chunks[1],
        buf,
        Some("empty for current directory"),
    );

    key_pos.or(value_pos)
}

pub fn render_folder_delete_confirm_float<F>(
    state: &FolderDeleteConfirmState<F>,
    area: Rect,
    buf: &mut Buffer,
) where
    F: FnOnce(ConfirmChoice, &mut FolderNormalState, &mut LinkDirSet),
{
    render_conform_border(area, buf);

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
    render_conform_border(area, buf);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let hint_message = "Are you sure to DELETE this link?";
    render_confirm_message(chunks[0], buf, hint_message);

    render_confirm_yes_no_choice(chunks[1], buf, state.choice());
}

pub fn render_conform_border(area: Rect, buf: &mut Buffer) {
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
    let paragraph = Paragraph::new(message)
        .centered()
        .wrap(Wrap { trim: false })
        .style(Color::LightRed)
        .add_modifier(Modifier::BOLD);
    paragraph.render(common::vertical_centered_text(message, area, 0, 0), buf);
}

pub fn render_confirm_yes_no_choice(area: Rect, buf: &mut Buffer, choice: ConfirmChoice) {
    let choice_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let yes_message = "Yes(Y)";
    let no_message = "No(N)";
    let highlight_style = Style::default().fg(Color::Black).bg(Color::Cyan);

    let (yes_style, no_style) = match choice {
        ConfirmChoice::Yes => (highlight_style, Style::default()),
        ConfirmChoice::No => (Style::default(), highlight_style),
    };

    let yes_paragraph = Paragraph::new(yes_message)
        .centered()
        .style(Style::default())
        .wrap(Wrap { trim: false })
        .block(Block::bordered().border_type(BorderType::Plain))
        .style(yes_style);
    let no_paragraph = Paragraph::new(no_message)
        .centered()
        .style(Style::default())
        .wrap(Wrap { trim: false })
        .block(Block::bordered().border_type(BorderType::Plain))
        .style(no_style);
    yes_paragraph.render(
        common::vertical_centered_text(yes_message, choice_areas[0], 2, 2),
        buf,
    );
    no_paragraph.render(
        common::vertical_centered_text(no_message, choice_areas[1], 2, 2),
        buf,
    );
}
