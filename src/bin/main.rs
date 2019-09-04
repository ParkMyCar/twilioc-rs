use twilioc_rs::client::TwilioClient;
use twilioc_rs::config::read_config_from_file;

fn main() {
    let config = read_config_from_file("api.toml.key").unwrap();
    let twilio = TwilioClient::new(config);

    twilio.send_sms(
        String::from("+12034480597"),
        String::from("This is a test"),
        None,
    );
}
