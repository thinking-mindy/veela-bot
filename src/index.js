import express from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';
import {MongoClient} from 'mongodb';
import { getTranslationText } from "lingva-scraper";
import Corr from 'autocorrect'


//import the data set
import data from './dataset/it_dataset.json' assert { type: "json" };

import idata from './dataset/interest.json' assert { type: "json" };
import sdata from './dataset/strenghts.json' assert { type: "json" };


const app=express();
app.use(cors());
app.use(bodyParser.json());
const autocorrect = Corr();
let db

app.use(cors());
app.use(bodyParser.json());


async function start(){
  const client = new MongoClient("mongodb+srv://mimo:mimi@cluster0.j9xtsye.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0");
  await client.connect().catch((e)=>{console.log(`Can't connect bacause: \n${e}`)});
  db=client.db("cgb");
  app.listen(4009,()=>console.log("Listening on 4009"));
}
start()

//testing pinging server
app.get('/',(req,res)=>{res.send("<b>Hello there, i'm up and running.</b>")})

//post attendance user
app.post('/register',function(req,res){
  console.log("Registering attempt by student")
  let found=false;
  const user={name:req.body.mail,mail:Buffer.from(req.body.pass).toString('base64')};
  db.collection("students").findOne(user,function(err,result){
    if(err) throw err
    if(result?.length){found=true;res.send({aye:1})}})
    !found&&db.collection("students").insertOne({mail:req.body.mail,pass:Buffer.from(req.body.pass).toString('base64'),chats:[{qns:"",ans:""}]},
    function(err,ress){if(err)throw err;console.log("1 user added");res.send({aye:0});});
})

app.post('/resetpassword',function(req,res){
  db.collection("students").findOne({mail:req.body.mail},function(err,result){
    if(err) throw err
    if(result?.mail){
      res.send({aye:1});
      //sent email
    }else{res.send({aye:0})}
  })
})
app.post('/resetpassword-link',function(req,res){
  db.collection('students').updateOne({name:req.body.name,mail:req.body.mail},{$set:{pass:Buffer.from(req.body.pass).toString('base64')}},
  function(err,ress){if(err)throw err;console.log("1 updated well")})
})

app.post('/login',function(req,res){
  console.log("Login attempt by student")
  const user={mail:req.body.mail,pass:Buffer.from(req.body.pass).toString('base64')};
  db.collection("students").findOne(user,function(err,result){
    if(err) throw err
    if(result?.mail){res.send({aye:1,mail:result.mail})}else{res.send({aye:0})}
  })
})

//get exercise
app.post('/gchat',(req,res)=>{
  db.collection("students").findOne({mail:req.body.mail},function(err,result){
    if(err) throw err
    res.send({chats:result?.chats})
  })
})

//handle form
app.post('/subform',(req,res)=>{
const fdata=req.body.fdata;
let allint=fdata.interests.join(',')+","+fdata.ointerests;
let allstr=fdata.strength.join(',')+","+fdata.ostrength;

let ians=[],sans=[]
//interests
idata.interests.forEach((i)=>{
  let tmpres=[]
  allint.split(',').forEach((token)=>{
    if(i.keys1.includes(token)){tmpres.push(1)}
  })
  if(tmpres.length){ians.push({len:tmpres.length,prog:i.rp[0]})}
})
ians.sort((a,b)=>parseInt(b.len)-parseInt(a.len)); 

//strength
sdata.strength.forEach((s)=>{
  let tmpres=[]
  allstr.split(',').forEach((token)=>{
    if(s.keys1.includes(token)){tmpres.push(1)}
  })
  if(tmpres.length){sans.push({len:tmpres.length,prog:s.rp[0]})}
})
sans.sort((a,b)=>parseInt(b.len)-parseInt(a.len)); 

let resultprog=ians[0]?ians[0]:sans[0]
if(resultprog){res.send({aye:1,info:`From the infomation you have provided, it seems you can qualify for ${resultprog.prog}.`})}
else{res.send({aye:1,info:"Unfortunately i couldn't find the best program that suits you with regard to the information you provided. Please retake the Assessment Form."})}
})

