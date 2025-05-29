use color_eyre::{eyre::{Ok, Result}};
use ratatui::{
    crossterm::{event::{
        self, Event, KeyEvent
    }, style::Color}, layout::{Constraint, Layout}, style::{Style, Stylize}, widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Widget}, DefaultTerminal, Frame
};


//List of all TODO items
#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItems>,
    list_state: ListState,
    is_add_new: bool,
}


//Represents a single item on the TODO list
#[derive(Debug, Default)]
struct TodoItems {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;

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
            if handle_key(key, app_state){
                break;
            }
        }
    }
    Ok(())
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }

        event::KeyCode::Char(char) => {
            match char {
                'j' => {
                    app_state.list_state.select_next();
                }
                'k' => {
                    app_state.list_state.select_previous();
                }
                'D' => {
                    if let Some(selected_item) = app_state.list_state.selected(){
                        app_state.items.remove(selected_item);
                    }
                }
                'A' => {
                    app_state.is_add_new = true; 
                }
                _ => {},
            }
        }

        _ => {},
    }
    false

}


fn render(frame: &mut Frame, app_state: &mut AppState){
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

    let list = List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x.description.clone()))
        )
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Green.into()));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
