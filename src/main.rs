use std::env::args;

#[tokio::main]
async fn main() {
    let vee=veela::mind::veela_online(args().nth(1).unwrap().to_string()).await;
    let begin_veela=veela::mind::make_sentance(veela::mind::learn(vee));
    println!("{}",begin_veela);
}
