use std::sync::mpsc::{channel, Receiver, SendError, Sender};

pub struct UnboundedBroadcast<T> {
    channel: Option<Sender<T>>
}

impl<T: 'static + Clone + Send + Sync> UnboundedBroadcast<T> {
    pub fn new() -> Self {
        Self {channel: None}
    }

    pub fn subscribe(&mut self) -> Receiver<T> {
        let (tx, rx) = channel();
        self.channel = Some(tx);
        rx
    }

    pub fn send(&self, message: T) -> Result<(), SendError<T>> {
        if let Some(c) = self.channel.clone(){
            c.send(message.clone())?;
        }
        Ok(())
    }
}