// use std::thread::{sleep, spawn};
use async_std::task::{sleep, spawn};
use std::time::Duration;
use std::future::Future;

use std::pin::Pin;
use std::task::{Poll, Context};

struct DoNothing;

struct SleepPrint<Fut> {
    sleep: Fut,
}

impl<Fut: Future<Output=()>> Future for SleepPrint<Fut> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let sleep: Pin<&mut Fut> = unsafe { self.map_unchecked_mut(|s| &mut s.sleep) };
        match sleep.poll(ctx) {
            Poll::Ready(()) => {
                println!("inside sleep print");
                Poll::Ready(())
            },
            Poll::Pending => Poll::Pending
        }

    }
}

impl Future for DoNothing {
    type Output = ();
    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        // unimplemented!()
        Poll::Ready(())
    }

}

async fn sleepus() -> impl std::future::Future<Output=()> {
    // for i in 1..=10 {
    //     println!("sleepus {}", i);
    //     sleep(Duration::from_millis(500));
    // }
    // async_std::future::ready(())
    // DoNothing.await
    SleepPrint {
        sleep: sleep(Duration::from_millis(3000))
    }
}


async fn interruptus() {
    for i in 1..=5 {
        println!("interruptus {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

#[async_std::main]
pub async fn main() {
    let sleepus = spawn(sleepus());
    // let interruptus = spawn(interruptus());
    // interruptus.await;
    interruptus().await;

    sleepus.await;
}
