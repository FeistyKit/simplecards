mod storage;
mod vocab;

use crate::storage::AnyResult;

use std::time;
use tui::*;
use crossterm::event;

fn prepare_terminal() -> AnyResult<tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>> {
    // https://docs.rs/tui/0.16.0/tui/index.html
    let stdout = std::io::stdout();
    let backend = tui::backend::CrosstermBackend::new(stdout);
    Ok(tui::Terminal::new(backend)?)
}

pub type TermFrame<'a> = tui::terminal::Frame<'a, tui::backend::CrosstermBackend<std::io::Stdout>>;

fn main() -> AnyResult<()> {
    let mut term = prepare_terminal()?;
    loop {
        term.draw(termdraw)?;
    }
    Ok(())
}

fn termdraw<'a>(f: &mut TermFrame) {
    let chunks = layout::Layout::default()
        .margin(1)
        .constraints(
            [
                layout::Constraint::Length(3),
                layout::Constraint::Min(0),
            ].as_ref()
        )
        .split(f.size());
    let titles = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(text::Spans::from).collect();
    let tabs = widgets::Tabs::new(titles)
        .block(widgets::Block::default().title("Tabs").borders(widgets::Borders::ALL))
        .style(style::Style::default().fg(style::Color::White))
        .highlight_style(style::Style::default().fg(style::Color::Yellow))
        .divider(symbols::DOT);
    let block = widgets::Block::default()
        .title("Block")
        .borders(widgets::Borders::ALL);
    f.render_widget(tabs, chunks[0]);
    f.render_widget(block, f.size());

}

// Get the polled events
fn get_event() -> AnyResult<Option<event::Event>> {
    if event::poll(time::Duration::from_secs(0))? {
        return Ok(Some(event::read()?));
    }
    Ok(None)
}

struct TabState<'a> {
    index: usize,
    tab_names: widgets::Tabs<'a>,
    items: Vec<Box<dyn widgets::Widget>>,
}
