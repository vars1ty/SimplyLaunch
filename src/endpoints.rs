/// Login Auth back-end.
/// ## Usage
/// Uses JSON and takes these parameters:
/// `username`: String
/// `password`: String
/// `launcher_version`: String
/// `launcher_platform`: String, should be set to `desktop`
/// `client_os`: String, normal acknowledged values are `Windows` and `Mac OS`
/// `browser_family`: String, should be set to `Electron`
/// `deviceId`: String, leave empty
/// ## Output
/// A JSON Response, relevant data being `launcher_hash`, `account_id`, `metrics_url` and
/// `metrics_groups`.
pub const AUTH_LOGIN: &str = "https://launcher-proxy.starstable.com/launcher/0.1/auth";

/// Queue Create back-end.
/// ## Usage
/// Uses JSON and takes one parameter, which is `launcher_hash` retrieved via `AUTH_LOGIN`.
/// ## Output
/// A JSON Response, relevant data being:
/// `success`: bool
/// `passedTheQueue`: bool,
/// `queueToken`: String
pub const AUTH_QUEUE_CREATE: &str = "https://launcher-proxy.starstable.com/launcher/login-queue/";

/// Launcher Proxy URL.
pub const LAUNCHER_PROXY: &str = "https://launcher-proxy.starstable.com";

/// User Agent retrieved via `navigator.userAgent`.
pub const USER_AGENT: &str = "'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) StarStableOnline/2.18.0 Chrome/104.0.5112.124 Electron/20.3.8 Safari/537.36'";
