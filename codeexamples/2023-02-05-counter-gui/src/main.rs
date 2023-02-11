use iced::widget::{button, column, text};
use iced::Sandbox;

// This represents the State of our application:
struct CounterState {
    // The counter value
    value: i32,
}

// These are our Message; the possible user interactions of our counter: the button presses.
#[derive(Debug, Clone, Copy)]
pub enum CounterMessage {
    IncrementPressed,
    DecrementPressed,
}

impl iced::Sandbox for CounterState {
    type Message = CounterMessage;

    // We need a `new` for Iced to know what State to start with
    fn new() -> Self {
        CounterState { value: 1 }
    }

    // We also need to provide a "title" for our app
    fn title(&self) -> String {
        "Counter App".to_string()
    }

    // The Update Logic is where we accept messages to update the State
    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::IncrementPressed => {
                self.value += 1;
            }
            CounterMessage::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    // The View Logic displays our State and the other components the user can use to interact with our app
    fn view(&self) -> iced::Element<CounterMessage> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(CounterMessage::IncrementPressed),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(CounterMessage::DecrementPressed),
        ]
        .into()
    }
}

fn main() {
    CounterState::run(iced::Settings::default()).unwrap();
}
