#[derive(Debug, Clone)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug,Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }
  pub fn add_worker(&mut self, role: String, name: String) {
    let new_worker = Box::new(Worker {
        role,
        name,
        next: None,
    });

    let mut current = &mut self.grade;
    
    // Traverse the list until we find the end
    while let Some(worker) = current {
        current = &mut worker.next;
    }
    
    // Insert the new worker at the end
    *current = Some(new_worker);
}

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.clone() {
            None => None,
            Some(mut head) => {
                let name = head.name.clone();
                self.grade = head.next.take();
                Some(name)
            }
        }
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        match  self.grade.clone() {
                None=>None,
                Some(last)=>{
                    Some((last.name,last.role))
                },
        }
    }
}
                