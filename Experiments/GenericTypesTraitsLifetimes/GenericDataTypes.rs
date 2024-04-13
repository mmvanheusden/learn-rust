pub fn main() {
    // Now what if we want to get the largest char of an array of numbers AND chars?
    // We make separate functions for it.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // THIS IS INEFFICIENT!!!!! we can do better

    // Let's make use of a new function that uses generic types instead.

    let char_list = vec!['w', 'a', 'y', 'q','f'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let number_list = vec![2,55435,22432432,432423];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    // We can now pass different types into largest(), as long as the type implements the std::cmp::PartialOrd trait, it works.

}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            // Alpabetical
            largest = item;
        }
    }

    largest
}

// uses generic types
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // We can't compare this unless T implemented the std::cmp::PartialOrd trait.
        if item > largest {
            largest = item;
        }
    }

    largest
}