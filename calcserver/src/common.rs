use std::sync::Mutex;

pub struct AppState {
    pub calc: Mutex<calclib::Calc>, // <- Mutex is necessary to mutate safely across threads
}
