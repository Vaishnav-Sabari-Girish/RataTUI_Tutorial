use color_eyre::eyre::{Ok, Result};
use ratatui::{
    crossterm::event::{
        self, Event
    }, widgets::{Paragraph, Widget}, DefaultTerminal, Frame
};


//List of all TODO items
#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItems>,
}


//Represents a single item on the TODO list
##[derive(Debug, Default)]
struct TodoItems {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal, &mut state);

    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {

    loop {
        
        //Rendering 
        terminal.draw(|f| render(f, app_state));
        
        

        //Input Handling 
        //Keybinding 

        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }

                _ => todo!(),
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState){
    Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());
}
