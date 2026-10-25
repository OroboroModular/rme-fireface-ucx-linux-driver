//! Blocking work (native file dialogs, folder scans), run off the GUI thread.
//!
//! rfd's blocking dialogs pump a nested message loop; on the editor thread
//! that loop re-enters baseview's window procedure while it's still handling
//! the click that opened the dialog, which panics and takes the host down
//! (Oroboro Bass found this). Folder scans are just slow. So both run on a
//! worker thread and hand their result back through a channel, which the
//! editor polls every frame (egui redraws continuously under nih-plug).

use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::Mutex;

/// At most one task in flight. (The receiver sits in a mutex so the editor's
/// state stays `Sync`, as nih-plug's editors require.)
pub struct PendingTask<T> {
    rx: Option<Mutex<Receiver<T>>>,
}

impl<T: Send + 'static> Default for PendingTask<T> {
    fn default() -> Self {
        Self { rx: None }
    }
}

impl<T: Send + 'static> PendingTask<T> {
    pub fn is_busy(&self) -> bool {
        self.rx.is_some()
    }

    /// Runs `task` on its own thread. A task still running is abandoned: it
    /// finishes, but its result is dropped.
    pub fn start(&mut self, task: impl FnOnce() -> T + Send + 'static) {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            // nobody may be listening any more
            let _ = tx.send(task());
        });
        self.rx = Some(Mutex::new(rx));
    }

    /// The task's result, once it has finished.
    pub fn take(&mut self) -> Option<T> {
        let received = self.rx.as_ref()?.lock().map_err(|_| TryRecvError::Disconnected).and_then(|rx| rx.try_recv());
        let result = match received {
            Ok(value) => Some(value),
            Err(TryRecvError::Empty) => return None,
            // the worker panicked
            Err(TryRecvError::Disconnected) => None,
        };
        self.rx = None;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hands_the_result_back_once() {
        let mut task = PendingTask::default();
        task.start(|| 42);
        let mut got = None;
        for _ in 0..1000 {
            if let Some(v) = task.take() {
                got = Some(v);
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        assert_eq!(got, Some(42));
        assert!(!task.is_busy() && task.take().is_none());
    }
}
