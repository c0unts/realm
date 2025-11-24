pub fn get_users() -> Result<Dict> {} {
    use sysinfo::Users;

    let mut users = Users::new();
    users.refresh();
    for user in users.list() {
        println!("{user:?}");
    }
}
