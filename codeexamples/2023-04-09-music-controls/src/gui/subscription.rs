use super::message;
use super::state;

pub fn sink_callback(app: &state::State) -> iced::Subscription<message::Message> {
    iced::subscription::unfold(
        "sink message callback",
        app.sink.sink_callback_recv.take(),
        move |mut callback| async move {
            let msg = callback.as_mut().unwrap().recv().unwrap();
            (Some(message::Message::SinkCallback(msg)), callback)
        },
    )
}
