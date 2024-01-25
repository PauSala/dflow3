use dotenv_codegen::dotenv;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
static SECRET_KEY: &'static str = dotenv!("SECRET_KEY");

pub fn encrypt_service(value: &str) -> String {
    let mc = new_magic_crypt!(SECRET_KEY, 256);
    mc.encrypt_str_to_base64(&value)
}

pub fn decrypt_service(base64: &str) -> String {
    let mc = new_magic_crypt!(SECRET_KEY, 256);
    mc.decrypt_base64_to_string(base64).expect("Decrypt failed!")
}
