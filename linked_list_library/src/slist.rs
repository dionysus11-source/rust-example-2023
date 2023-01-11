pub struct Node{
    data : isize,
    link : Option<Box<Node>>,
}

pub struct List{
    head : Option<Box<Node>>,
}

impl List{
    pub fn new() -> Self{
        Self{head: None}
    }

    pub fn unshift(&mut self, val: isize){
       let tmp = Box::new(Node{data : val, link: self.head.take()});
       self.head = Some(tmp);
    }
    pub fn push(&mut self, v:isize){
       let tmp = Box::new(Node{data : v, link: None});
        match self.head.as_ref(){
            None => self.head = Some(tmp),
            Some(ptr) => {}
        }

       
    }
    pub fn get(&self){

    }
}