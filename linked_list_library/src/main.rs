mod slist;
fn main() {
    let mut list = slist::List::new();
    list.unshift(100);
    list.unshift(200);
    list.unshift(300);
    list.push(400);

    let b = list.get(3).unwrap();
    //let b = list.get(1).unwrap();
    //let b = list.get(2).unwrap();
    println!("result {}",b);
}