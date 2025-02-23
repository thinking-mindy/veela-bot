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
            tmpdata[i + 1].to_string()
        };
        let prev = if i < 1 {
            " 0x0 ".to_string()
        } else {
            tmpdata[i - 1].to_string()
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
                let mut nxt: Vec<String> = vec![];
                //nxt.insert_str(0, item["n"].as_str().unwrap());
                nxt.push(item["n"].to_string());
                nxt.push(json!(" 0x5 ").to_string());
                nxt.push(sitem["n"].to_string());

                let mut prev: Vec<String> =vec![];
                prev.push(item["p"].to_string());
                prev.push(json!(" 0x5 ").to_string());
                prev.push(sitem["p"].to_string());

                if nxt.contains(&sitem["n"].to_string()) || prev.contains(&sitem["p"].to_string()) {
                    if let None = found.join(" ").find(&sitem["w"].to_string()) {
                        fresult.push(json!({"w":sitem["w"],"n":json!(nxt.join(" ")),"p":json!(prev.join(" "))}));
                        found.push(sitem["w"].to_string());
                    }
                };
                done = true;
            }
        }
        if !done {fresult.push(item.clone())};
    } 
    
    for item in fresult.clone(){println!("{:?}",item);}

    /*
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
