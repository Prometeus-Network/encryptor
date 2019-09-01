extern crate base64;
extern crate openssl;
extern crate pem;
use openssl::rsa::{Rsa, Padding};
use serde_derive::Deserialize;

pub struct RsaKey {
	pub public_key: String,
	pub private_key: String,
}

// Encrypt by public key
#[derive(Deserialize)]
pub struct RsaEncryptPublicKey {
	pub public_key: String,
	pub content: Vec<u8>
}

pub struct RsaEncryptedPublicKey {
	pub content: String
}
// Encrypt by public key

// Decrypt by private key
#[derive(Deserialize)]
pub struct RsaDecryptPrivateKey {
	pub private_key: String,
	pub content: String
}

pub struct RsaDecryptedPrivateKey {
	pub content: Vec<u8>
}
// Decrypt by private key

impl RsaKey {
	pub fn new() -> Self {
		let keypair = Rsa::generate(2048).unwrap();
    	let pubkey_pem = keypair.public_key_to_pem_pkcs1().unwrap();
		let privkey_pem = keypair.private_key_to_pem().unwrap();

		let public_pem = base64::encode(&pubkey_pem);
		let private_pem = base64::encode(&privkey_pem);

    	RsaKey {
		    public_key: public_pem,
		    private_key: private_pem,
		}
	}
}

impl RsaEncryptedPublicKey {
	pub fn new(encrypt: RsaEncryptPublicKey) -> Self {
	    let mut pubkey_pem = base64::decode(&encrypt.public_key).unwrap();

		let pubkey = Rsa::public_key_from_pem_pkcs1(&mut pubkey_pem).unwrap();

		let mut encrypted = vec![0; pubkey.size() as usize];

		let _len = pubkey.public_encrypt(&encrypt.content, &mut encrypted, Padding::PKCS1).unwrap();

	    let result = base64::encode(&mut encrypted);

		RsaEncryptedPublicKey {
			content: result
		}
	}
}

impl RsaDecryptedPrivateKey {
	pub fn new(decrypt: RsaDecryptPrivateKey) -> Self {
    	let mut privkey_pem = base64::decode(&decrypt.private_key).unwrap();

    	let privkey = Rsa::private_key_from_pem(&mut privkey_pem).unwrap();

    	let content = base64::decode(&decrypt.content).unwrap();

    	let mut decrypted = vec![0; privkey.size() as usize];

    	let len = privkey.private_decrypt(&content, &mut decrypted, Padding::PKCS1).unwrap();
		let data = decrypted[..len].to_vec();
		RsaDecryptedPrivateKey {
			content: data
		}
	}
}