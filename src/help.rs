pub fn all() {

}

pub fn mode() {
    let text = "\n
    Unknown mode set. Available modes:\n
        local - Starts database in local mode. You cannot use database remotely.\n
        web - Starts database in web mode. You can connect to database from remote application or server.";
    println!("{}", text);
}