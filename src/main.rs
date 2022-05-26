mod models;
mod services;

use services::service::user_service::get_user;

fn main() {
    println!("Hello, world!");
    let user = get_user();
    print!("{:?}", user);
}
