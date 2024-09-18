fn main() {
    //vectors
    let _: Vec<i32> = Vec::new();  // if defining a vec as immutauble type needs to be specified

    // if you just define the vec as mutable and dont add values then also type needs to be specified
    //let mut v : Vec<i32> = Vec::new();

    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    let x = vec![1, 2]; // macro way of creating vector
}
