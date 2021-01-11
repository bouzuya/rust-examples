use lettre::{
    smtp::{error::Error, response::Response},
    ClientSecurity, SendableEmail, SmtpClient, SmtpTransport, Transport,
};
use lettre_email::Email;

fn main() {
    let email: Email = Email::builder()
        .to(("m@bouzuya.net", "bouzuya"))
        .from("m@bouzuya.net")
        .subject("SUBJECT")
        .text("Hello lettre.")
        .build()
        .unwrap();
    let sendable_email: SendableEmail = email.into();

    // let smtp_client = SmtpClient::new_unencrypted_localhost().unwrap();
    let smtp_client: SmtpClient =
        SmtpClient::new(("localhost", 3025), ClientSecurity::None).unwrap();
    let mut smtp_transport: SmtpTransport = smtp_client.transport();
    let result: Result<Response, Error> = smtp_transport.send(sendable_email);
    match result {
        Ok(resp) => println!("Ok: {:?}", resp),
        Err(err) => println!("Err: {:?}", err),
    }
}
