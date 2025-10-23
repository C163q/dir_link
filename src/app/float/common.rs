use crate::app::message::AppMessage;

pub trait FloatState {
    type Message: AppMessage;
}
