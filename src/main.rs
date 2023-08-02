use rusoto_core::Region;
use rusoto_sns::{Sns, SnsClient, PublishInput};

#[tokio::main]
async fn main() {
    let region = Region::default(); // Or specify your desired region
    let topic_arn = "arn:aws:sns:ap-south-1:481252995737:sample_topicc";

    let client = SnsClient::new(region);

    let message = "This is a test message";

    let input = PublishInput {
        topic_arn: Some(topic_arn.to_string()),
        message: message.to_string(),
        ..Default::default()
    };

    match client.publish(input).await {
        Ok(_) => println!("Message published successfully"),
        Err(err) => {
            println!("Error publishing message: {:?}", err);
            let topic_arn = "arn:aws:sns:ap-south-1:481252995737:sample_topic";
            // Handle the error by sending a message through SNS
            let error_message = format!("Error occurred: {:?}", err);
            let error_input = PublishInput {
                topic_arn: Some(topic_arn.to_string()),
                message: error_message,
                ..Default::default()
            };
            match client.publish(error_input).await {
                Ok(_) => println!("Error message sent successfully"),
                Err(e) => println!("Failed to send error message: {:?}", e),
            }
        }
    }
}
