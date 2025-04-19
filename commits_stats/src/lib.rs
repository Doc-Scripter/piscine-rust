pub use chrono::prelude::*;
pub use std::collections::HashMap;
pub use serde::{Deserialize,Serialize};
pub use json::*;
pub use std::fs;
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Author{
    name:String,
    email:String,
    date:String
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Tree{
    sha:String,
    url:String
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct  Verification{
    verified:bool,
    reason:String,
    signature:Option<String>,
    payload:Option<String>,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Commit{
    author:Author,
    committer:Author,
    message:String,
    tree:Tree,
    url:String,
    comment_count:i64,
    verification:Verification

}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct  AuthorMain{
    login:String,
    id:i64,
    node_id:String,
    avatar_url:String,
    gravatar_id:String,
    url:String,
    html_url:String,
    followers_url:String,
    following_url:String,
    gists_url:String,
    starred_url:String,
    subscriptions_url:String,
    organizations_url:String,
    repos_url:String,
    events_url:String,
    received_events_url:String,
    r#type:String,
    site_admin:bool
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Parents{
    sha:String,
    url:String,
    html_url:String,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct CommitData{
    sha:String,
    node_id:String,
    commit:Commit,
    url:String,
    html_url:String,
    author:AuthorMain,
    committer:AuthorMain,
    parents:Vec<Parents>
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