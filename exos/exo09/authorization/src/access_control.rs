use casbin::prelude::*;

const CONFIG: &str = "accessControl/access_control.conf";
const POLICY: &str = "accessControl/access_control.csv";

///Centralized access control mechanism
pub async fn auth(subject: &str, ressource: &str) -> bool {
    todo!("Create an enforcer and apply the policy")
}
