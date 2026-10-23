use tokio::time::{sleep, Duration};

async fn send_email(){
    sleep(Duration::from_secs(3)).await;
    println!("email sent");
}
#[tokio::main]
async fn main(){
    println!("start");
    let is_email_sent = tokio::spawn(async {
        send_email().await;
    });
    println!("doing other work");
    is_email_sent.await.unwrap();
    println!("end");
}