//an api route for the chatbot
app.post('/chat',async function(req,res){

  const stop=data.stop;let results=[]
  const d=/\d/g;const cha=/\W/g;
  let rp={};

  const tmpv=req.body.lang==="en"?req.body.value:await getTranslationText("auto","en",req.body.value);
  let values=tmpv.split(' ');
  function log(x){console.log(x)};

    values=values.filter(x=>!stop.includes(x.toLowerCase())); //remove the stop words from the user input
    values=values.filter(xz=>!d.test(xz)); //remove digits from the tokens
    values=values.filter(xyz=>/\w/.test(xyz)); //remove non-word items from the tokens

    //values=values.map(xyz=>autocorrect(xyz)); //autocorrect each token

    values=[...new Set(values)]; //remove duplicate

    data.oneword.forEach((mood)=>{
      let vresult=[];const tmpkey=Array.isArray(mood.keys1)?mood.keys1.join(' '):mood.keys1;
      values.forEach((x)=>{
        let tmpre=x.replace(cha,""); //remove char in word
        if(tmpkey.toLowerCase().match('\\b'+tmpre.toLowerCase()+'\\b')){vresult.push(1)} //check if token in the keys
      })

      if(vresult.length){results.push({len:vresult.length,rps:mood.rp,bot:mood.bot,asses:mood.asses,sugy:mood.sug})} //if tokens finding exists for a question

    });

    if(!results.length){
      data.morewords.forEach((mood)=>{
        let vresult=[];const tmpkey=Array.isArray(mood.keys1)?mood.keys1.join(' '):mood.keys1;
        values.forEach((x)=>{
        let tmpre=x.replace(cha,""); //remove char in word
          if(tmpkey.toLowerCase().match('\\b'+tmpre.toLowerCase()+'\\b')){vresult.push(1)} //check if token in the keys
        })
  
        if(vresult.length>1){results.push({len:vresult.length,rps:mood.rp,bot:mood.bot,asses:mood.asses,sugy:mood.sug})} //if tokens finding exists for a question
  
      });
    }

    results.sort((a,b)=>parseInt(b.len)-parseInt(a.len)); //get the highest probable response
    rp=results[0]

    //randomize the suggestions

    let saverp=""
    //check if there no response found and respond with fallback
    if(!rp){
      let finalrp=req.body.lang==="en"?data.els.rp[Math.floor(Math.random()*data.els.rp.length)]:await getTranslationText("en",req.body.lang,data.els.rp[Math.floor(Math.random()*data.els.rp.length)])
      let sug=data.els.sug;
      for(let i in [1,2,3,4,5]){sug=sug.sort(()=>Math.random()-0.5);}
    
      res.send({rps:finalrp,sugs:sug.slice(0,4),bot:true,asses:false});
      saverp=finalrp;
    }

    //check if there's a response found and respond with thee if available
    if(rp){
      let finalrpp=req.body.lang==="en"?rp.rps[Math.floor(Math.random()*rp.rps.length)]:await getTranslationText("en",req.body.lang,rp.rps[Math.floor(Math.random()*rp.rps.length)])
      let sug=rp.sugy;
      for(let i in [1,2,3,4,5]){sug=sug.sort(()=>Math.random()-0.5);}
      res.send({rps:finalrpp,sugs:sug.slice(0,4),bot:rp.bot,asses:rp.asses});
      saverp=finalrpp;
    }

    if(req.body.mail){
    db.collection("students").findOne({mail:req.body.mail},function(err,result){
      if(err) throw err
      let fchats=result?.chats?result.chats:[{qns:"",ans:""}];
      fchats.push({qsn:req.body.value,ans:saverp,date:new Date().toString()})
      db.collection('students').updateOne({mail:req.body.mail},{$set:{chats:fchats}},function(err,ress){if(err)throw err;console.log(`${req.body.mail} just talked to the bot`)})
    })
  }
})
