// A trait is basically a sort of list of rules an instance needs to implement. Let's make one.
// pub trait Summary {
//     fn summarize(&self) -> String; // We don't make the function here. That must be done by each enum/struct independeltly.
// }
pub trait Summary {
    // Actually we can define something here! When the Summary trait is not implemented, but it is specfied, we can still use it.
    fn summarize(&self) -> String {
        String::from("Unknown")
    }
}

pub struct BlogArticle {
    pub title: String,
    pub author: String,
    pub content: String,
    pub views: u32,
}

impl Summary for BlogArticle {
    fn summarize(&self) -> String {
        format!("Blogpost\n\ttitle: \"{}\"\n\tviews: {}\n\tauthor: {}", self.title, self.views, self.author)
    }
}

pub struct MastodonPost {
    pub handle /*handle=username*/: String,
    pub publicaton_date: String,
    pub post_id: u32,
    pub content: String,
    pub hits: u32,
}

impl Summary for MastodonPost {
    fn summarize(&self) -> String {
        format!("Mastodon post #{} (posted on {})\n\tviews: {} times\n\tposter: @{}", self.post_id, self.publicaton_date, self.hits, self.handle)
    }
}



pub struct JJJJJJJ {
    pub hi: String
}

impl Summary for JJJJJJJ {} // See line #6


//see: https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax
// We're restricting only things that implement the Summary trait to this function. (trait bound)
fn debug_print<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}