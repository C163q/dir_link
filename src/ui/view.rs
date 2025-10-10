use ratatui::prelude::*;
use ratatui::widgets::{
    Block, BorderType, Borders, Cell, HighlightSpacing, List, ListItem, ListState, Row, Table,
    TableState,
};
use ratatui::{buffer::Buffer, layout::Rect};

use crate::ui::App;
use crate::ui::state::AppState;

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
            .map(|dir| ListItem::new(dir.identifier())),
    );

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
            .row_highlight_style(Style::default().bg(Color::Blue))
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
