use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use super::message::Message;

pub struct Worker {
  thread: thread::JoinHandle<()>,
}

impl Worker {
  pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
    let thread = thread::spawn(move || loop {
      let receiver = receiver.lock().unwrap();
      let task = receiver.recv().unwrap();

      match task {
        Message::Task(job) => {
          if let Ok(mut job) = job.try_lock() {
            job.execute();
          }
        }
        Message::Terminate => break,
      }
    });

    Self { thread }
  }

  pub fn join(self) {
    self.thread.join().unwrap();
  }
}
