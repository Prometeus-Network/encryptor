extern crate base64;
extern crate openssl;
extern crate pem;
use serde_derive::Deserialize;
use openssl::symm::{decrypt, encrypt, Cipher};
use rand::Rng;

// Encrypt content
#[derive(Deserialize)]
pub struct AesContent {
	pub content: Vec<u8>
}

#[derive(Deserialize)]
pub struct AesManager {
	pub key: String,
	pub iv: String,
	pub content: String
}

impl AesManager {
	pub fn encrypt(encrypt_request: AesContent) -> Self {
		let cipher = Cipher::aes_128_cbc();
    	let key = rand::thread_rng().gen::<[u8; 16]>();
		let iv = rand::thread_rng().gen::<[u8; 16]>();

		let ciphertext = encrypt(cipher, &key, Some(&iv), &encrypt_request.content).unwrap();
		
        let base64_encode_key = base64::encode(&key);
        let base64_encode_iv = base64::encode(&iv);
        let base64_ciphertext = base64::encode(&ciphertext);

		AesManager {
			key: base64_encode_key,
			iv: base64_encode_iv,
			content: base64_ciphertext
		}
	}

	pub fn decrypt(decrypt_request: AesManager) -> AesContent {
		let cipher = Cipher::aes_128_cbc();
		let key = base64::decode(&decrypt_request.key).unwrap();
		let iv = base64::decode(&decrypt_request.iv).unwrap();
		let content = base64::decode(&decrypt_request.content).unwrap();

    	let decrypttext = decrypt(cipher, &key, Some(&iv), &content).unwrap();
    	
    	AesContent {
    		content: decrypttext
    	}
	}
}