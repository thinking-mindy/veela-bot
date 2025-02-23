use super::{make_sentances,memories,mind,learn};

pub async fn start(a:String)->String{
    let r_m=memories::roam_memories(a.clone(),memories::get_memories());
    let vee=mind::mind_local(a.clone());
    let begin_veela=make_sentances::make_sentance(learn::learn(vee));
    if r_m.is_empty() {memories::keep_memory(serde_json::json!({"q":a,"rp":begin_veela}));}
    let result=if !r_m.is_empty() {r_m}else{begin_veela};
    result
}