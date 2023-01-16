trait TreasureBox{
    fn open(&self, key_no : i32)->bool;
    fn check(&self);
}
struct JewelryBox{
    price : i32,
    key_no : i32,
}
impl TreasureBox for JewelryBox{
    fn open(&self, key_no: i32)-> bool{
        self.key_no == key_no
    }
    fn check(&self){
        println!("gold treasure {} gold get",self.price);
    }
}
struct TrapBox{
    damage : i32,
}
impl TreasureBox for TrapBox{
    fn open(&self, _key: i32) -> bool{
        return true;
    }
    fn check(&self){
        println!("trap, damaged {}",self.damage);
    }
}
fn open_box(tbox: &impl TreasureBox, key_no : i32){
    if !tbox.open(key_no){
        println!("it is not key!");
        return;
    }
    tbox.check();
}
fn main() {
    let box1 = JewelryBox{
        price : 30,
        key_no : 1,
    };
    let box2 = TrapBox{damage:3};
    let box3 = JewelryBox{
        price : 20,
        key_no : 2,
    };
    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);


}
