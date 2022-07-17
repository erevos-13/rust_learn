fn main() {
    let num_list = [1, 2, 3, 4, 5, 6];
    let months = ["orfeas", "ilina", "lida"];
    let num_list2: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let num_list3 = ["HI"; 10];
    println!("{}", num_list[0]);

    let mut index = 0;
    while index < 5 {
        println!("{}", num_list[index]);
        index = index + 1;
    }
    for elem in &num_list3 {
        println!("{}", elem);
    }
    let b = months.iter();
    // println!("{}", &b.count());
    for i in b.rev() {
        println!("{}", i);
    }
    // tuples
    let tuples = (10, 12, "orfeas");
    println!("{}",tuples.0);
    
}
