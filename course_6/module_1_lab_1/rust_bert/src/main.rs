extern crate anyhow;
use std::env;
use rust_bert::pipelines::ner::NERModel;

fn main() -> anyhow::Result<()> {
    let ner_model = NERModel::new(Default::default())?;
    let input: Vec<String> = env::args().skip(1).collect();

    if input.is_empty(){
        println!("No input");
        std::process::exit(1);
    }

    let output = ner_model.predict_full_entities(&input);
    for entity in output{
        println!("{entity:?}");
    }
 Ok(())
}
