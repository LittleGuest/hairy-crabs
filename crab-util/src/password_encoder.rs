/// 密码编码器
pub struct PasswordEncoder;

impl PasswordEncoder {
    /// 加密密码
    pub fn encode(pwd: &str) -> String {
        // let digest = md5::compute(pwd);
        // format!("{:x}", digest)
        pwd.to_string()
    }

    /// 校验密码
    pub fn verify(pwd: &str, raw_pwd: &str) -> bool {
        if pwd.eq(raw_pwd) {
            return true;
        }
        let pwd = PasswordEncoder::encode(pwd);
        pwd.eq(&raw_pwd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("123456");
        println!("{}", s);
        assert_eq!(
            PasswordEncoder::encode("123456"),
            PasswordEncoder::encode("123456")
        )
    }

    #[test]
    fn test_verify() {
        let password = "12345";
        let raw_password = "12345";

        assert!(PasswordEncoder::verify(password, raw_password));

        let encode_password = PasswordEncoder::encode(password);
        assert!(PasswordEncoder::verify(&encode_password, raw_password));
    }
}
