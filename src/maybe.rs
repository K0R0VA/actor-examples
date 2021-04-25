use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use actix::dev::{MessageResponse, OneshotSender};
use actix::{Actor, Message};

pub enum Maybe<T> {
    Some(T),
    None
}

impl<Fut, T> Future for Maybe<Fut> where Fut: Future<Output = T> {
    type Output = Maybe<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        unsafe {
            let mut this = self.get_unchecked_mut();
            match this {
                Maybe::Some( fut) => {
                    match Pin::new_unchecked(fut).poll(cx) {
                        Poll::Ready(item) => Poll::Ready(Maybe::Some(item)),
                        Poll::Pending => Poll::Pending
                    }
                },
                Maybe::None => Poll::Ready(Maybe::None)
            }
        }
    }
}

impl<A, M, Fut, T> MessageResponse<A, M> for Maybe<Fut>
    where
        A: Actor,
        M: Message<Result = Maybe<T>>,
        Fut: Future<Output = T> + 'static,
        T: 'static
{
    fn handle(self, ctx: &mut <A as Actor>::Context, tx: Option<OneshotSender<M::Result>>) {
        actix::spawn(async {
            if let Some(tx) = tx {
                tx.send(self.await);
            }
        });
    }
}

impl<T> Maybe<T> {
    pub fn unwrap(self) -> T {
        match self {
            Maybe::Some(item) => item,
            Maybe::None => panic!()
        }
    }
}
