// #[macro_use]
// extern crate std;

use std::collections::HashMap;

macro_rules! hash_map {
($($key:expr => $val:expr),*) => {
    //返回值是一个表达式，这里的大括号表示表达式
    {
        let mut map = HashMap::new();
        $(
            map.insert($key,$val);
        )*
        map
    }
};
}

pub fn main() {
    let map = hash_map! {
    "one"=> 1,
    "two"=>2,
    "three"=>3
    };
    println!("{:?}", map);
}
