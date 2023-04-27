use api::API;

mod api;
mod endpoints;

fn main() {
    // Collect the arguments.
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

    // If there's any remaining arguments after password, assume its the user overriding the
    // language.
    // Otherwise default to "en" / English.
    let language = args.next().unwrap_or_else(|| "en".to_owned());

    println!("Logging in and launching, runtime language: {language}");
    println!("If you wish to use a different language, add the language identifier after password.");
    println!("For example: ./simply-launch email@example.com password123 en");

    // Login and launch.
    API::launch_game(API::login(email, password), language)
}
