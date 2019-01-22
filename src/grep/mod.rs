mod job;
mod message;
mod worker;

use std::io;
use std::sync::{mpsc, Arc, Mutex};

use self::job::Job;
use self::message::Message;
use self::worker::Worker;

pub struct Grep<'a> {
  lines: Box<Iterator<Item = io::Result<String>> + 'a>,
  needle: Arc<String>,
}

impl<'a> Grep<'a> {
  pub fn new(reader: impl io::BufRead + 'a, needle: String) -> Self {
    Grep {
      lines: Box::new(reader.lines()),
      needle: Arc::new(needle),
    }
  }

  pub fn execute(self, threads: usize) {
    assert!(threads > 0);
    let mut workers = Vec::with_capacity(threads);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for _ in 0..threads {
      workers.push(Worker::new(receiver.clone()));
    }

    let mut jobs = Vec::new();

    for line in self.lines {
      let job = Job::new(line, self.needle.clone());
      let job = Arc::new(Mutex::new(job));
      jobs.push(job.clone());

      let message = Message::Task(job);
      sender.send(message).unwrap();
    }

    for job in jobs.into_iter() {
      let mut job = job.lock().unwrap();
      job.execute();

      if let Some(result) = job.result() {
        println!("{}", result);
      }
    }

    for _ in 0..threads {
      sender.send(Message::Terminate).unwrap();
    }

    for worker in workers {
      worker.join();
    }
  }
}
