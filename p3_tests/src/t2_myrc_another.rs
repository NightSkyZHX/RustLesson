use std::cell::RefCell;
use std::ops::Deref;

struct MyRcRef<T> {
    obj: T,
    cnt: i32,
}

impl<T> MyRcRef<T> {
    fn new(obj: T) -> Self {
        MyRcRef { obj, cnt: 1 }
    }
    fn change_cnt(&mut self, delta: i32) {
        self.cnt += delta;
    }
    fn get_cnt(&self) -> i32 {
        self.cnt
    }
}

pub struct MyRc<T> {
    ptr: RefCell<MyRcRef<T>>,
}

impl<T> MyRc<T> {
    pub fn new(obj: T) -> Self {
        Self {
            ptr: RefCell::new(MyRcRef::new(obj)),
        }
    }
    pub fn strong_count(&self) -> i32 {
        self.ptr.borrow().get_cnt()
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        self.ptr.borrow_mut().change_cnt(1);
        MyRc {
            ptr: RefCell::new()
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.ptr.borrow().obj
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        self.ptr.borrow_mut().change_cnt(-1);
        if self.ptr.borrow().get_cnt() == 0 {
            drop(self.ptr.borrow_mut());
        }
    }
}

impl<T> std::fmt::Display for MyRc<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyRc({})", self.ptr.borrow().obj)
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
