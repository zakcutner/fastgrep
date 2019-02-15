mod job;
mod message;
mod printer;
mod worker;

use std::io;
use std::sync::{mpsc, Arc, Mutex};

use self::job::Job;
use self::message::Message;
use self::printer::Printer;
use self::worker::Worker;

pub struct Grep<'a> {
  lines: Box<dyn Iterator<Item = io::Result<String>> + 'a>,
  needle: Arc<String>,
}

impl<'a> Grep<'a> {
  pub fn new<R>(reader: R, needle: String) -> Self
  where
    R: io::BufRead + 'a,
  {
    Self {
      lines: Box::new(reader.lines()),
      needle: Arc::new(needle),
    }
  }

  pub fn execute<W>(mut self, writer: W, threads: usize, size: usize)
  where
    W: io::Write + Send + 'static,
  {
    assert!(threads > 0);

    let (print_sender, receiver) = mpsc::channel();
    let printer = Printer::new(writer, receiver);

    let (work_sender, receiver) = mpsc::sync_channel(threads);
    let receiver = Arc::new(Mutex::new(receiver));

    let senders = (&print_sender, &work_sender);
    let mut workers = Vec::with_capacity(threads);

    for _ in 0..threads {
      workers.push(Worker::new(receiver.clone()));
    }

    let mut chunk = Vec::with_capacity(size);

    while let Some(line) = self.lines.next() {
      chunk.push(line);

      if chunk.len() == size {
        self.send_chunk(senders, chunk);
        chunk = Vec::with_capacity(size);
      }
    }

    if !chunk.is_empty() {
      self.send_chunk(senders, chunk);
    }

    print_sender.send(Message::Terminate).unwrap();

    for _ in 0..threads {
      work_sender.send(Message::Terminate).unwrap();
    }

    printer.join();

    for worker in workers {
      worker.join();
    }
  }

  fn send_chunk(
    &self,
    (print_sender, work_sender): (&mpsc::Sender<Message>, &mpsc::SyncSender<Message>),
    chunk: Vec<io::Result<String>>,
  ) {
    let job = Job::new(chunk, self.needle.clone());
    let job = Arc::new(Mutex::new(job));

    let message = Message::Task(job.clone());
    print_sender.send(message).unwrap();

    let message = Message::Task(job);
    work_sender.send(message).unwrap();
  }
}
