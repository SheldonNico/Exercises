use kilo::Kilo;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    Kilo::new().run().unwrap();
}
