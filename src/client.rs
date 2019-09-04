use std::time::Duration;

use actix_rt::System;
use actix_web::client::{Client, ClientBuilder, Connector};
use futures::future::{lazy, Future};
use openssl::ssl::{SslConnector, SslMethod};
use serde_derive::{Deserialize, Serialize};

use crate::config::Config;
use crate::regex::valid_number_re;

pub struct TwilioClient {
    account_sid: String,
    client: Client,
    from: Option<String>,
}
impl TwilioClient {
    pub fn new(config: Config) -> TwilioClient {
        // Setup SSL Connection for the Actix Client
        let ssl_connector = SslConnector::builder(SslMethod::tls())
            .expect("Unable to build SSL connector!")
            .build();
        let connector = Connector::new()
            .ssl(ssl_connector)
            .timeout(Duration::from_secs(5))
            .finish();

        // Get Twilio Keys to setup HTTP Basic Auth header
        let (account_sid, auth_token) = match config.keys {
            Some(keys) => (keys.account_sid.unwrap(), keys.auth_token.unwrap()),
            None => panic!("In order to create a Twilio Client you need to provide an account_sid, and auth_token with your config"),
        };

        // Create Actix Client with SSL and auth
        let client = ClientBuilder::default()
            .basic_auth(&account_sid, Some(auth_token.as_str()))
            .connector(connector)
            .finish();

        // Unwrap UserPrefs
        let from = match config.user_prefs {
            Some(prefs) => match prefs.from {
                Some(from) => {
                    assert!(
                        valid_number_re(from.as_str()),
                        "The default from number provided is not a valid number!"
                    );
                    Some(from)
                }
                None => None,
            },
            None => None,
        };

        TwilioClient {
            account_sid,
            client,
            from,
        }
    }

    pub fn send_sms(&self, to: String, body: String, from: Option<String>) {
        assert!(
            from.is_some() || self.from.is_some(),
            "You need to specify a default 'from' in your config, or you need to provide one as an argument!"
        );
        let from = from.unwrap_or(self.from.as_ref().unwrap().clone());
        let sms_form = SMSForm::new(to, from, body);
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            &self.account_sid
        );

        System::new("test")
            .block_on(lazy(|| {
                self.client
                    .post(url)
                    .send_form(&sms_form)
                    .map_err(|err| (println!("{:?}", err)))
                    .and_then(|response| {
                        println!("{:?}", response);
                        Ok(())
                    })
            }))
            .unwrap();
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct SMSForm {
    To: String,
    From: String,
    Body: String,
}
impl SMSForm {
    pub fn new(to: String, from: String, body: String) -> SMSForm {
        SMSForm {
            To: to,
            From: from,
            Body: body,
        }
    }
}
