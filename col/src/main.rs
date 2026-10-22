
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
   let news = NewsArticle{
     author: String::from("abhinayjangde"),
     content: String::from("PHP found RCE bug"),
     headline: String::from("Hacked PHP"),
     location: String::from("India")
   };

   let post = SocialPost{
    username: String::from("abhinayjangde"),
    content: String::from("I am learning Rust"),
    reply: false,
    repost: true
   };

   news_aggregator(post);
   news_aggregator(news);
}

fn news_aggregator(source: impl Summary){
    println!("{}", source.summarize());
}