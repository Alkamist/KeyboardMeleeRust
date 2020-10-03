use std::time::{Instant, Duration};

use iced::time;
use iced::{
    button, executor, Button, Align, Column,
    Element, Application, Settings, Text,
    Command, Subscription
};

use keyboard_melee_controller::KeyboardMeleeController;

//#[tokio::main]
//async fn main() {
//    let mut controller = KeyboardMeleeController::new();
//
//    let mut interval = tokio::time::interval(Duration::from_millis(1));
//    loop {
//        controller.poll();
//        interval.tick().await;
//    }
//}

pub fn main() {
    App::run(Settings {
        window: iced::window::Settings {
            size: (400, 200),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    }).unwrap();
}

struct App {
    controller: KeyboardMeleeController,
    test_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    UpdateController(Instant),
    TestButtonPressed,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                controller: KeyboardMeleeController::new(),
                test_button: Default::default()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Window Test".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::UpdateController(_) => self.controller.update(),
            Message::TestButtonPressed => println!("Button Pressed!"),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.test_button, Text::new("Test"))
                    .on_press(Message::TestButtonPressed),
            )
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(300))
            .map(Message::UpdateController)
    }
}
