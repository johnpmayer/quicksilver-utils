use futures_util::{
    future::{poll_fn, LocalFutureObj},
    stream::{FuturesUnordered, Stream},
    task::LocalSpawnExt,
};
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Poll, Waker};

pub struct TaskContext<'a, E> {
    events: Arc<RefCell<Vec<E>>>,
    futures: Arc<RefCell<FuturesUnordered<LocalFutureObj<'a, ()>>>>,
    task_waker: Arc<RefCell<Option<Waker>>>,
}

impl<'a, E> Clone for TaskContext<'a, E> {
    fn clone(&self) -> Self {
        TaskContext {
            events: self.events.clone(),
            futures: self.futures.clone(),
            task_waker: self.task_waker.clone(),
        }
    }
}

impl<'a, E> Default for TaskContext<'a, E> {
    fn default() -> Self {
        TaskContext {
            events: Arc::new(RefCell::new(Vec::new())),
            futures: Arc::new(RefCell::new(FuturesUnordered::new())),
            task_waker: Arc::new(RefCell::new(None)),
        }
    }
}

impl<'a, E> TaskContext<'a, E> {
    pub fn new() -> Self {
        TaskContext::default()
    }

    pub async fn run_until_stalled(&mut self) {
        poll_fn(move |cx| {
            let mut x = self.futures.borrow_mut();
            loop {
                let pinned_pool = Pin::new(&mut *x);
                let pool_state = pinned_pool.poll_next(cx);
                // trace!("Task context run pool_state: {:?}", pool_state);
                match pool_state {
                    Poll::Pending => break Poll::Ready(()),
                    Poll::Ready(Some(_)) => {
                        // debug!("Task finished");
                        continue;
                    }
                    Poll::Ready(None) => {
                        self.task_waker.replace(Some(cx.waker().clone()));
                        break Poll::Ready(());
                    }
                }
            }
        })
        .await
    }

    pub fn spawn<Fut>(&mut self, task: Fut)
    where
        Fut: 'static + Future<Output = ()>,
    {
        // debug!("Spawning new task");
        self.futures.borrow().spawn_local(task).expect("");
        if let Some(waker) = self.task_waker.replace(None) {
            waker.wake();
        }
    }

    pub fn dispatch(&self, event: E) {
        self.events.borrow_mut().push(event)
    }

    pub fn drain(&self) -> Vec<E> {
        self.events.replace(Vec::new())
    }
}
