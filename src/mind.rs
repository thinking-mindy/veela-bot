use std::io::Write;
use rand::{self, Rng};
use serde_json::{json, Value};

pub fn get_learning_data() -> String {
    match std::fs::read_to_string("data/ll.txt") {
        Ok(file) => file,
        Err(_) => "None".to_string(),
    }
}

pub fn get_learned_data() -> String {
    match std::fs::read_to_string("data/mind") {
        Ok(file) => file,
        Err(_) => "None".to_string(),
    }
}

pub fn get_memories() -> String {
    match std::fs::read_to_string("data/memories") {
        Ok(file) => file,
        Err(_) => "None".to_string(),
    }
}

pub fn make_sentance(data: Vec<Value>) -> String {
    let mut s: Vec<String> = vec![];
    let learned: Vec<Value> = data;

    //group start words
    let mut rn = rand::thread_rng();
    let mut start: Vec<String> = vec![];

    let mut sobject: Value = json!({});
    for s in learned.clone() {
        if let Some(data) = s.as_object() {
            if let Some(prev) = data.get("p") {
                if prev.to_string().contains("0x0") {
                    start.push(data.get("w").unwrap().to_string());
                    sobject = s;
                }
            }
        }
    }

    //create a sentance

    //capitalize first word
    let tss = start[rn.gen_range(0..start.len())].as_str();
    let cap = tss.replacen(
        tss.get(1..2).unwrap(),
        tss.get(1..2).unwrap().to_uppercase().as_str(),
        1,
    );
    s.push(cap); //add first word to sentence

    //add rest of the words
    let mut body: Vec<Value> = vec![];
    for word in learned.clone() {
        if let Some(data) = word.as_object() {
            if let Some(nxt) = data.get("n") {
                if !nxt.to_string().contains("0x0") {
                    body.push(word);
                }
            }
        }
    }

    fn add_body(curr: Value, mut s: Vec<String>, body: Vec<Value>) -> Vec<String> {
        let mut done: bool = false;

        let mut prev: Value = json!({"":""});
        let mut rnv = rand::thread_rng();
        let bb = body;
        let mut res = s.clone();

        //add current word to sentance
        match curr.get("w") {
            Some(w) => s.push(w.to_string()),
            None => done = true,
        };

        //find nxt word
        match curr.get("n") {
            Some(nxt) => {
                let str = nxt.to_string();
                let tnxt: Vec<&str> = str.split("0x5").collect();
                let ttnxt = tnxt[rnv.gen_range(0..tnxt.len())];
                res.push(ttnxt.to_string());
                println!("{:?}",res);

                for b in bb.clone() {
                    let bref = b.get("w").unwrap().to_string();
                    if bref == ttnxt {
                        prev = b;
                    }
                }
            }
            None => done = true,
        }

        if !done {
            res = add_body(prev, res.clone(), bb);
        } else {
            if let Some(w) = curr.get("w") {
                s.push(w.to_string())
            };
        }

        res
    }

    add_body(sobject, s, body).join(" ")
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

            if (rate/data.len()).to_string()<0.8.to_string(){
                result.push(d["rp"].to_string())
            }
        }

        if !result.is_empty(){
            result[0].to_string()
        }else{
            "".to_string()
        }
    }
}

