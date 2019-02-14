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
  jobs: Vec<Arc<Mutex<Job>>>,
}

impl<'a> Grep<'a> {
  pub fn new(reader: impl io::BufRead + 'a, needle: String) -> Self {
    Grep {
      lines: Box::new(reader.lines()),
      needle: Arc::new(needle),
      jobs: Vec::new(),
    }
  }

  pub fn execute(mut self, threads: usize, size: usize) {
    assert!(threads > 0);
    let mut workers = Vec::with_capacity(threads);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for _ in 0..threads {
      workers.push(Worker::new(receiver.clone()));
    }

    let mut chunk = Vec::with_capacity(size);

    while let Some(line) = self.lines.next() {
      chunk.push(line);

      if chunk.len() == size {
        self.send_chunk(&sender, chunk);
        chunk = Vec::with_capacity(size);
      }
    }

    if !chunk.is_empty() {
      self.send_chunk(&sender, chunk);
    }

    for job in self.jobs.into_iter() {
      let mut job = job.lock().unwrap();
      job.execute();

      for line in job.result() {
        println!("{}", line);
      }
    }

    for _ in 0..threads {
      sender.send(Message::Terminate).unwrap();
    }

    for worker in workers {
      worker.join();
    }
  }

  fn send_chunk(&mut self, sender: &mpsc::Sender<Message>, chunk: Vec<io::Result<String>>) {
    let job = Job::new(chunk, self.needle.clone());
    let job = Arc::new(Mutex::new(job));
    self.jobs.push(job.clone());

    let message = Message::Task(job);
    sender.send(message).unwrap();
  }
}
