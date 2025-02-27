use serde_json::{json, Value};
use rand::{self, Rng};

pub fn get_learned_data() -> String {
    match std::fs::read_to_string("data/mind") {
        Ok(file) => file,
        Err(_) => "None".to_string(),
    }
}

//ROADMAP
//Inputs: VeelaData, TrainedData
//group TrainingData(StartWords, BodyWords)
//search StartWords in VeelaData
//if found: begin from that word
//else not found: use defoult fist word
//make whole sentance

//MAKING A SENTENCE
//start from start word
//get next word in VeelaData esle full Stop. Repeat until all words are used in the sentance(when sentence length is greater than VeelaData, all stringfied)
//verify final sentence using learned data and autocorrect

pub fn make_sentance(veela_data: String) -> String {
    let mut s: Vec<String> = vec![];
    
    //TrainedData
    let mut learned:Vec<Value> = vec![];
    for d in get_learned_data().split("\n").into_iter(){
        if d != "" {
            let tm: Value = serde_json::from_str(&d).unwrap();
            learned.push(tm)
        }
    }
    ////Group TrainingData(StartWords, BodyWords)
    let mut rn = rand::thread_rng();
    let mut start: Vec<String> = vec![];
    let mut body: Vec<Value> = vec![];
    let mut sobject: Value = json!({});
    let mut f_tmp:bool=false; //find the fist stsrting word and stop wasting time re-assigning it again, i hope lols

    for s in learned {
        if let Some(veela_data) = s.as_object() {
            //StartWords
            if let Some(prev) = veela_data.get("p") {
                if prev.to_string().contains("0x0") && !f_tmp {
                    start.push(veela_data.get("w").unwrap().to_string());
                    sobject = s.clone();
                    f_tmp=true;
                }
            }
            //BodyWords
            if let Some(nxt) = veela_data.get("n") {
                if !nxt.to_string().contains("0x0") {
                    body.push(s);
                }
            }
        }
    }

    //search StartWords in VeelaData
    //if found: begin from that word
    //else not found: use defoult fist word
    let mut first_word:String = String::from("");

    for word in veela_data.split("").map(|x|x.to_string()).into_iter(){
        if start.contains(&word){
            first_word=word;
        }else{
            first_word=start[0].clone();
        }
    }
    //capitalize first word
    let cap = first_word.replacen(
        first_word.get(1..2).unwrap(),
        first_word.get(1..2).unwrap().to_uppercase().as_str(),
        1,
    );
    s.push(cap); //add first word to sentence

    //MAKING A SENTENCE
    //start from start word

    //get next word in VeelaData esle full Stop. Repeat until all words are used in the sentance(when sentence length is greater than VeelaData, all stringfied)
    //verify final sentence using learned data and autocorrect

    fn add_body(curr: Value, s: Vec<String>, body: Vec<Value>) -> Vec<String> {
        let mut done: bool = false;

        let mut prev: Value = json!({});
        let mut rnv = rand::thread_rng();
        let bb = body;
        let mut res = s.clone();

        //add current word to sentance
        match curr.get("w") {
            Some(w) => res.push(w.to_string()),
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
                res.push(w.to_string())
            };
        }
        res
    }

    add_body(sobject, s, body).join(" ")
}
