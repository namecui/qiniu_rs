use data_encoding::BASE64URL;
use ring::hmac;
use ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY;
use serde_json::{self, json};
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Clone)]
pub struct Qiniu {
     pub ak: String,
     pub sk: String,
     pub scope: String,
}

impl Qiniu { 
     pub fn new(ak: String, sk: String, scope: String) -> Qiniu {
        Qiniu { ak, sk, scope}
     }

    fn get_storage(&self) -> String {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 + 3600;
        let encode_json = json!({"scope": self.scope, "deadline": time});
        BASE64URL.encode(&serde_json::to_vec(&encode_json).unwrap())
    }

     pub fn auth_token(&self) -> String {
        let key = hmac::Key::new(HMAC_SHA1_FOR_LEGACY_USE_ONLY, self.sk.as_bytes());
        let self_base64 = self.get_storage();
        let singature = hmac::sign(&key, self_base64.as_bytes());
        let singature_base64 = data_encoding::BASE64URL.encode(singature.as_ref());
        format!("{}:{}:{}", self.ak, singature_base64, self_base64)
     }
}

#[cfg(test)]
mod tests {
     #[test]
     fn it_works() {
          assert_eq!(2 + 2, 4);
     }

}
