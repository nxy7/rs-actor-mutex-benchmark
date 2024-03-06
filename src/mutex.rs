use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot, watch, Mutex};

use crate::REACHED_COUNT_SIGNAL_AMOUNT;

#[derive(Default)]
pub struct BenchMutex {
    count: Mutex<i64>,
    /// optional channel to signal when implementation reached REACHED_COUNT_SIGNAL_AMOUNT
    tx: Option<mpsc::Sender<()>>,
}

impl BenchMutex {
    pub fn new(m: mpsc::Sender<()>) -> Self {
        Self {
            tx: Some(m),
            ..Default::default()
        }
    }

    pub async fn increase_by(&self, i: i64) {
        (*self.count.lock().await) += i;
    }
    pub async fn decrease_by(&self, i: i64) {
        (*self.count.lock().await) -= i;
    }
    pub async fn increase_by_checked(&self, i: i64) {
        let mut v = self.count.lock().await;
        *v += i;
        if *v == REACHED_COUNT_SIGNAL_AMOUNT {
            if let Some(tx) = self.tx.clone() {
                tx.send(()).await;
            }
        }
    }
    pub async fn decrease_by_checked(&self, i: i64) {
        let mut v = self.count.lock().await;
        *v -= i;
        if *v == REACHED_COUNT_SIGNAL_AMOUNT {
            if let Some(tx) = self.tx.clone() {
                tx.send(()).await;
            }
        }
    }
    pub async fn get(&self) -> i64 {
        *self.count.lock().await
    }
}
