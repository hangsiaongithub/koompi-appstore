use crate::structures::{application::App, store::Store};

impl Store {
    pub fn new(apps: Vec<App>) -> Self {
        Self { apps }
    }
}
