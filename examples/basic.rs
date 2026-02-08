use trad::{Translator, languages};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let translator = Translator::setup(None).await?;

    let text = "Hello world";
    let translated = translator.translate(text, languages::ENGLISH, languages::SPANISH).await?;

    println!("{text} -> {translated}");
    Ok(())
}
