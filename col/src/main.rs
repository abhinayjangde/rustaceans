use tokio::time::{sleep, Duration};

async fn greet1(){
    sleep(Duration::from_secs(2)).await;
    println!("hello, abhi 1");
}
async fn greet2(){
    sleep(Duration::from_secs(2)).await;
    println!("hello, abhi 2");
}
#[tokio::main]
async fn main(){
    println!("start");
    tokio::join!(greet1(),greet2());
    println!("end");
}