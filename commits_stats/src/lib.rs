pub use chrono::prelude::*;
pub use std::collections::HashMap;
pub use serde::{Deserialize,Serialize};
pub use json::*;
pub use std::fs;
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Author{
    pub name:String,
    pub email:String,
    pub date:String
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Tree{
    pub sha:String,
    pub url:String
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct  Verification{
    pub verified:bool,
    pub reason:String,
    pub signature:Option<String>,
    pub payload:Option<String>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Commit{
   pub  author:Author,
    pub committer:Author,
    pub message:String,
    pub tree:Tree,
    pub url:String,
    pub comment_count:i64,
    pub verification:Verification

}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct  AuthorMain{
pub login:String,
pub id:i64,
pub node_id:String,
pub avatar_url:String,
pub gravatar_id:String,
pub url:String,
pub html_url:String,
pub followers_url:String,
pub following_url:String,
pub gists_url:String,
pub starred_url:String,
pub subscriptions_url:String,
pub organizations_url:String,
pub repos_url:String,
pub events_url:String,
pub received_events_url:String,
pub r#type:String,
pub site_admin:bool
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Parents{
pub sha:String,
pub url:String,
pub html_url:String,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct CommitData{
pub sha:String,
pub node_id:String,
pub commit:Commit,
pub url:String,
pub html_url:String,
pub author:AuthorMain,
pub committer:AuthorMain,
pub parents:Vec<Parents>
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut res:HashMap<String, u32>=HashMap::new();
   

    let data:Vec<CommitData>=serde_json::from_str(&data.dump()).unwrap();
    for val in data.iter(){
        let mut count=0;
        let commit_name=&val.commit.author.name;
        data.iter().for_each(|x|if &x.commit.author.name==commit_name{
            count+=1;
        });
        res.insert(commit_name.clone(), count);
    }
    res
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut res:HashMap<String, u32>=HashMap::new();
    let data:Vec<CommitData>=serde_json::from_str(&data.dump()).unwrap();
    for val in data.iter(){
        let mut count=0;
        let commit_date=&val.commit.author.date;
        let date_time1=DateTime::parse_from_rfc3339(&commit_date).unwrap();
        let week_number1=date_time1.iso_week();
        data.iter().for_each(|x|{
        let commit_date=&x.commit.author.date;
        let date_time=DateTime::parse_from_rfc3339(&commit_date).unwrap();
        let week_number=date_time.iso_week();

            if week_number==week_number1{
            count+=1;
        }});
        let res_date:Vec<String>=commit_date.split("-").map(|x|x.to_string()).collect();

        res.insert(format!("{}-W{}",res_date[0],week_number1.week().to_string()), count);
    }
    res
}