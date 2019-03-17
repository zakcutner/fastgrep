pub mod pattern;

mod job;
mod message;
mod printer;
mod worker;

use std::io;
use std::sync::{mpsc, Arc, Mutex};

use self::job::Job;
use self::message::Message;
use self::pattern::Pattern;
use self::printer::Printer;
use self::worker::Worker;

type ArcPattern = Arc<dyn Pattern + Send + Sync>;
type Senders<'a> = (&'a mpsc::Sender<Message>, &'a mpsc::SyncSender<Message>);

pub struct Grep<'a> {
  lines: Box<dyn Iterator<Item = io::Result<String>> + 'a>,
  pattern: Option<ArcPattern>,
  threads: usize,
  size: usize,
  colour: bool,
}

impl<'a> Grep<'a> {
  pub fn new<R>(reader: R) -> Self
  where
    R: io::BufRead + 'a,
  {
    Self {
      lines: Box::new(reader.lines()),
      pattern: None,
      threads: 1,
      size: 1,
      colour: true,
    }
  }

  pub fn set_pattern<P>(&mut self, pattern: P)
  where
    P: Pattern + Send + Sync + 'static,
  {
    self.pattern = Some(Arc::new(pattern));
  }

  pub fn set_threads(&mut self, threads: usize) {
    assert!(threads > 0);
    self.threads = threads;
  }

  pub fn set_size(&mut self, size: usize) {
    assert!(size > 0);
    self.size = size;
  }

  pub fn set_colour(&mut self, colour: bool) {
    self.colour = colour;
  }

  pub fn execute<W>(mut self, writer: W)
  where
    W: io::Write + Send + 'static,
  {
    let (print_sender, receiver) = mpsc::channel();
    let printer = Printer::new(writer, receiver);

    let (work_sender, receiver) = mpsc::sync_channel(self.threads);
    let receiver = Arc::new(Mutex::new(receiver));

    let senders = (&print_sender, &work_sender);
    let mut workers = Vec::with_capacity(self.threads);

    for _ in 0..self.threads {
      workers.push(Worker::new(receiver.clone()));
    }

    let mut chunk = Vec::with_capacity(self.size);

    while let Some(line) = self.lines.next() {
      chunk.push(line);

      if chunk.len() == self.size {
        self.send_chunk(senders, chunk);
        chunk = Vec::with_capacity(self.size);
      }
    }

    if !chunk.is_empty() {
      self.send_chunk(senders, chunk);
    }

    print_sender.send(Message::Terminate).unwrap();

    for _ in 0..self.threads {
      work_sender.send(Message::Terminate).unwrap();
    }

    printer.join();

    for worker in workers {
      worker.join();
    }
  }

  fn send_chunk(&self, (print_sender, work_sender): Senders, chunk: Vec<io::Result<String>>) {
    let pattern = self.pattern.as_ref().unwrap();

    let job = Job::new(chunk, pattern.clone(), self.colour);
    let job = Arc::new(Mutex::new(job));

    let message = Message::Task(job.clone());
    print_sender.send(message).unwrap();

    let message = Message::Task(job);
    work_sender.send(message).unwrap();
  }
}
