use std::sync::OnceLock;

use lib_utils::envs::{get_env_b64u_as_u8s, get_env_parse};

pub fn auth_config() -> &'static AuthConfig {
    static INSTANCE: OnceLock<AuthConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AuthConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - while loading auth_conf - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct AuthConfig {
    // ---- Crypt.
    pub DEV_PWD_KEY: Vec<u8>,
    pub PWD_KEY: Vec<u8>,
    pub DEV_TOKEN_KEY: Vec<u8>,
    pub TOKEN_KEY: Vec<u8>,
    pub DEV_TOKEN_DURATION_SEC: f64,
    pub TOKEN_DURATION_SEC: f64,
}

impl AuthConfig {
    fn load_from_env() -> lib_utils::envs::Result<AuthConfig> {
        Ok(AuthConfig {
            // ---- Crypt.
            DEV_PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,
            PWD_KEY: get_env_b64u_as_u8s("PASSWORD_KEY")?,
            DEV_TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_KEY: get_env_b64u_as_u8s("TOKEN_KEY")?,
            DEV_TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
            TOKEN_DURATION_SEC: get_env_parse("TOKEN_DURATION_SEC")?,
        })
    }
}