pub fn veela_local(data: String) -> String {
    let vdata = match std::fs::read_to_string("./src/data/mind") {
        Ok(file) => file,
        Err(_) => "None".to_string(),
    };
    let vdata = match serde_json::from_str(&vdata) {
        Ok(drjson) => drjson,
        Err(_) => {
            serde_json::json!({"sym":"None","dis":"None"})
        }
    };
    let mut rn = rand::thread_rng();

    //clean my data, remove stop words and charcters
    let junk = regex::Regex::new(r"\n|\d|\W").unwrap();
    let junk_words = regex::Regex::new(vdata["stop"].as_str().unwrap()).unwrap();
    let user_input = junk
        .replace_all(data.to_lowercase().as_str(), " ")
        .to_string();
    let user_input = junk_words.replace_all(user_input.as_str(), "").to_string();

    //remove empty strings
    let mut tokens: Vec<&str> = user_input.split(' ').filter(|x| x != &"").collect();

    //remove duplicates tokens
    tokens.sort();
    tokens.dedup();

    let mut results: Vec<serde_json::Value> = Vec::new();
    let final_result: String;

    if tokens.len() < 2 {
        for d in vdata["oneword"].as_array().unwrap() {
            let mut rate = 0;
            for data in &tokens {
                let v = d["keys1"].as_str().unwrap().to_string().to_lowercase();
                if v.find(data.to_lowercase().as_str()) != None {
                    rate = rate + 1
                }
            }
            results.push(json!({"rp":d["rp"],"rate":rate,"sug":d["sug"]}))
        }
        let mut results: Vec<serde_json::Value> = results
            .into_iter()
            .filter(|x| x["rate"].as_i64().unwrap() > 0)
            .collect();
        results.sort_by_key(|f| f["rate"].as_i64());

        final_result = match results.last() {
            Some(value) => value["rp"][0].to_string(),
            None => vdata["fallback"]["rp"][rn.gen_range(0..18)].to_string(),
        };
    } else {
        for d in vdata["morewords"].as_array().unwrap() {
            let mut rate = 0;
            for data in &tokens {
                let v = d["keys1"].as_str().unwrap().to_string().to_lowercase();
                if v.find(data.to_lowercase().as_str()) != None {
                    rate = rate + 1
                }
            }
            results.push(json!({"rp":d["rp"],"rate":rate,"sug":d["sug"]}))
        }
        let mut results: Vec<serde_json::Value> = results
            .into_iter()
            .filter(|x| x["rate"].as_i64().unwrap() > 0)
            .collect();
        results.sort_by_key(|f| f["rate"].as_i64());

        final_result = match results.last() {
            Some(value) => value["rp"][0].to_string(),
            None => vdata["fallback"]["rp"][rn.gen_range(0..18)].to_string(),
        };
    }

    final_result
}

pub async fn veela_online(data: String)->String{
    let result:String;
    let client = search_with_google::Client::default();
    let results = client.search(&data, 3, None).await;
    //println!("{:?}",results);
    if let Ok(result_list) = results {
        result=result_list[0].description.to_string();
    }else{
        result="I can't talk now".to_string();
    }
    result
}

pub fn learn(data: String)->Vec<Value> {
    let mut result: Vec<Value> = vec![];
    let mut fresult: Vec<Value> = vec![];
    let tmpdata: Vec<String> = data
        .to_lowercase()
        .replace(",", " ,")
        .replace("\n", " ")
        .split(" ")
        .map(|x| x.to_string())
        .filter(|x| x != " ")
        .collect();
    //println!("{:?}",tmpdata);

    //find  next and prev
    for (i, word) in tmpdata.iter().enumerate() {
        let nxt = if i == (tmpdata.len() - 1) {
            " 0x0 ".to_string()
        } else {
            tmpdata[i + 1].clone()
        };
        let prev = if i < 1 {
            " 0x0 ".to_string()
        } else {
            tmpdata[i - 1].clone()
        };
        result.push(json!({"w":word,"n":nxt,"p":prev}));
    }
    //println!("{:?}",result);
    //gropu same keys
    let mut found: Vec<String> = vec![];

    for (i, item) in result.iter().enumerate() {
        let mut done = false;
        for (k, sitem) in result.iter().enumerate() {
            if item["w"] == sitem["w"] && i != k {
                let mut nxt: String = "".to_string();
                nxt.insert_str(0, item["n"].as_str().unwrap());
                nxt.insert_str(0, " 0x5 ");
                nxt.insert_str(0, sitem["n"].as_str().unwrap());

                let mut prev: String = "".to_string();
                prev.insert_str(0, item["p"].as_str().unwrap());
                prev.insert_str(0, " 0x5 ");
                prev.insert_str(0, sitem["p"].as_str().unwrap());

                if sitem["n"] != nxt || sitem["p"] != prev {
                    if let None = found.join(" ").find(sitem["w"].as_str().unwrap()) {
                        fresult.push(json!({"w":sitem["w"],"n":nxt,"p":prev}));
                        found.push(sitem["w"].to_string());
                    }
                };
                done = true;
            }
        }
        if !done {
            fresult.push(item.clone())
        };
    } 
    
    /*
      for item in fresult{
          println!("{:?}",item);
      }
      println!("{:?}",found);

    //save learned data
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("data/mind")
        .unwrap();
    for v in fresult {
        file.write((serde_json::to_string(&v).unwrap() + "\n").as_bytes())
            .unwrap();
    }
       */
      fresult
}

pub fn keep_memory(data:Value){
    //save learned data
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("data/memories")
        .unwrap();
        file.write((serde_json::to_string(&data).unwrap() + "\n").as_bytes()).unwrap();
}
