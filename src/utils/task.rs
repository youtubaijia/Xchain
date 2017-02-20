use std::thread;
use std::clone::Clone;
use std::sync::mpsc::{Sender, Receiver, channel};

#[allow(dead_code)]
pub struct Task {
    handler: thread::JoinHandle<()>,
    name: &'static str,
}

#[allow(dead_code)]
pub struct TaskWithInputs<A> {
    task: Task,
    channel_input: TaskMessageBox<A>,
}

pub fn task_create<F>(name: &'static str, f: F) -> Task
  where F: FnOnce() -> (),
        F: Send + 'static,
{
    let handler = thread::spawn(move || { f() });
    Task {
        handler: handler,
        name: name,
    }
}

pub fn task_create_with_inputs<F, A>(name: &'static str, f: F) -> TaskWithInputs<A>
  where F: FnOnce(Receiver<A>) -> (),
        F: Send + 'static,
        A: Send + 'static,
{
    let (tx, rx) = channel();

    let handler = thread::spawn(move || {
        f(rx)
    });
    let task = Task {
        handler: handler,
        name: name,
    };
    TaskWithInputs {
        task: task,
        channel_input: TaskMessageBox(tx),
    }
}

#[derive(Clone)]
pub struct TaskMessageBox<A>(Sender<A>);

impl<A> TaskMessageBox<A> {
    pub fn send_to(self, a: A) {
        self.0.send(a).unwrap()
    }
}

impl<A> TaskWithInputs<A> {
    pub fn get_message_box(&self) -> TaskMessageBox<A> {
        TaskMessageBox(self.channel_input.0.clone())
    }
}
