use uuid::Uuid;
fn main() {
    while 1 == 1{
        let id = Uuid::new_v4();
        let uppercase_id = id.to_string().to_uppercase();
        println!("{}", uppercase_id);
    }
}
