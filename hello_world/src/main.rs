use color_eyre::{eyre::Ok, Result};
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn main()-> Result<()> {
    let _ = color_eyre::install();
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)){
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame){
    frame.render_widget("Hello World", frame.area());
}
