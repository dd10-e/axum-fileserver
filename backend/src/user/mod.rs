use crate::auth::{generate_jwt, Claims};

pub(crate) mod signup;
pub(crate) mod list;
pub(crate) mod get;
pub(crate) mod delete;
pub(crate) mod search;
pub(crate) mod update;
pub(crate) mod login;
pub(crate) mod authentication;

pub async fn protected_test(
    claims: Claims,
) -> String  {
    format!("OK")
}