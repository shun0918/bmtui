use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::interval;

pub struct EventHandler {
    event_stream: EventStream,
    tick_interval: tokio::time::Interval,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            event_stream: EventStream::new(),
            tick_interval: interval(Duration::from_millis(50)),
        }
    }

    pub async fn next(&mut self) -> Option<Event> {
        tokio::select! {
            _ = self.tick_interval.tick() => {
                None
            }
            Some(Ok(event)) = self.event_stream.next() => {
                Some(event)
            }
            else => None,
        }
    }
}
