
/// Docuemnt the item present after it
///
///
///
///
///
///

mod search {

    #[path="sequeue.rs"]
    pub mod sequeue;
}

// use search::sequeue;

//mod collection;

// use super::collection;
//


fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}


fn main() {
//    collection::generate_array::<u32>(2);
        let data = [1, 5, 3, 2, 9, 8, 7];
        // search::sequeue::sentry2(data, 8);
        let data = vec![1,2,3,7,66];
        print_type_of(data);

        // println!("data: {}", data[0])
        // search::sequeue::sentry2(&data[0..7], 8);
}
