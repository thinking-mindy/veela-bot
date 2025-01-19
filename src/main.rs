use serde_json::json;

#[tokio::main]
async fn main() {
    let user_input="help".to_string();
    let r_m=veela::mind::roam_memories(user_input.clone(),veela::mind::get_memories());
    let vee=veela::mind::veela_local(user_input.clone());
    let begin_veela=veela::mind::make_sentance(veela::mind::learn(vee));
    if r_m.is_empty() {veela::mind::keep_memory(json!({"q":user_input,"rp":begin_veela}));}
    let f_result=if !r_m.is_empty() {r_m}else{begin_veela};
    println!("{}",f_result);
}
