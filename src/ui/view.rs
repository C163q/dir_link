use ratatui::prelude::*;
use ratatui::style::Styled;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, HighlightSpacing, List, ListItem, ListState, Paragraph, Row,
    Table, TableState, Wrap,
};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::ui::App;
use crate::ui::state::{AppState, FolderEditState, InputMode};

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
        None => {
            let area = common::center(area, Constraint::Length(5), Constraint::Length(1));
            render_right_list_empty(area, buf);
        }
        Some(idx) => {
            // TODO: style
            let header = ["Name", "Path"]
                .into_iter()
                .map(Cell::from)
                .collect::<Row>()
                .height(1);

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
    };
}

pub fn render_right_list_empty(area: Rect, buf: &mut Buffer) {
    Text::raw("Empty")
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .centered()
        .render(area, buf);
}

pub fn render_folder_edit(
    state: &mut FolderEditState,
    area: Rect,
    buf: &mut Buffer,
) -> Option<(u16, u16)> {
    let edit_type = match state.list_state().selected() {
        Some(_) => "Rename".set_style(Color::Cyan),
        None => "Append".set_style(Color::Green),
    };
    let edit_state = match state.mode() {
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

    let chunks = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Min(3)])
        .margin(1)
        .split(area);

    let text = Line::from("Input Folder Name:")
        .left_aligned()
        .style(Style::default().fg(Color::White));
    text.render(chunks[0], buf);

    let mode = state.mode().to_owned();
    let input = state.input_mut();
    let width = chunks[1].width.saturating_sub(3);
    let scroll = input.visual_scroll(width as usize);

    let style = match mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    };
    let input_text = Paragraph::new(input.value())
        .style(style)
        .scroll((0, scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(style)
                .title_top(Line::from("Input").centered().style(style))
                .style(style),
        )
        .wrap(Wrap { trim: false });
    input_text.render(chunks[1], buf);

    if mode == InputMode::Editing {
        let x = input.visual_cursor().max(scroll) - scroll + 1;
        Some((chunks[1].x + x as u16, chunks[1].y + 1))
    } else {
        None
    }
}
