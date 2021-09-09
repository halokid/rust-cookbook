Concurrency In Rust; Can It Stack Up Against Go’s Goroutines?
==========================


In Rust, there are two approaches we can take to run code concurrently. Async/Await, and threading. Async/Await is a paradigm that is orthogonal to threading, which means that it has the potential to run tasks on a single thread OR on multiple threads depending on the executor that is used.

Threading on its own makes use of multiple cores and uses typical operating-system threads.