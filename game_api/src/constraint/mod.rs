pub mod password;
pub mod username;

pub struct Constraint {
    pub regex: &'static str,
    pub error: &'static str,
}

pub const USERNAME: Constraint = Constraint {
    regex: r#"^[a-zA-Z]\w{0,19}$"#,
    error: "username must start with a letter, and contain up to 20 alpha-numeric characters",
};

pub const PASSWORD: Constraint = Constraint {
    regex: r#"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[*&^%$#@!]).{8,}$"#,
    error: "password must have at least one digit, one lowercase letter and one uppercase letter, while also having a length of 8 or more characters",
};
