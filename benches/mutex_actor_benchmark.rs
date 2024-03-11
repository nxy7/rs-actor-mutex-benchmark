use std::{ops::Range, sync::Arc, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use futures::future::join_all;
use rs_actor_mutex_benchmark::{
    actors::{BenchActor, Message},
    mutex::BenchMutex,
    parking_lot_mutex::BenchParkingLotMutex,
    std_mutex::BenchStdMutex,
    REACHED_COUNT_SIGNAL_AMOUNT,
};
use tokio::{
    runtime::Runtime,
    sync::{mpsc, oneshot},
};

const iter_range: Range<i64> = 0..100000;

fn benchmark_async(c: &mut Criterion) {
    let mut group = c.benchmark_group("mutex vs actor 'async'");

    group.bench_function("mutex", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let (tx, rx) = mpsc::channel(100);
            let bench_mutex = Arc::new(BenchMutex::new(tx));

            let bench_tasks = iter_range
                .map(|_| {
                    let m_copy = bench_mutex.clone();
                    async move {
                        m_copy.increase_by_checked(1).await;
                        m_copy.increase_by_checked(1).await;
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;

            assert_eq!(bench_mutex.get().await, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.bench_function("actor", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let (tx, rx) = oneshot::channel();
            let actor = BenchActor::new(tx);
            let actor_tx = actor.start().await;

            let bench_tasks = iter_range
                .map(|_| {
                    let actor_tx = actor_tx.clone();

                    async move {
                        actor_tx.send(Message::IncreaseBy(1)).await.unwrap();
                        actor_tx.send(Message::IncreaseBy(1)).await.unwrap();
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;
            rx.await.unwrap();

            // let (tx, rx) = oneshot::channel();
            // actor_tx.send(Message::Get(tx)).await.unwrap();
            // let s = rx.await.unwrap();
            // assert_eq!(s, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.finish();
}
fn benchmark_sync(c: &mut Criterion) {
    let mut group = c.benchmark_group("mutex vs actor 'sync'");

    group.bench_function("mutex", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let mutex = Arc::new(BenchMutex::default());

            let bench_tasks = iter_range
                .map(|_| {
                    let m_copy = mutex.clone();
                    async move {
                        m_copy.increase_by(2).await;
                        m_copy.decrease_by(0).await;
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;

            assert_eq!(mutex.get().await, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.bench_function("actor", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let actor = BenchActor::default();
            let actor_tx = actor.start().await;

            let bench_tasks = iter_range
                .map(|_| {
                    let actor_tx = actor_tx.clone();

                    async move {
                        let (tx, rx) = oneshot::channel();
                        actor_tx.send(Message::IncreaseBySync(2, tx)).await.unwrap();
                        let _ = rx.await.unwrap();
                        let (tx, rx) = oneshot::channel();
                        actor_tx.send(Message::DecreaseBySync(0, tx)).await.unwrap();
                        let _ = rx.await.unwrap();
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;

            let (tx, rx) = oneshot::channel();
            actor_tx.send(Message::Get(tx)).await.unwrap();
            let s = rx.await.unwrap();
            assert_eq!(s, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.finish();
}
fn benchmark_mutexes(c: &mut Criterion) {
    let mut group = c.benchmark_group("std mutex vs tokio mutex");

    group.bench_function("tokio mutex", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let mutex = Arc::new(BenchMutex::default());

            let bench_tasks = iter_range
                .map(|_| {
                    let m_copy = mutex.clone();
                    async move {
                        m_copy.increase_by(2).await;
                        m_copy.decrease_by(0).await;
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;

            assert_eq!(mutex.get().await, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.bench_function("std mutex", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let mutex = Arc::new(BenchStdMutex::default());

            let bench_tasks = iter_range
                .map(|_| {
                    let m_copy = mutex.clone();
                    async move {
                        m_copy.increase_by(2).await;
                        m_copy.decrease_by(0).await;
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;

            assert_eq!(mutex.get().await, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.finish();
}

fn benchmark_std_parking_lot_mutex(c: &mut Criterion) {
    let mut group = c.benchmark_group("std mutex vs parking lot mutex");

    group.bench_function("parking lot mutex", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let mutex = Arc::new(BenchParkingLotMutex::default());

            let bench_tasks = iter_range
                .map(|_| {
                    let m_copy = mutex.clone();
                    async move {
                        m_copy.increase_by(2).await;
                        m_copy.decrease_by(0).await;
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;

            assert_eq!(mutex.get().await, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.bench_function("std mutex", |b| {
        b.to_async(Runtime::new().unwrap()).iter(|| async {
            let mutex = Arc::new(BenchStdMutex::default());

            let bench_tasks = iter_range
                .map(|_| {
                    let m_copy = mutex.clone();
                    async move {
                        m_copy.increase_by(2).await;
                        m_copy.decrease_by(0).await;
                    }
                })
                .collect::<Vec<_>>();

            join_all(bench_tasks).await;

            assert_eq!(mutex.get().await, REACHED_COUNT_SIGNAL_AMOUNT);
        });
    });

    group.finish();
}

criterion_group! {
    name = benching;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    // targets = benchmark_sync, benchmark_async, benchmark_mutexes
    // targets =  benchmark_mutexes
    targets = benchmark_std_parking_lot_mutex
}

criterion_main!(benching);
