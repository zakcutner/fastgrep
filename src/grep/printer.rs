use std::sync::mpsc;
use std::thread;

use super::message::Message;

pub struct Printer {
  thread: thread::JoinHandle<()>,
}

impl Printer {
  pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
    let thread = thread::spawn(move || loop {
      let task = receiver.recv().unwrap();

      match task {
        Message::Task(job) => {
          let mut job = job.lock().unwrap();
          job.execute();

          for line in job.result() {
            println!("{}", line);
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
