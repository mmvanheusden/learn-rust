pub fn main() {
    // Again, using T we can pass multiple types into it
    let average_area_of_building = Area { width: 1400, height:4244 };
    let a4_paper = Area { width: 8.27, height: 11.7 };

    // We can't use different types in one. Either i32,i32 or f32,f32. not a combination of both.
    // let festival_ticket = Area { width: 42432, height: 45256.42 };


    // Let's use two generics to deal with this problem
    // We can mix 2 types now.
    let bike_size = Area2 {width: 15.3, height:123};

    // READ: https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions
}

// 2D area with a generic T
struct Area<T> {
    width: T,
    height: T,
}

// 2D area with two generics; T and U.
struct Area2<T, U> {
    width: T,
    height: U,
}
