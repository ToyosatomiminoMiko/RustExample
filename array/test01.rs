use std::io;
use std::num::ParseIntError;
use std::usize;

/*
 *  用户输入以修改数组中的值
 *
 * 
 *  Copyright (C) 2023.03.16.23:46 ToyosatomiminoMiko
 *  
 */

fn stoi(s: &str) -> Result<u8, ParseIntError> {
    s.trim().parse::<u8>()
}

fn create() -> (u8, u8) {
    let mut input: String = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let v: Vec<&str> = input.trim().split(',').collect();
        match stoi(v[0]) {
            Ok(x0) => match stoi(v[1]) {
                Ok(y0) => {
                    break (x0, y0);
                }
                Err(_) => continue,
            },
            Err(_) => continue,
        };
    }
}

#[derive(Copy, Clone, Debug)]
struct Array {
    a: [u8; 5],
}

struct User<'a> {
    array: &'a mut Array,
}
impl<'a> User<'a> {
    fn set_value(&mut self, i: usize, v: u8) -> Array {
        self.array.a[i] = v;
        *self.array
    }
}

fn main() {
    let mut a1 = Array { a: [0; 5] };
    loop {
        let (x, y) = create();
        let mut u1 = User { array: &mut a1 };
        let mut a1 = u1.set_value(x as usize, y);
        println!("{:?}", a1);
        let (x, y) = create();
        let mut u2 = User { array: &mut a1 };
        let mut a1 = u2.set_value(x as usize, y);
        println!("{:?}", a1);
    }
}
