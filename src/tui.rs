use crate::storage::AnyResult;

use std::time;
use tui::*;
use crossterm::event;

// prepare this
fn prepare_terminal() -> AnyResult<tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>> {
    // https://docs.rs/tui/0.16.0/tui/index.html
    let stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = tui::backend::CrosstermBackend::new(stdout);
    Ok(tui::Terminal::new(backend)?)
}

// I don't like to write this out
pub type TermFrame<'a> = tui::terminal::Frame<'a, tui::backend::CrosstermBackend<std::io::Stdout>>;

// the entry point into the terminal bit of the application
pub fn run_tui() -> AnyResult<()> {
    let mut term = prepare_terminal()?;
    let mut quit = false;
    let mut app = TabState::testing();
    term.clear()?;
    while !quit {
        term.draw( |f| termdraw(f, &mut quit, &mut app))?;
    }
    // TODO: implement neovim-like saving of the terminal and restoring after program has exited
    term.clear()?;
    Ok(())
}

// called every time the terminal is drawn
fn termdraw<'a>(f: &mut TermFrame, quit: &mut bool, app: &mut TabState<'a>) {
    while let Some(event) = next_event().expect("Could not get next event!") {
        if let event::Event::Key(key_ev) = event {
            match key_ev.code {
                event::KeyCode::Char(last) => { // hopefully the last layer that needs to be dereferenced

                    // quit on a press of 'q'
                    // TODO: customize quit button
                    if last.to_ascii_lowercase() == 'q' {
                        *quit = true;
                    }
                },
                _ => {},
            }
        }
    }

    app.draw(f);
}

// Get the next available event if one exists
fn next_event() -> AnyResult<Option<event::Event>> {
    if event::poll(time::Duration::from_secs(0))? {
        return Ok(Some(event::read()?));
    }
    Ok(None)
}

struct TabState<'a> {
    index: usize,
    len: usize,
    tab_names: widgets::Tabs<'a>,
    items: Vec<WidgetType<'a>>,
    constraints: Vec<layout::Constraint>
}

impl<'a> TabState<'a> {
    // move the selected tab over by one to the right
    fn increment(&mut self) {
        if self.index >= self.len {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
    //move the selected tab over by one to the left
    fn decrement(&mut self) {
        if self.index == 0 {
            self.index = self.len;
        } else {
            self.index -= 1;
        }
    }
    // get the details of the current tab if one exists, else return none
    fn current_tab_details(&self) -> Option<WidgetType> {
        self.items.get(self.index).cloned()
    }

    // whew that's a long declaration.
    // User must be sure that the number of tabs is the same as the number of items
    fn new(tabs: widgets::Tabs<'a>, items: Vec<WidgetType<'a>>, constraints: Vec<layout::Constraint>) -> Self
    {
        TabState {
            index: 0,
            len: items.len(),
            tab_names: tabs,
            items,
            constraints
        }
    }

    // a UI for testing purposes
    fn testing() -> Self {
        let constraints = vec![
                layout::Constraint::Length(3),
                layout::Constraint::Min(0),
            ];
        let tabs = widgets::Tabs::new(["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(text::Spans::from).collect()).block(widgets::Block::default().title("Tabs!").borders(widgets::Borders::ALL));
        let text = vec![text::Spans::from(vec![
            text::Span::raw("Test "),
            text::Span::styled("tab!", style::Style::default().fg(style::Color::Yellow))
        ])];
        let items = vec![WidgetType::Paragraph(widgets::Paragraph::new(text.clone()).block(widgets::Block::default().borders(widgets::Borders::ALL).title("Paragraph title !!"))); 4];
        Self::new(tabs, items, constraints)
    }

    // draw the application
    fn draw(&self, f: &mut TermFrame) {
        // if there is no item, there are no tabs to draw
        if let Some(item) = self.current_tab_details() {
            let chunks = layout::Layout::default()
                .margin(1)
                .constraints(
                    self.constraints.clone()
                )
            .split(f.size());
            let tabs = self.tab_names.clone();

            // drawing the stuff
            f.render_widget(tabs, chunks[0]);
            item.draw(f, chunks[1]);
        }
    }
}

// A way to generalize over different widgets
// I will add more variants as they are needed
#[derive(Clone, Debug)]
enum WidgetType<'a> {
    Block(widgets::Block<'a>),
    Paragraph(widgets::Paragraph<'a>)
}

impl<'a> WidgetType<'a> {
    fn draw(&self, f: &mut TermFrame, area: layout::Rect) {
        match self {
            WidgetType::Block(b) => {
                let other = b.clone();
                f.render_widget(other, area);
            },
            WidgetType::Paragraph(p) => {
                let other = p.clone();
                f.render_widget(other, area);
            }
        }
    }
}
