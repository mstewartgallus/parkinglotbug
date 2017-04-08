use parking_lot::{Condvar, Mutex};

struct ParkEventState {
    triggered: bool,
}

/// Uses platform primitives to create an event object
pub struct ParkEvent {
    when_triggered: Condvar,
    state: Mutex<ParkEventState>,
}
unsafe impl Send for ParkEvent {}
unsafe impl Sync for ParkEvent {}

impl ParkEvent {
    pub fn new() -> Self {
        ParkEvent {
            when_triggered: Condvar::new(),
            state: Mutex::new(ParkEventState { triggered: false }),
        }
    }

    pub fn wait(&self) {
        let mut state = self.state.lock();

        loop {
            if state.triggered {
                state.triggered = false;
                break;
            }
            self.when_triggered.wait(&mut state);
        }

        state.unlock_fair();
    }

    pub fn signal(&self) {
        let mut state = self.state.lock();

        state.triggered = true;
        self.when_triggered.notify_all();

        state.unlock_fair();
    }
}
