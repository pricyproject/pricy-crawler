use anyhow::Result;
use rake::*;

pub fn tagger(text: String, country: &str) -> Result<()> {
    // Find stop words for different countries: https://github.com/stopwords-iso

    let file_path = format!("./lp/stop_words/stopwords-{country}.txt");
    let sw = StopWords::from_file(file_path)?;

    let rake = Rake::new(sw);
    let keywords = rake.run(text.as_str());

    for (i, k) in keywords.iter().enumerate() {
        println!("{i}: {}  {}", k.keyword, k.score)
    }
    Ok(())
}
