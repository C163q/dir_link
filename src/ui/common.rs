use ratatui::{
    layout::Flex,
    prelude::*,
    widgets::{Block, BorderType, Paragraph, Wrap},
};
use unicode_width::UnicodeWidthStr;

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn centered_text(text: &str, area: Rect, additional_x: u16, additional_y: u16) -> Rect {
    let lines: Vec<_> = text.lines().collect();
    let lines_width: Vec<_> = lines
        .iter()
        .map(|&line| UnicodeWidthStr::width(line))
        .collect();

    let area_width = area.width.saturating_sub(additional_x).max(1);
    let line_height = lines_width
        .iter()
        .map(|&w| w.div_ceil(area_width as usize))
        .sum::<usize>() as u16;
    let line_width = lines_width.iter().max().cloned().unwrap_or(1) as u16;

    center(
        area,
        Constraint::Length(line_width + additional_x),
        Constraint::Length(line_height + additional_y),
    )
}

pub fn render_border(
    top: Option<Line>,
    bottom: Option<Line>,
    style: Style,
    area: Rect,
    buf: &mut Buffer,
) -> Rect {
    let mut block = Block::bordered()
        .border_style(style)
        .border_type(BorderType::Rounded);
    if let Some(line) = top {
        block = block.title_top(line);
    }
    if let Some(line) = bottom {
        block = block.title_bottom(line);
    }
    block.render(area, buf);
    Layout::default()
        .margin(1)
        .constraints([Constraint::Min(0)])
        .split(area)[0]
}

pub fn render_comfirm_choice<const N: usize>(
    area: Rect,
    buf: &mut Buffer,
    messages: [&str; N],
    choice: usize,
    // (lines, columns)
    split: (u32, u32),
) {
    assert!(split.0 != 0 && split.1 != 0);

    let choice_veritcal_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, split.0); split.0 as usize])
        .split(area);
    let mut choice_areas: Vec<Rect> = Vec::with_capacity((split.0 * split.1) as usize);
    for choice_vertical_area in choice_veritcal_areas.iter() {
        choice_areas.extend(
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, split.1); split.1 as usize])
                .split(*choice_vertical_area)
                .iter(),
        );
    }

    let highlight_style = Style::default().fg(Color::Black).bg(Color::Cyan);
    for (idx, &message) in messages.iter().enumerate() {
        let paragraph = Paragraph::new(message)
            .centered()
            .style(Style::default())
            .wrap(Wrap { trim: false })
            .block(Block::bordered().border_type(BorderType::Plain))
            .style(if idx == choice {
                highlight_style
            } else {
                Style::default()
            });
        paragraph.render(centered_text(message, choice_areas[idx], 2, 2), buf)
    }
}
