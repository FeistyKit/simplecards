use crate::storage::AnyResult;

use std::time;
use tui::*;
use crossterm::event;

// prepare this
fn prepare_terminal() -> AnyResult<tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>> {
    // https://docs.rs/tui/0.16.0/tui/index.html
    let stdout = std::io::stdout();
    let backend = tui::backend::CrosstermBackend::new(stdout);
    Ok(tui::Terminal::new(backend)?)
}

// I don't like to write this out
pub type TermFrame<'a> = tui::terminal::Frame<'a, tui::backend::CrosstermBackend<std::io::Stdout>>;

// the entry point into the terminal bit of the application
pub fn run_tui() -> AnyResult<()> {
    let mut term = prepare_terminal()?;
    let mut quit = false;
    while !quit {
        term.draw( |f| termdraw(f, &mut quit))?;
    }
    Ok(())
}



// I'm doing this to make the compiler a little less irritating at me:w
//

// called every time the terminal is drawn
fn termdraw<'a>(f: &mut TermFrame, quit: &mut bool, ) {
    while let Some()
}

// Get the next available event if one exists
fn get_event() -> AnyResult<Option<event::Event>> {
    if event::poll(time::Duration::from_secs(0))? {
        return Ok(Some(event::read()?));
    }
    Ok(None)
}

struct TabState<'a> {
    index: usize,
    len: usize,
    tab_names: widgets::Tabs<'a>,
    items: Vec<Box<dyn widgets::Widget>>,
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
        if self.index <= 0 {
            self.index = self.len;
        } else {
            self.index -= 1;
        }
    }
    // get the details of the current tab if one exists, else return none
    fn current_tab_details(&self) -> Option<Box<dyn widgets::Widget>> {
        self.items.get(self.index).cloned()
    }

    // whew that's a long declaration.
    fn new<T>(tab_names: Vec<T>, items: Vec<Box<dyn widgets::Widget>>, constraints: Vec<layout::Constraint>) -> Self
        where T: Into<text::Spans<'a>>
    {
        assert!(tab_names.len() == items.len(), "The number of tab names is not equal to the number of items! ({} != {})", tab_names.len(), items.len());
        let tab_names = tab_names.into_iter().map(text::Spans::from).collect();
        TabState {
            index: 0,
            len: items.len(),
            tab_names,
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
        let titles = ["Tab1", "Tab2", "Tab3", "Tab4"].iter().cloned().map(text::Spans::from).collect();
        let text = vec![text::Spans::from(vec![
            text::Span::raw("Test"),
            text::Span::styled("tab!", style::Style::default().fg(style::Color::Yellow))
        ])];
        let items = vec![Box::new(widgets::Paragraph::new(text.clone())); 2];
        Self::new(titles, text, constraints)
    }

    // draw the application
    fn draw(&self, f: &mut TermFrame) {
        // if there is no item, there are no tabs to draw
        if let Some(item) = self.current_tab_details() {
            let chunks = layout::Layout::default()
                .margin(1)
                .constraints(
                    self.constraints
                )
            .split(f.size());
            let tabs = self.tab_names.cloned();

            // drawing the stuff
            f.render_widget(tabs, chunks[0]);
            f.render_widget(item, chunks[1]);
        }
    }
}
