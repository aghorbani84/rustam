use dioxus::prelude::*;
use std::time::Duration;

pub fn use_interval<F>(mut callback: F, duration: Duration)
where
    F: FnMut() + 'static,
{
    use_hook(|| {
        spawn(async move {
            let mut interval = gloo_timers::future::IntervalStream::new(duration.as_millis() as u32);
            use futures::StreamExt;
            while interval.next().await.is_some() {
                callback();
            }
        });
    });
}
