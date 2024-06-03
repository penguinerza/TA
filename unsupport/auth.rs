// use prusti_contracts::*;

struct User {
    username: String,
    password_hash: u64,
}

impl User {
    // #[pure]
    fn new(username: String, password_hash: u64) -> Self {
        User {
            username,
            password_hash,
        }
    }

    // #[pure]
    fn password_matches(&self, password: &str) -> bool {
        let hashed_password = hash_password(password);
        self.password_hash == hashed_password
    }
}

// #[pure]
fn hash_password(password: &str) -> u64 {
    let mut hash: u64 = 0;
    for c in password.bytes() { // iterator
        hash = hash.wrapping_mul(31).wrapping_add(c as u64);
    }
    hash
}

struct AuthenticationSystem {
    users: Vec<User>,
}

impl AuthenticationSystem {
    fn new() -> Self {
        AuthenticationSystem { users: Vec::new() }
    }

    // #[requires(username.len() > 0)]
    // #[requires(password.len() > 0)]
    fn register_user(&mut self, username: String, password: &str) {
        let password_hash = hash_password(password);
        let new_user = User::new(username, password_hash);
        self.users.push(new_user);
    }

    // #[requires(username.len() > 0)]
    // #[requires(password.len() > 0)]
    fn authenticate_user(&self, username: &str, password: &str) -> bool {
        for user in &self.users { // iterator
            if user.username == username && user.password_matches(password) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut auth_system = AuthenticationSystem::new();

    auth_system.register_user(String::from("alice"), "password123");
    auth_system.register_user(String::from("bob"), "qwerty");

    let alice_authenticated = auth_system.authenticate_user("alice", "password123");
    let bob_authenticated = auth_system.authenticate_user("bob", "password123");

    assert!(alice_authenticated, "Alice should be authenticated");
    assert!(!bob_authenticated, "Bob should not be authenticated");

}