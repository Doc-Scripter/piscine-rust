
pub struct Message{
    pub content : String,
    pub user: String,
  }
  
  impl Message{
    #[allow(dead_code)]
    pub fn new(content:String,user:String,)->Message{
      Message{content:content,user:user}
    }
    pub fn send_ms<'a>(self)->Option<String>{
            if self.content==""||self.content.contains("stupid"){
              None
              }else{
            
            Some(self.content)
        }
    }
  }
  
  pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message.to_string(), "new".to_string());
      if msg.send_ms().is_some(){
        Ok(message)

      }else{
        Err("ERROR: illegal")
      }
  }
  