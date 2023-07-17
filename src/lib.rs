use std::sync::mpsc::{
    sync_channel,
    SyncSender,
};

pub enum Cont {
    TASK(Task),
    DONE,
}

pub struct Executor {
    tx: SyncSender<Task>,
}

impl Executor {
    pub fn init() -> Self {
        let (tx, rx) = sync_channel::<Task>(10_000);
        let tx_c = tx.clone();

        rayon::spawn(move || {
            rayon::scope(|s| {
                for task in rx {
                    s.spawn(|_| {
                        if let Cont::TASK(res_task) = task.invoke() {
                            tx.send(res_task).unwrap();
                        }
                    });
                }
            });
        });

        Self {
            tx: tx_c
        }
    }

    pub fn spawn<F>(
        &self,
        f: F,
    ) where
        F: FnOnce() -> Cont + Send + 'static,
    {
        self.tx.send(Task::new(f)).unwrap();
    }
}

pub struct Task {
    f: Box<dyn FnOnce() -> Cont + Send + 'static>,
}

impl Task {
    fn new<F>(f: F) -> Self
    where
        F: FnOnce() -> Cont + Send + 'static,
    {
        let f = Box::new(f);
        Self {
            f,
        }
    }

    fn invoke(self) -> Cont {
        (self.f)()
    }
}

pub fn continue_with<F>(f: F) -> Cont
where
    F: FnOnce() -> Cont + Send + 'static,
{
    Cont::TASK(Task::new(f))
}
