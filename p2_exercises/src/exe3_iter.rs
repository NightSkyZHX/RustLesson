fn main(){
    let ori = vec!['a', 'b', 'c', 'd', 'e'];
    let iter = ori.iter().map(|x| (*x as u8 + 1) as char);
    let res: Vec<char> = iter.collect();
    println!("{:?}", res);
}
