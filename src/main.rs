use api::API;

mod api;
mod endpoints;

fn main() {
    // TODO: --email --password
    let mut args = std::env::args();

    // Skip the first argument, it's the executable name.
    args.next();

    // Collect Email and Password.
    let email = args
        .next()
        .expect("[ERROR] Missing Email! USAGE: ./simply-launch email@example.com password123");
    let password = args
        .next()
        .expect("[ERROR] Missing Password! USAGE: ./simply-launch email@example.com password123");

    // Login and launch.
    API::launch_game(API::login(email, password))
}
