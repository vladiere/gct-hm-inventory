use modql::field::Fields;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Fields)]
#[allow(non_snake_case)]
pub struct AdminForInsert {
    pub ADM_ID: String,
    pub DETAIL_ID: i64,
    pub LOGIN_ID: i64,
    pub ADM_ROLE: String,
}

#[derive(Deserialize, Serialize, Fields)]
#[allow(non_snake_case)]
pub struct DetailForInsert {
    pub FIRST_NAME: String,
    pub MIDDLE_NAME: String,
    pub LAST_NAME: String,
    pub EMAIL_ADDRESS: String,
    pub PHONE_NUMBER: String,
}

#[derive(Deserialize, Serialize, Fields)]
#[allow(non_snake_case)]
pub struct LoginForInsert {
    pub USERNAME: String,
    pub PASSWORD: String,
    pub RECOVERY_CODE: String,
}
