//! ### 顺序查找
//! **改进方向**：只能减少系数(基本语句)，而数量级不会变
//!

use std::vec;

pub struct WorldModel {
    name: String,
}

impl WorldModel {
    pub fn new(name: String) -> Self {
        WorldModel { name }
    }

    pub fn say(&self, content: &str) {
        println!("Say: {}",content);
    }
}

///
///
pub fn sentry(data: &[usize], value: usize) -> usize {
    let mut index = data.len() - 1;
    while index > 0 && data[index] != value {
        println!("index: {}, value:{},f:{}", index, data[index], value);
        index = index - 1;
    }
    index
}

pub fn sentry2(data: vec::Vec<usize>, value: usize) -> usize {
    let mut index = data.len() - 1;
    while index > 0 && data[index] != value {
        println!("index: {}, value:{},f:{}", index, data[index], value);
        index = index - 1;
    }
    index
}

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

//    //!  - Inner line doc 只能出现在scope的最前面
//    /*!  - Inner block doc */
//    ///  - Outer line doc (exactly 3 slashes)
//    /**  - Outer block doc (exactly) 2 asterisks */
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sentry() {
        let data = vec![1, 5, 3, 2, 9, 8, 7];
        let world = WorldModel::new(String::from("Hello world !"));
        world.say("Hello !");
        //let index1 = sentry2(data, 8);
        // let index = sequeue::sentry(&data[0..7], 8);
        // print_type_of(data);
    }

    #[test]
    fn sentry_find_on_vec() {}
}
