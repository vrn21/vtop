use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Table},
};
use std::io;
use std::time::Duration;
use sysinfo::System;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = init_terminal()?;
    let result = App::new().run(&mut terminal);
    restore_terminal(&mut terminal)?;
    result
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    terminal.show_cursor()?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    system: System,
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self {
            running: true,
            system,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
            self.system.refresh_all();
            std::thread::sleep(Duration::from_secs(1));
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(frame.area());

        let cpu_usage = format!("CPU Usage: {:.2}%", self.system.global_cpu_usage());
        let memory_usage = format!(
            "Memory Usage: {:.2} / {:.2} MB",
            self.system.used_memory() as f64 / 1024.0,
            self.system.total_memory() as f64 / 1024.0
        );

        let header = Paragraph::new(vec![
            Line::from(Span::styled(cpu_usage, Style::default().fg(Color::Green))),
            Line::from(Span::styled(
                memory_usage,
                Style::default().fg(Color::Green),
            )),
        ])
        .block(Block::default().borders(Borders::ALL).title("System Info"));

        let processes: Vec<_> = self
            .system
            .processes()
            .values()
            .map(|process| {
                vec![
                    process.pid().to_string(),
                    process.name().to_string_lossy().into_owned(),
                    format!("{:.2}%", process.cpu_usage()),
                    format!("{:.2} MB", process.memory() as f64 / 1024.0),
                ]
            })
            .collect();

        let rows = processes
            .iter()
            .map(|item| ratatui::widgets::Row::new(item.iter().map(|c| c.as_str())));

        let table = Table::new(
            rows,
            &[
                Constraint::Length(10),
                Constraint::Length(30),
                Constraint::Length(10),
                Constraint::Length(10),
            ],
        )
        .header(ratatui::widgets::Row::new(vec![
            "PID", "Name", "CPU", "Memory",
        ]))
        .block(Block::default().borders(Borders::ALL).title("Processes"));

        frame.render_widget(header, chunks[0]);
        frame.render_widget(table, chunks[1]);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

// use color_eyre::Result;
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
// use ratatui::{
//     DefaultTerminal, Frame,
//     style::Stylize,
//     text::Line,
//     widgets::{Block, Paragraph},
// };

// fn main() -> color_eyre::Result<()> {
//     color_eyre::install()?;
//     let terminal = ratatui::init();
//     let result = App::new().run(terminal);
//     ratatui::restore();
//     result
// }

// /// The main application which holds the state and logic of the application.
// #[derive(Debug, Default)]
// pub struct App {
//     /// Is the application running?
//     running: bool,
// }

// impl App {
//     /// Construct a new instance of [`App`].
//     pub fn new() -> Self {
//         Self::default()
//     }

//     /// Run the application's main loop.
//     pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
//         self.running = true;
//         while self.running {
//             terminal.draw(|frame| self.render(frame))?;
//             self.handle_crossterm_events()?;
//         }
//         Ok(())
//     }

//     /// Renders the user interface.
//     ///
//     /// This is where you add new widgets. See the following resources for more information:
//     ///
//     /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
//     /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
//     fn render(&mut self, frame: &mut Frame) {
//         let title = Line::from("Ratatui Simple Template")
//             .bold()
//             .blue()
//             .centered();
//         let text = "Hello, Ratatui!\n\n\
//             Created using https://github.com/ratatui/templates\n\
//             Press `Esc`, `Ctrl-C` or `q` to stop running.";
//         frame.render_widget(
//             Paragraph::new(text)
//                 .block(Block::bordered().title(title))
//                 .centered(),
//             frame.area(),
//         )
//     }

//     /// Reads the crossterm events and updates the state of [`App`].
//     ///
//     /// If your application needs to perform work in between handling events, you can use the
//     /// [`event::poll`] function to check if there are any events available with a timeout.
//     fn handle_crossterm_events(&mut self) -> Result<()> {
//         match event::read()? {
//             // it's important to check KeyEventKind::Press to avoid handling key release events
//             Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
//             Event::Mouse(_) => {}
//             Event::Resize(_, _) => {}
//             _ => {}
//         }
//         Ok(())
//     }

//     /// Handles the key events and updates the state of [`App`].
//     fn on_key_event(&mut self, key: KeyEvent) {
//         match (key.modifiers, key.code) {
//             (_, KeyCode::Esc | KeyCode::Char('q'))
//             | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
//             // Add other key handlers here.
//             _ => {}
//         }
//     }

//     /// Set running to false to quit the application.
//     fn quit(&mut self) {
//         self.running = false;
//     }
// }
