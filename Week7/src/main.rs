use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

struct MyGUI {
    button_state: button::State,
}

#[derive(Debug, Clone)]
enum MyMessage {
    ButtonClicked,
}

impl Sandbox for MyGUI {
    type Message = MyMessage;

    fn new() -> MyGUI {
        MyGUI {
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("My Rust GUI")
    }

    fn update(&mut self, message: MyMessage) {
        match message {
            MyMessage::ButtonClicked => {
                println!("Button clicked!");
            }
        }
    }

    fn view(&mut self) -> Element<MyMessage> {
        Column::new()
            .align_items(Align::Center)
            .spacing(20)
            .push(Text::new("Hello, Rust GUI!"))
            .push(
                Button::new(&mut self.button_state, Text::new("Click me!"))
                    .on_press(MyMessage::ButtonClicked),
            )
            .into()
    }
}

fn main() {
    MyGUI::run(Settings::default());
}
