use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};

use crate::REACHED_COUNT_SIGNAL_AMOUNT;

#[derive(Default)]
pub struct BenchActor {
    count: i64,
    /// optional channel to signal when implementation reached REACHED_COUNT_SIGNAL_AMOUNT
    tx: Option<oneshot::Sender<()>>,
}

pub enum Message {
    IncreaseBySync(i64, oneshot::Sender<()>),
    DecreaseBySync(i64, oneshot::Sender<()>),
    IncreaseBy(i64),
    DecreaseBy(i64),
    Get(oneshot::Sender<i64>),
}

impl BenchActor {
    pub fn new(m: oneshot::Sender<()>) -> Self {
        Self {
            tx: Some(m),
            ..Default::default()
        }
    }

    pub async fn start(mut self) -> Sender<Message> {
        let (tx, mut rx) = mpsc::channel(10000);
        tokio::spawn(async move {
            while let Some(m) = rx.recv().await {
                match m {
                    Message::IncreaseBySync(i, r) => {
                        self.count += i;
                        r.send(());
                    }
                    Message::DecreaseBySync(i, r) => {
                        self.count -= i;
                        r.send(());
                    }
                    Message::Get(r) => {
                        r.send(self.count);
                    }
                    Message::IncreaseBy(i) => {
                        self.count += i;
                        if self.count == REACHED_COUNT_SIGNAL_AMOUNT {
                            if let Some(tx) = self.tx {
                                tx.send(());
                                break;
                            }
                        }
                    }
                    Message::DecreaseBy(i) => {
                        self.count -= i;
                        if self.count == REACHED_COUNT_SIGNAL_AMOUNT {
                            if let Some(tx) = self.tx {
                                tx.send(());
                                break;
                            }
                        }
                    }
                }
            }
        });
        tx
    }
}
