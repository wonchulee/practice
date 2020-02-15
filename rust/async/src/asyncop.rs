use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub struct Task1 {
    pub count: i32,
}

impl Future for Task1 {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if &self.count > &100 {
            Poll::Ready(())
        } else {
            println!("Task 1 Progress - {}", &self.count);

            self.get_mut().count += 1;
            cx.waker().clone().wake();

            Poll::Pending
        }
    }
}

#[derive(Debug)]
pub struct Task2 {
    pub count: i32,
}

impl Future for Task2 {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if &self.count > &100 {
            Poll::Ready(())
        } else {
            println!("Task 2 Progress - {}", &self.count);

            self.get_mut().count += 1;
            cx.waker().clone().wake();

            Poll::Pending
        }
    }
}
