pub fn all() {
    let text = "".to_owned() +
    "Application configuration attributes:
    -m - Sets the startup mode. Attributes:
        local,
        web.
    -p - Sets the port of application. Used only if the -m attribute is set to web.
    -h or --help - Show this text.";
    println!("{}", text);
}

pub fn mode() {
    let text = "".to_owned() +
    "Unknown mode set. Available modes:
        local - Starts database in local mode. You cannot use database remotely.
        web - Starts database in web mode. You can connect to database from remote application or server.";
    println!("{}", text);
}