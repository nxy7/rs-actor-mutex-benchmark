# Mutex vs Actor benchmark
This very simple benchmark intends to benchmark Mutex and Actors used to synchronize state.
In this implementation Actors reply with messages to callers which allows for very similar API to mutex implementation, the only
thing I intended to 'benchmark' here was what's more expensive - locking a value behind mutex or having
access to value limited to just one thread. In practice, it's measuring overhead of mutex vs oneshot channels.

# Mutex vs Actor that returns values
## Results
```
mutex vs actor/mutex    time:   [1.4809 ms 1.4839 ms 1.4872 ms]
                        change: [+288.15% +290.30% +292.68%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
mutex vs actor/actor    time:   [4.5462 ms 4.5587 ms 4.5713 ms]
                        change: [-96.680% -96.664% -96.648%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
```
As you can see mutex version turned out 3x faster. It doesn't really surprise me because this actor implementation was returning values
over oneshot channel, which is something that you might sometimes want, but I'm sure it slows it down significantly. I've mainly decided
to make actor return values because it made for easier benchmark setup. It's good to know though how much slower is returning values over channel
vs memory access. This benchmark results should apply to all actor methods that modify state and want to know new value instantly.

# Mutex vs Actor that doesn't have to return values
## Results
```
mutex vs actor 'async'/mutex
                        time:   [1.3995 ms 1.4029 ms 1.4067 ms]
                        change: [+17.433% +17.905% +18.354%] (p = 0.00 < 0.05)
mutex vs actor 'async'/actor
                        time:   [2.7879 ms 2.7944 ms 2.8008 ms]
                        change: [+48.814% +49.399% +49.947%] (p = 0.00 < 0.05)
```
I've added 'async' version of benchmark. Async here means that caller of actor doesn't want to get value back, so he's essentially just sending Message over
channel and leaves. This required additional check to know when Actor received all messages. To keep both implementations the same I've made Mutex also do
the same equiality check thinking that it would significantly increase bench time for Mutex, but to my surprise the equality check was so fast, that it
didn't make any difference in Mutex time. There was a difference for Actor implementation however. Without the need to send return signal on each Message
Actors became ~40% faster. They're still 2x slower than Mutexes though. 


# Higher concurrency
In first set of benchmarks I was spawning 10000 futures per bench iteration. I've decided to see what happens if I increase it by x10. Is the time going to change
linearly for both implementations or are Mutexes going to suffer because they have to fight for each lock?

## Results
```
  mutex vs actor 'sync'/mutex
                        time:   [16.629 ms 16.762 ms 16.908 ms]
                        change: [+11058% +11156% +11266%] (p = 0.00 < 0.05)
mutex vs actor 'sync'/actor
                        time:   [48.231 ms 48.551 ms 48.909 ms]
                        change: [+9403.5% +9498.0% +9591.7%] (p = 0.00 < 0.05)
mutex vs actor 'async'/mutex
                        time:   [16.613 ms 16.667 ms 16.730 ms]
                        change: [+1081.8% +1086.4% +1091.2%] (p = 0.00 < 0.05)
mutex vs actor 'async'/actor
                        time:   [32.053 ms 32.181 ms 32.353 ms]
                        change: [+1047.2% +1052.8% +1059.3%] (p = 0.00 < 0.05)
```
So that didn't really happen. Mutexes are still 2x faster than actors that don't return values and 3x faster than actors that return values.
I think that actors can make some designs simpler and mutexes can be hard to use, but when it comes to performance it seems that Mutexes
are clear winner.

 
