use std::thread;
use std::time::Duration;

struct EmailMsg {
    from: String,
    to: String,
    subject: String,
    body: String
}
fn main() {
    let from = String::from("my.email@gmail.com");
    let to = String::from("my.to.email@gmail.com");
    let subject = String::from("Hi");
    let body = String::from("Hope you are doing well!");

    match send_email(EmailMsg { from, to, subject, body }) {
        Ok(_) => println!("Succesfully sent email"),
        Err(err) => println!("{}", err)
    }
}


fn send_email(email: EmailMsg) -> Result<(), Box<dyn std::error::Error>> {
    thread::sleep(Duration::new(5, 0));

    Ok(())
}



