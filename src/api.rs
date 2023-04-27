use crate::endpoints;

/// Implementation of the `launcher-proxy` API.
pub struct API {
    /// User Email.
    email: String,

    /// User Password.
    password: String,
}

/// Auth response.
#[derive(Debug)]
pub struct AuthResponse {
    /// The users Account ID.
    user_id: String,

    /// The users Launcher Hash, aka. Auth Token.
    launcher_hash: String,

    /// The queue token for the user.
    queue_token: String,
}

impl API {
    pub fn launch_game(auth_response: AuthResponse) {
        if !std::path::Path::new("./SSOClient.exe").exists() {
            panic!("[ERROR] No 'SSOClient.exe' is present. Make sure that this executable is located within the 'client' directory!")
        }

        let current_directory = std::env::current_dir().expect("[ERROR] Failed retrieving path!");
        let current_directory = current_directory.to_string_lossy();
        // Save me from this horrible way of passing arguments.
        // Thank you Johan.
        std::process::Command::new("./SSOClient.exe")
            .args([
                "-Language=en",
                &format!("-NetworkUserId={}", auth_response.user_id),
                "-MetricsServer=https://metrics.starstable.com/metric/v1/metrics",
                "-MetricsGroup=[1]",
                &format!("-LoginQueueToken={}", auth_response.queue_token),
                &format!("-NetworkLauncherHash={}", auth_response.launcher_hash),
                &format!("-ProjectUserDataPath=\"{current_directory}\""),
                &format!("-NetworkLauncherServer={}", endpoints::LAUNCHER_PROXY),
            ])
            .spawn()
            .expect("[ERROR] Couldn't start 'SSOClient.exe'!");
    }

    /// Attempts to login.
    /// ## Returns
    /// A structure containing the User ID and Launcher Hash.
    /// Panics if the API `success` value is false, or if there's an error with retrieving/sending
    /// data.
    pub fn login(email: String, password: String) -> AuthResponse {
        let json = json::object! {
            username: email,
            password: password,
            launcher_version: "2.18.0", // Update when the launcher updates.
            launcher_platform: "desktop",
            client_os: "Windows",
            browser_family: "Electron",
            deviceId: "0"
        };

        println!("Grabbing Launcher Hash and User ID...");
        let client = reqwest::blocking::Client::new();
        let response = json::parse(
            &client
                .post(endpoints::AUTH_LOGIN)
                .body(json.dump())
                .header("Content-Type", "application/json")
                .header("User-Agent", endpoints::USER_AGENT)
                .send()
                .expect("[ERROR] Couldn't send POST request!")
                .text()
                .expect("[ERROR] Couldn't get raw text response from the request!"),
        )
        .expect("[ERROR] Couldn't parse response as JSON!");

        if response["success"]
            .as_bool()
            .expect("[ERROR] No 'success' key is present?")
        {
            // Success, get the queue_token and return.
            let launcher_hash = response["launcher_hash"]
                .as_str()
                .expect("[ERROR] Couldn't find 'launcher_hash'!")
                .to_owned();
            AuthResponse {
                user_id: response["account_id"].to_string(),
                launcher_hash: launcher_hash.to_owned(),
                queue_token: Self::get_queue_token(launcher_hash, client),
            }
        } else {
            panic!("[ERROR] Couldn't login, response: {response}")
        }
    }

    /// Attempts to get the queue token.
    /// ## Returns
    /// A `String` containing the token.
    /// Panics if the API `success` value is `false`, or there's an error with retrieving/sending
    /// data.
    fn get_queue_token(launcher_hash: String, client: reqwest::blocking::Client) -> String {
        let json = json::object! {
            launcher_hash: launcher_hash
        };

        println!("Grabbing Queue Token...");
        let response = json::parse(
            &client
                .post(endpoints::AUTH_QUEUE_CREATE)
                .body(json.dump())
                .header("Content-Type", "application/json")
                .header("User-Agent", endpoints::USER_AGENT)
                .send()
                .expect("[ERROR] Couldn't send POST request!")
                .text()
                .expect("[ERROR] Couldn't get raw text response from the request!"),
        )
        .expect("[ERROR] Couldn't parse response as JSON!");

        if response["success"]
            .as_bool()
            .expect("[ERROR] No 'success' key is present?")
        {
            // Success, get the token.
            response["queueToken"]
                .as_str()
                .expect("[ERROR] Couldn't find 'queueToken'!")
                .to_owned()
        } else {
            panic!("[ERROR] Couldn't get queue token, response: {response}")
        }
    }
}
