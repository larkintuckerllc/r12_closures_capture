fn main() {
    let x = 4;
    // fn equal_to_x(z: i32) -> bool { z == x } // CANNOT CAPTURE
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
           println!("Got: {}", val); // 1 2 3
        }
        /*
        for val in v1_iter { // MOVED VALUE
            println!("Got: {}", val);
        }
        */
    }

    {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v1_iter_map = v1.iter().map(|x| x + 1); // CLOSURE NOT EXECUTED
        let v2: Vec<_> = v1_iter_map.collect();
        let v2_iter = v2.iter();
        for val in v2_iter {
           println!("Got: {}", val); // 2 3 4
        }
    }

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    for val in in_my_size {
        println!("Got: {}", val.style); // sneaker boot
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

struct Shoe {
    size: u32,
    style: String,
}
