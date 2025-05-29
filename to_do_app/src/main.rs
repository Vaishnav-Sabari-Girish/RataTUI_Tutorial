use color_eyre::eyre::{Ok, Result};
use ratatui::{
    crossterm::{event::{
        self, Event
    }, style::Color}, layout::{Constraint, Layout}, style::Stylize, widgets::{Block, BorderType, List, ListItem, Paragraph, Widget}, DefaultTerminal, Frame
};


//List of all TODO items
#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItems>,
}


//Represents a single item on the TODO list
#[derive(Debug, Default)]
struct TodoItems {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.items.push(TodoItems{
        is_done: false, 
        description: String::from("Finish the app")
    });

    state.items.push(TodoItems{
        is_done: true, 
        description: String::from("Do some shit ")
    });

    state.items.push(TodoItems{
        is_done: false, 
        description: String::from("Do somemore shit")
    });

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
    let [border_area] = Layout::vertical([Constraint::Fill(1)]).
        margin(1).
        areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)]).
        margin(1).
        areas(border_area);

    Block::bordered().border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    //Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());

    List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x.description.clone()))
        )
        .render(inner_area, frame.buffer_mut());
}
