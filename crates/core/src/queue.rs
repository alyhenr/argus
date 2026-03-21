use tokio::sync::mpsc;
use crate::models::Event;

/// The Producer: Handed to the Axum web handlers. 
/// Cheap to clone. When a web request comes in, 
/// it uses this to push data into the channel without locking the whole system.
#[derive(Clone)]
pub struct EventProducer {
    tx: mpsc::Sender<Event>,
}

impl EventProducer {
    /// Pushes an event into the ring buffer. 
    /// If the buffer is completely full, this will wait (yielding to the async runtime)
    /// until there is space.
    pub async fn push(&self, event: Event) -> Result<(), &'static str> {
        self.tx.send(event).await.map_err(|_| "Channel closed, cannot send event")
    }
}

/// The Consumer: Owned exclusively by the background worker thread.
/// It drains the queue and prepares batches for the database.
pub struct EventConsumer {
    rx: mpsc::Receiver<Event>,
}

impl EventConsumer {
    /// Pulls a single event from the queue. 
    /// If the queue is empty, it goes to sleep (zero CPU usage) until a new event arrives.
    pub async fn recv(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    /// Drain up to `batch_size` events at once.
    /// This prevents from doing 10,000 separate database inserts.
    pub async fn flush_batch(&mut self, max_batch_size: usize) -> Vec<Event> {
        let mut batch = Vec::with_capacity(max_batch_size);
        
        // Wait for at least ONE event to start the batch
        if let Some(event) = self.recv().await {
            batch.push(event);
        } else {
            return batch; // Channel closed
        }

        // Quickly grab any other events currently sitting in the buffer
        // Without going to sleep if the buffer is temporarily empty.
        while batch.len() < max_batch_size {
            if let Ok(event) = self.rx.try_recv() {
                batch.push(event);
            } else {
                break; // Buffer is empty for now, stop collecting
            }
        }

        batch
    }
}

/// Initializes custom in-memory pipeline.
/// `capacity` defines how many events can sit in RAM before the web server 
/// has to slow down and wait for the worker to catch up.
pub fn create_ingestion_queue(capacity: usize) -> (EventProducer, EventConsumer) {
    // Under the hood, creates a highly optimized, lock-free ring buffer
    let (tx, rx) = mpsc::channel(capacity);
    (EventProducer { tx }, EventConsumer { rx })
}