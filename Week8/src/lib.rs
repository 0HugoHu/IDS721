//build a function that grabs the content from a wikipedia page
//Using wikipedia-rs crate
extern crate wikipedia;

use rust_bert::pipelines::sentiment::SentimentModel;

pub fn get_wiki_content(page: &str) -> String {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title((page).to_owned());
    page.get_content().unwrap()
}

//build a function that summarizes the content from a wikipedia page
pub fn sentiment_content(content: &str){
    let input = if content.len() > 100 {
        content.chars().take(100).collect::<String>() + "..."
    } else {
        content.to_string()
    };

    // Load a pre-trained sentiment analysis model
    let model = SentimentModel::new(Default::default()).unwrap();
    let input = input.split("\n").collect::<Vec<&str>>();
    

    let output = model.predict(&input);
    println!("Input text: {}", content);
    println!("Predicted sentiment: {:?}", output);
}
