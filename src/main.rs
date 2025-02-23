use veela::minds::veela;

#[tokio::main]
async fn main() {
    let user_input="help".to_string();
    let f_result=veela::start(user_input).await;
    println!("{}",f_result);
}
