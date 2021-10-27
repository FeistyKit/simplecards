mod storage;
mod vocab;

use crate::storage::AnyResult;

use tui::widgets;

fn prepare_terminal() -> AnyResult<tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>> {
    // https://docs.rs/tui/0.16.0/tui/index.html
    let stdout = std::io::stdout();
    let backend = tui::backend::CrosstermBackend::new(stdout);
    Ok(tui::Terminal::new(backend)?)
}

fn main() -> AnyResult<()> {
    let mut term = prepare_terminal()?;
    term.draw(termdraw)?;
    Ok(())
}

fn termdraw<'a>(f: &mut tui::terminal::Frame<'_, tui::backend::CrosstermBackend<std::io::Stdout>>) {
    let size = f.size();
    let block = widgets::Block::default()
        .title("Block")
        .borders(widgets::Borders::ALL);
    f.render_widget(block, size);
}
