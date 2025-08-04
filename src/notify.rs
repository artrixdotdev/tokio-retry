use std::{future::Future, pin::Pin, time::Duration};

pub trait Notify<E> {
    async fn notify(&mut self, err: &E, duration: Duration);
}

impl<E, F> Notify<E> for F
where
    F: FnMut(&E, Duration),
{
    async fn notify(&mut self, err: &E, duration: Duration) {
        self(err, duration)
    }
}

pub enum NotifyFn<E> {
    Sync(Box<dyn FnMut(&E, Duration) + Send>),
    Async(Box<dyn FnMut(&E, Duration) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>),
}

impl<E> NotifyFn<E> {
    pub fn from_sync<F>(f: F) -> Self
    where
        F: FnMut(&E, Duration) + Send + 'static,
    {
        NotifyFn::Sync(Box::new(f))
    }

    pub fn from_async<F>(f: F) -> Self
    where
        F: FnMut(&E, Duration) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + 'static,
    {
        NotifyFn::Async(Box::new(f))
    }
}

impl<E> Notify<E> for NotifyFn<E> {
    async fn notify(&mut self, err: &E, duration: Duration) {
        match self {
            NotifyFn::Sync(f) => f(err, duration),
            NotifyFn::Async(f) => f(err, duration).await,
        }
    }
}
