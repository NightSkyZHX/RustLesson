fn compare_string(x:&str, y:&str) -> bool{
    let mut i=0;
    while i <x.len()&&i<y.len(){
        if x.as_bytes()[i] > y.as_bytes()[i]{
            return true;
        }
        i += 1;
    }
    return x.len()>y.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cmp() {
        let x = String::from("abc");
        let y = String::from("abc");
        println!("{}", compare_string(&x, &y));
    }
}
