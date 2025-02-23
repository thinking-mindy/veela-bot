use serde_json::{json, Value};
use rand::{self, Rng};

pub fn make_sentance(data: Vec<Value>) -> String {
    let mut s: Vec<String> = vec![];
    let learned: Vec<Value> = data;

    //group start words
    let mut rn = rand::thread_rng();
    let mut start: Vec<String> = vec![];
    let mut body: Vec<Value> = vec![];
    let mut sobject: Value = json!({});

    for s in learned {
        if let Some(data) = s.as_object() {
            if let Some(prev) = data.get("p") {
                if prev.to_string().contains("0x0") {
                    start.push(data.get("w").unwrap().to_string());
                    sobject = s.clone();
                }
            }
            if let Some(nxt) = data.get("n") {
                if !nxt.to_string().contains("0x0") {
                    body.push(s);
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

    fn add_body(curr: Value, mut s: Vec<String>, body: Vec<Value>) -> Vec<String> {
        let mut done: bool = false;

        let mut prev: Value = json!({});
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
                //get one nword in next words
                let str = nxt.to_string();
                let tnxt: Vec<&str> = str.split("0x5").collect();
                let ttnxt = tnxt[rnv.gen_range(0..tnxt.len())];
                println!("{ttnxt}");
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
        println!("Status: {} on {:?}",done,prev);
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
