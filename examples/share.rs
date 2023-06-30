use std::time::Duration;

use lib_rs::share::Share;
use lib_rs::share::ShareType;
use tokio::task;
use tokio::task::JoinError;
use tokio::task::JoinHandle;
use tokio::time::sleep;

fn spawn_task(task_number: i32, counter: ShareType<i32>, duration: Duration) -> JoinHandle<()> {
    println!("Task[{}] > started ...", task_number);

    task::spawn(async move {
        let mut counter = counter.write().await;
        
        sleep(duration).await;
        *counter += 1;
        
        println!("Task[{}] > Counter: {}", task_number, counter);
    })
}

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    let counter = Share::new(0);

    let task_01 = spawn_task(1, counter.clone(), Duration::new(1, 500));
    let task_02 = spawn_task(2, counter.clone(), Duration::new(3, 000));
    let task_03 = spawn_task(3, counter.clone(), Duration::new(2, 500));
    let task_04 = spawn_task(4, counter.clone(), Duration::new(0, 500));
    let task_05 = spawn_task(5, counter.clone(), Duration::new(6, 250));

    task_01.await?;
    task_02.await?;
    task_03.await?;
    task_04.await?;
    task_05.await?;

    Ok(())
}
