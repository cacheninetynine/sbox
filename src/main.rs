use uuid::Uuid;
use std::time::Duration;
use std::thread;

fn main() {
    println!("Please wait, generating best key ever");
    thread::sleep(Duration::from_millis(1000));
    let id = Uuid::new_v4();
    let uppercase_id = id.to_string().to_uppercase();
    println!("{}", uppercase_id);
}
