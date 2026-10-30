use openai_dive::v1::api::Client;
use openai_dive::v1::resources::decision::{
    DecisionChoice, DecisionParametersBuilder, DecisionQuestion, DecisionScoreLevel,
};

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    let parameters = DecisionParametersBuilder::default()
        .model("gpt-6-luna")
        .input(
            "I was charged twice for my subscription this month. Please refund the extra charge.",
        )
        .questions(vec![
            DecisionQuestion::predicate("billing", "Is this message about billing?"),
            DecisionQuestion::choice(
                "tone",
                "What is the tone of this message?",
                vec![
                    DecisionChoice::new("angry", Some("An upset or hostile message")),
                    DecisionChoice::new("calm", Some("A neutral or polite message")),
                    DecisionChoice::new("excited", Some("An enthusiastic or eager message")),
                ],
            ),
            DecisionQuestion::score(
                "urgency",
                "How urgent is this message?",
                vec![
                    DecisionScoreLevel::new("low", Some("Can wait")),
                    DecisionScoreLevel::new("medium", Some("Needs attention this week")),
                    DecisionScoreLevel::new("high", Some("Needs attention today")),
                ],
            ),
        ])
        .build()
        .unwrap();

    let result = client.decisions().create(parameters).await.unwrap();

    println!("{result:#?}");
}
