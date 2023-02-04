pub mod inits;
pub mod synced_state;
mod tweak_data;

use inits::{StateInit, StateManage};
pub use synced_state::Synced;
use tauri::{
    plugin::{self, TauriPlugin},
    RunEvent, State, Wry,
};

pub type SyncState<'a> = State<'a, Synced>;

pub struct PluginBuilder {
    states_manage: Vec<Box<dyn StateManage + Sync + Send>>,
}

impl PluginBuilder {
    pub fn new() -> Self {
        Self {
            states_manage: Vec::new(),
        }
    }

    pub fn manage(mut self) -> Self {
        let state = StateInit::new();
        self.states_manage.push(Box::new(state));
        self
    }

    pub fn build(self) -> TauriPlugin<Wry> {
        plugin::Builder::new("synced_state")
            .setup(move |handle| {
                self.states_manage.iter().for_each(|state| {
                    state.manage(handle);
                });

                Ok(())
            })
            .on_event(move |_, event| match event {
                RunEvent::Exit => (),
                _ => return,
            })
            .build()
    }
}
