use std::collections::HashMap;

fn main() {
    //vectors
    let _: Vec<i32> = Vec::new();  // if defining a vec as immutauble type needs to be specified

    // if you just define the vec as mutable and dont add values then also type needs to be specified
    //let mut v : Vec<i32> = Vec::new();

    let mut v = Vec::new();
    v.push(1);
    v.push(2);

    let x = vec![1, 2]; // macro way of creating vector

    //accessing vectors
    let item = v.get(0);
    println!("Element at index 0 is : {}", item.unwrap());
    let item = &x[1];
    println!("Element at index 1 is : {}", item);

    // borrowing rules
    let y = &v[0];
    // this cannot be done as y holds immutable reference and pushing new elements in vec can
    // cause to deallocated old ref.
    // v.push(3);
    println!("Element at index 0 is: {}", y);

    // iterating over a vector
    for item in &v {
        print!("{item} ");
    }

    // modifying element while iterating, use dereference operator * before modifying elements
    // make sure the vector is mutable 
    for item in &mut v{
        *item += 50
    }
    println!();
    for item in &v {
        print!("{item} ");
    }
    println!();

    hash_maps();
}

fn hash_maps() {
    let mut map = HashMap::new();
    map.insert("Blue", 2);
    map.insert("Yellow", 3);

    println!("{map:?}");

    // accessing elements
    let x = map.get("Blue").unwrap();
    println!("Value of Blue is: {x}");

    let x = map.get("Green").copied().unwrap_or(0);
    println!("Value of Green is: {x}");

    // updating map
    map.entry("Green").or_insert(1);
    let score = map.entry("Yellow").or_insert(4);
    *score += 1;

    for (key, value) in &map{
        println!("{key}:{value}")
    }

    println!("{map:?}");
}
