/*A library that uses Hugging Face to Translate Text
*/
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs::File;
use std::io::Read;

//build a function that reads a file and returns a string
pub fn read_file(path: String) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//build a function that reads a file and returns an array of the lines of the file
pub fn read_file_array(path: String) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let array = contents.lines().map(|s| s.to_string()).collect();
    Ok(array)
}

//build a function that reads a file and translates it
pub fn translate_file(path: String, language: String) -> anyhow::Result<()> {
    // switch on the language
    let model = match language.as_str() {
        "spanish" => TranslationModelBuilder::new()
            .with_source_languages(vec![Language::English])
            .with_target_languages(vec![Language::Spanish])
            .create_model()?,
        "french" => TranslationModelBuilder::new()
            .with_source_languages(vec![Language::English])
            .with_target_languages(vec![Language::French])
            .create_model()?,
        "german" => TranslationModelBuilder::new()
            .with_source_languages(vec![Language::English])
            .with_target_languages(vec![Language::German])
            .create_model()?,
        
        "chinese" => TranslationModelBuilder::new()
            .with_source_languages(vec![Language::English])
            .with_target_languages(vec![Language::ChineseMandarin])
            .create_model()?,
        "italian" => TranslationModelBuilder::new()
            .with_source_languages(vec![Language::English])
            .with_target_languages(vec![Language::Italian])
            .create_model()?,
        "portuguese" => TranslationModelBuilder::new()
            .with_source_languages(vec![Language::English])
            .with_target_languages(vec![Language::Portuguese])
            .create_model()?,
        &_ => todo!(),
    };    

    let text = read_file_array(path)?;
    //pass in the text to the model
    // switch on the language
    let output = match language.as_str() {
        "spanish" => model.translate(&text, None, Language::Spanish)?,
        "french" => model.translate(&text, None, Language::French)?,
        "german" => model.translate(&text, None, Language::German)?,
        "chinese" => model.translate(&text, None, Language::ChineseMandarin)?,
        "italian" => model.translate(&text, None, Language::Italian)?,
        "portuguese" => model.translate(&text, None, Language::Portuguese)?,
        &_ => todo!(),
    };
    for sentence in output {
        println!("{}", sentence);
    }
    Ok(())
}
