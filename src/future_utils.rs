use std::{sync::{Mutex, Arc}, future::Future, pin::Pin, task::{Context, Poll}};

pub struct CallbackFuture<T> {
    loader: Option<Box<dyn FnOnce(Box<dyn FnOnce(T) + Send + 'static>) + Send + 'static>>,
    result: Arc<Mutex<Option<T>>>,
}

impl<T> CallbackFuture<T> {
    pub fn new(loader: impl FnOnce(Box<dyn FnOnce(T) + Send + 'static>) + Send + 'static)
        -> CallbackFuture<T> {
        CallbackFuture {
            loader: Some(Box::new(loader)),
            result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn ready(value: T) -> CallbackFuture<T> {
        CallbackFuture {
            loader: None,
            result: Arc::new(Mutex::new(Some(value))),
        }
    }
}

impl<T: Send + 'static> Future for CallbackFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let self_mut = self.get_mut();
        match self_mut.loader.take() {
            // in case loader is still present, loader was not yet invoked: invoke it
            Some(loader) => {
                let waker = cx.waker().clone();
                let result = self_mut.result.clone();
                loader(Box::new(move |value| {
                    *result.lock().unwrap() = Some(value);
                    waker.wake();
                }));
                Poll::Pending
            }
            // in case loader was moved-out: either result is already ready,
            // or we haven't yet received callback
            None => {
                match self_mut.result.lock().unwrap().take() {
                    Some(value) => Poll::Ready(value),
                    None => Poll::Pending, // we haven't received callback yet
                }
            }
        }
    }
}