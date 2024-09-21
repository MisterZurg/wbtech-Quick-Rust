# L1.17
Реализовать структуру-счетчик, которая будет инкрементироваться в конкурентной среде. 
По завершению программа должна выводить итоговое значение счетчика.

## Ref.
- [Spawning 100 threads and incrementing a shared counter, can this be optimzed/improved to complete in 1 second?](https://stackoverflow.com/questions/73737825/spawning-100-threads-and-incrementing-a-shared-counter-can-this-be-optimzed-imp)
- [std::sync::MutexGuard](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html)