
use serde_json::json;

fn veela(data:&String)->String{
    let vdata=match std::fs::read_to_string("./src/data/mind"){Ok(file)=>{file},Err(_)=>"None".to_string()};
    let vdata=match serde_json::from_str(&vdata) {Ok(drjson)=>{drjson},Err(_)=>{serde_json::json!({"sym":"None","dis":"None"})}};
    
    //clean my data, remove stop words and charcters
    let junk = regex::Regex::new(r"\n|\d|\W").unwrap();
    let junk_words = regex::Regex::new(vdata["stop"].as_str().unwrap()).unwrap();
    let user_input=junk.replace_all(data.to_lowercase().as_str(), " ").to_string();
    let user_input=junk_words.replace_all(user_input.as_str(), "").to_string();

    //remove empty strings
    let mut tokens:Vec<&str>=user_input.split(' ').filter(|x|x!=&"").collect();

    //remove duplicates tokens
    tokens.sort();
    tokens.dedup();

    let mut results:Vec<serde_json::Value>=Vec::new();
    let mut final_result:String=String::new();

    if tokens.len()<2 {
        for d in vdata["oneword"].as_array().unwrap(){
            let mut rate=0;
            for data in &tokens{
                let v=d["keys1"].as_str().unwrap().to_string().to_lowercase();
                if v.find(data.to_lowercase().as_str())!=None{rate=rate+1}
            }
            results.push(json!({"rp":d["rp"],"rate":rate,"sug":d["sug"]}))
        }
        let mut results:Vec<serde_json::Value>=results.into_iter().filter(|x|x["rate"].as_i64().unwrap()>0).collect();
        results.sort_by_key(|f|f["rate"].as_i64());

        final_result=match results.last(){
            Some(value)=>{value["rp"][0].to_string()},
            None=>{
                vdata["fallback"]["rp"][0].to_string()
            }};
    }else{
        for d in vdata["morewords"].as_array().unwrap(){
            let mut rate=0;
            for data in &tokens{
                let v=d["keys1"].as_str().unwrap().to_string().to_lowercase();
                if v.find(data.to_lowercase().as_str())!=None{rate=rate+1}
            }
            results.push(json!({"rp":d["rp"],"rate":rate,"sug":d["sug"]}))
        }
        let mut results:Vec<serde_json::Value>=results.into_iter().filter(|x|x["rate"].as_i64().unwrap()>0).collect();
        results.sort_by_key(|f|f["rate"].as_i64());
    
        final_result=match results.last(){
            Some(value)=>{value["rp"][0].to_string()},
            None=>{
                vdata["fallback"]["rp"][0].to_string()
            }};
        }

    final_result

}

fn main() {
    let umsg: String="you are lame".to_string();
    let res=veela(&umsg);
    println!("The response is {:?}",res);

}

