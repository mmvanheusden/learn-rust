#![allow(non_snake_case)]

use crate::aggregrator::{BlogArticle, JJJJJJJ, MastodonPost, Summary};

mod GenericDataTypes;
mod GenericTypesInStructDefinitions;
mod GenericTypesInStructDefinitions_MethodDefinitions;
mod aggregrator;

/// Traits are awesome xD
fn main() {
    // Let's start with the inefficient code.
    let number_list = vec![34, 50, 25, 100, 65];            // Create a new list
    let mut largest = &number_list[0];                          // Take the first index
    for number in &number_list {                                // Loop through each number in the list
        if number > largest {                                         // If a number from the list is bigger, set largest to it.
            largest = number;                                          //This way, we end up with the biggest number from the list.
        }
    }
    println!("The largest number is {}", largest);

    // Do the same here but for a different list.
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // This is CODE REPETITION!!! bad
    /*
            "
               Although this code works, duplicating code is tedious and error-prone.
               We also have to remember to update the code in multiple places when we
               want to change it.
            "
     */


    // To eliminate this issue, we create a function that does exactly that, but for any list.

    let list_1 = vec![43,4,3,524,5,43,643,6435,32,43,43434,3245,32];
    let list_2 = vec![54,534,6,436,43,645,6,457,45,7,7,7,7,7,7,67,55435,543,543,53];

    let result = find_largest(&list_1);
    println!("The largest number is {}", result);

    let result = find_largest(&list_2);
    println!("The largest number is {}", result);


    GenericDataTypes::main();
    GenericTypesInStructDefinitions::main();
    GenericTypesInStructDefinitions_MethodDefinitions::main();


    let my_awesome_mastodon_post = MastodonPost {
        handle: "ElonMusk".to_string(),
        publicaton_date: "28 November 2024".to_string(),
        post_id: 37,
        content: "Hello world!".to_string(),
        hits: 0
    };
    // Now let's pretend we're some sweaty server dev that wants to lookup a specific Mastodon post. We do that with the aggregrator we made.
    println!("Summary of Mastodon post {}:\n{}",my_awesome_mastodon_post.post_id, my_awesome_mastodon_post.summarize());

    // Now the same but with a blog post
    let my_awesome_blog_post = BlogArticle {
        content: "Hello internet!".to_string(),
        views: 0,
        title: "Important message from me".to_string(),
        author: "Crab".to_string()
    };
    println!("Summary of blog post:\n{}",my_awesome_blog_post.summarize());


    let something_else = JJJJJJJ {hi: "hwllo threre".to_string()};
    println!("Summary for (whatever this is): {}", something_else.summarize());
}

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0]; // Take first index in the list

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}