use serde_json::Value;
use std::io::Write;

pub fn get_memories() -> String {
    match std::fs::read_to_string("data/memories") {
        Ok(file) => file,
        Err(_) => "None".to_string(),
    }
}

pub fn roam_memories(d:String,mem:String)->String{
    let mut memories:Vec<Value>=vec![];
    let mut result:Vec<String>=vec![];
    let data:Vec<String>=d.split(" ").map(|x|x.to_string()).collect();
    for d in mem.split("\n").into_iter() {
        if d != "" {
            let tm: Value = serde_json::from_str(&d).unwrap();
            memories.push(tm)
        }
    }

    if memories.len()==0{
        "".to_string()
    }else{
        for d in memories {
            let tokens:Vec<String>=d["q"].to_string().split(" ").map(|x|x.to_string()).collect();
            let mut rate = 0;

            for t in tokens {
                for dt in &data{
                    if dt.to_lowercase() == t.to_lowercase(){
                        rate = rate + 1
                    }
                }
            }
            if (rate/data.len()).to_string()<0.8.to_string(){result.push(d["rp"].to_string())}
        }

        if !result.is_empty(){result[0].to_string()}
        else{"".to_string()}
    }
}

pub fn keep_memory(data:Value){
    //save learned data
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("data/memories")
        .unwrap();
        file.write((serde_json::to_string(&data).unwrap() + "\n").as_bytes()).unwrap();
}