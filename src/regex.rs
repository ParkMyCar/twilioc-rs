use regex::Regex;

pub fn valid_number_re(input: &str) -> bool {
    /// A valid phone number is a 10 digit number preceeded by "+1"
    /// e.g. +18559108712
    let re = Regex::new(r#"\+1[\d]{10}"#).unwrap();

    re.is_match(input)
}
