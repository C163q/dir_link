use ratatui::crossterm::event::KeyEvent;

use crate::app::{
    App,
    float::{Float, FloatActionResult, FloatState},
    message::FloatUpdater,
};

pub fn handle_common_key<S, F1, F2, F3>(
    app: &mut App,
    key: KeyEvent,
    mut state: S,
    handle_key: F1,
    handle_message: F2,
    float_with_state: F3,
) -> FloatActionResult
where
    S: FloatState,
    F1: FnOnce(KeyEvent) -> Option<S::Message>,
    F2: Fn(&mut App, S, S::Message) -> FloatUpdater<S>,
    F3: FnOnce(S) -> Float,
{
    let mut new_float = None;
    let mut opt_msg = handle_key(key);
    while let Some(msg) = opt_msg {
        let updater = handle_message(app, state, msg);
        opt_msg = updater.message;
        new_float = updater.float;
        match updater.state {
            Some(s) => state = s,
            None => return FloatActionResult::new().with_optional_new(new_float),
        }
    }
    FloatActionResult::new()
        .with_primary(float_with_state(state))
        .with_optional_new(new_float)
}
