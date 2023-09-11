use std::ops::Deref;

struct MyRcRef<T> {
    obj: T,
    cnt: i32,
}

impl<T> MyRcRef<T> {
    pub fn new(obj: T) -> Self {
        MyRcRef { obj, cnt: 1 }
    }
    fn change_cnt(&mut self, delta: i32) {
        self.cnt += delta;
    }
    pub fn get_cnt(&self) -> i32 {
        self.cnt
    }
}

pub struct MyRc<T> {
    ptr: *mut MyRcRef<T>,
}

impl<T> MyRc<T> {
    pub fn new(obj: T) -> Self {
        Self {
            ptr: Box::into_raw(Box::new(MyRcRef::new(obj))),
        }
    }
    pub fn strong_count(&self) -> i32 {
        unsafe { (*self.ptr).get_cnt() }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).change_cnt(1);
        }
        MyRc { ptr: self.ptr }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.ptr).obj }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).change_cnt(-1);
            if (*self.ptr).get_cnt() == 0 {
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}

impl<T> std::fmt::Display for MyRc<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", **self)
    }
}

pub fn main() {
    let five = MyRc::new(5);
    let five1 = five.clone();
    let five2 = five1.clone();
    println!("five1 = {}", five1);
    println!("five2 = {}", five2);
    println!("strong_cnt of *five* = {}", MyRc::strong_count(&five1));
    drop(five1);
    println!("strong_cnt of *five* = {}", MyRc::strong_count(&five2));
}
