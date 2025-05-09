
#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
// #[derive(Debug)]
impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Node { value, next: None });
            return;
        }
      
        let new_node = Node {
             value,
            next: Some(Box::new(self.head.take().unwrap())),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        // println!("{:#?}",self.head.as_ref());
        if self.head.is_none() {
            return;
        }
        self.head.take().map(|x|{
            
            if x.next.is_none(){
                return;
            }
            self.head=Some(*x.next.unwrap());
            
        });
    }

    pub fn len(&self) -> usize {
        let mut count=0.0;
        let mut current=self.head.as_ref();
        while let Some(node)=current{
          
            
            count+=1.0;
            current=node.next.as_ref().map(|n|n.as_ref());
        }
        count as  usize
    }
}
