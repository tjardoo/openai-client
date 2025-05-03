use openai_dive::v1::api::Client;
use openai_dive::v1::resources::image::{EditImageParametersBuilder, ImageQuality, ImageSize};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let client = Client::new_from_env();

    // Single File gpt-image-1
    let parameters = EditImageParametersBuilder::default()
        .prompt("Make this person smile with full teeth")
        .image(FileUpload::File("./images/person.png".to_string()))
        .model("gpt-image-1")
        .quality(ImageQuality::Low)
        .mime_type("image/png")
        .n(1u32)
        .size(ImageSize::Size1024X1024)
        .build()
        .unwrap();

    println!("gpt-image-1 with a Single File");
    let result = client.images().edit(parameters).await.unwrap();

    println!("{:#?}", result);

    // Multiple Files gpt-image-1
    let parameters = EditImageParametersBuilder::default()
        .prompt("Combine the person into the orginal image")
        .image(FileUpload::FileArray(vec![
            "./images/image_edit_original.png".to_string(),
            "./images/person.png".to_string(),
        ]))
        .model("gpt-image-1")
        .quality(ImageQuality::Low)
        .mime_type("image/png")
        .n(1u32)
        .size(ImageSize::Size1024X1024)
        .build()
        .unwrap();

    println!("gpt-image-1 with Multiple Files");
    let result = client.images().edit(parameters).await.unwrap();

    println!("{:#?}", result);
}
