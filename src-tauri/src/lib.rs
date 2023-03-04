pub mod api;
pub mod inits;
pub mod synced_state;
mod tweak_data;

use inits::StateInit;
use synced_state::Event;
pub use synced_state::Synced;
use tauri::{
    plugin::{self, TauriPlugin},
    Manager, RunEvent, State, Wry,
};

pub type SyncState<'a> = State<'a, Synced>;

#[derive(Debug)]
pub struct PluginBuilder {
    state: Option<StateInit>,
}

impl PluginBuilder {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn manage(mut self) -> Self {
        let state = StateInit::new();
        self.state = Some(state);
        self
    }

    pub fn build(mut self) -> TauriPlugin<Wry> {
        plugin::Builder::new("synced_state")
            .setup(move |handle| {
                if let Some(state) = self.state.take() {
                    state.manage(handle);
                };

                Ok(())
            })
            .on_event(move |handle, event| {
                match event {
                    RunEvent::Exit => (),
                    _ => return,
                }

                let state = handle.state::<Synced>();

                state.send_event(Event::Close);

                while state.player.try_lock().is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }

                state.abort_handles();
            })
            .build()
    }
}
