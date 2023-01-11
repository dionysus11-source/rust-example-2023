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
        match self.head{
            None => self.head = Some(tmp),
            Some(ref mut ptr) => {
                let mut pp = ptr;
                loop{
                    match pp.link{
                        None => {
                            pp.link = Some(tmp);
                            break;},
                        Some(ref mut v) =>{
                            pp = v;
                        }
                    }
                }
            }
        }
    }
    pub fn get(&mut self, index:usize) -> Option<isize>{
        let mut cnt = 0;
        let mut ret:Option<isize> = None;
        match self.head{
            None => return None,
            Some(ref mut ptr) => {
                let mut pp = ptr;
                loop{
                            if cnt == index{
                                return Some(pp.data);
                            }
                    match pp.link{
                        None => return None,
                        Some(ref mut v) =>{
                            pp = v;
                        }
                    
                }
                cnt += 1;
            }
        }
    
    }
    }
}