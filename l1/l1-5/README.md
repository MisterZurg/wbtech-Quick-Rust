## l1.5
Программа должна аккуратно завершаться по нажатию Ctrl+C. Выбрать и обосновать способ завершения работы всех воркеров.

Использовать tokio и flume (или другую аналогичную библиотеку для spmc/mpmc-каналов).


### Ref.
[Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown)


Сразу быоло понятно, что делать поскольку в Go используется такой-же подход:
```go
sigQuit := make(chan os.Signal, 2)
signal.Notify(sigQuit, syscall.SIGINT, syscall.SIGTERM)

eg, _ := errgroup.WithContext(context.Background())

eg.Go(func() error {
    select {
    case s := <-sigQuit:
        return fmt.Errorf("captured signal: %v", s)
    }
})

server := &http.Server{}

go func() {
    _ = server.ListenAndServe() // check error!
}()

if err := eg.Wait(); err != nil {
    logger.Infof("gracefully shutting down the server: %v", err)
}

_ = server.Shutdown(context.Background()) // check error!
```