use tokio::sync::mpsc;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tokio::time::{sleep, Duration};


async fn with_close_channel() {
    let (mut tx, mut rx) = mpsc::channel(32);

    // Запускаем задачу, которая читает из канала
    let reader = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("Получено значение: {}", value);
        }
        println!("Канал закрыт, задача завершена.");
    });

    // Отправляем значения
    for i in 0..5 {
        tx.send(i).await.unwrap();
        sleep(Duration::from_millis(100)).await;
    }

    // Закрываем канал
    drop(tx);

    // Ждем завершения задачи чтения
    reader.await.unwrap();
}

async fn with_cancellation_token() {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // Запускаем задачу
    let task = tokio::spawn(async move {
        loop {
            select! {
                _ = token_clone.cancelled() => {
                    println!("Задача отменена.");
                    break;
                }
                _ = sleep(Duration::from_secs(1)) => {
                    println!("Работаю...");
                }
            }
        }
    });

    // Ждем некоторое время и затем отменяем задачу
    sleep(Duration::from_secs(3)).await;
    token.cancel(); // Отмена задачи

    // Ждем завершения задачи
    task.await.unwrap();
}


#[tokio::main]
async fn main() {
    println!("Base");
    with_close_channel().await;

    println!("Token");
    with_cancellation_token().await;
}
