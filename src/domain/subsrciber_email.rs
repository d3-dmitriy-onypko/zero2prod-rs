use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);
impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        let err_message = format!("{} is wrong email", s);
        validate_email(&s)
            .then_some(Self(s)).ok_or(err_message)
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert!(SubscriberEmail::parse(email).is_err());
    }
}
