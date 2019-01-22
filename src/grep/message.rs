use std::sync::{Arc, Mutex};

use super::job::Job;

pub enum Message {
  Task(Arc<Mutex<Job>>),
  Terminate,
}
