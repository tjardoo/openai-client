use std::env;
use openai_dive::v1::{api::Client, resources::fine_tune::CreateFineTuneParameters};

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CreateFineTuneParameters {
        training_file: "file-XXX".to_string(),
        validation_file: None,
        model: Some("curie".to_string()),
        n_epochs: None,
        batch_size: None,
        learning_rate_multiplier: None,
        prompt_loss_weight: None,
        compute_classification_metrics: None,
        classification_n_classes: None,
        classification_positive_class: None,
        classification_betas: None,
        suffix: None,
    };

    let result = client.fine_tunes().create(parameters).await.unwrap();

    println!("{:?}", result);
}
