use node::Node;

mod cursor;
mod node;

fn main() {
    let mut list_a: Node<i32> = Node::new();
    let mut list_b: Node<i32> = Node::new();

    list_a.push(3);
    list_a.push(4);
    list_a.push(5);

    list_b.push(6);
    list_b.push(8);
    list_b.push(9);

    println!("{}", list_a.pop().unwrap());
    println!("{}", list_a.pop().unwrap());
    println!("{}", list_a.pop().unwrap());

    println!("=======================");

    for i in list_b.clone() {
        println!("{}", i);
    }

    println!("=======================");

    for i in list_b.clone().into_iter().filter(|x| x % 2 == 0) {
        println!("{}", i);
    }
}
