use std::future::Future;
use std::pin::Pin;
use std::time;
use std::task;

use pin_project_lite::pin_project;

pub struct PollMax<F>(F);

impl<F: Future> Future for PollMax<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let now = time::Instant::now();
        let fut = unsafe { 
            self.map_unchecked_mut(|f| {
                &mut f.0
            }) 
        };
        
        let res = fut.poll(cx);

        let elapsed = now.elapsed();
        if elapsed > time::Duration::from_millis(10) {
            // use your logging or tracing framework here
            println!("future polled for {:?}", elapsed);
        }
        res
    }
}

pub fn poll_max<F: Future>(f: F) -> PollMax<F> {
    PollMax(f)
}

pin_project! {
    pub struct Deadline<F> {
        #[pin]
        fut: F,
        deadline: time::Duration,
        start: Option<time::Instant>,
    }
}

impl<F: Future> Future for Deadline<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = self.project();
        
        if *this.start == None {
            *this.start = Some(time::Instant::now());
        }


        let res = this.fut.poll(cx);

        match &res {
            task::Poll::Ready(_) => {
                let elapsed = this.start.as_ref().unwrap().elapsed();
                if elapsed > *this.deadline {
                    // use your logging or tracing framework here
                    eprintln!("future missed deadline {:?}", elapsed);
                }
            },
            _ => {}
        }
        
        res
    }
}

pub fn deadline<F: Future>(deadline: time::Duration, f: F) -> Deadline<F> {
    Deadline {
        fut: f,
        deadline: deadline,
        start: None,  
    }
}