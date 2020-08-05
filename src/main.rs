extern crate base64;
extern crate openssl;
extern crate dotenv;
use serde_json::{json};
use jsonrpc_core::*;
use jsonrpc_http_server::{ServerBuilder};
use encryptor::core::rsa::*;
use encryptor::core::aes::*;

fn main() {
	dotenv::dotenv().expect("Failed to read .env file");
	let host = dotenv::var("HOST").expect("HOST not found");
	let port = dotenv::var("PORT").expect("PORT not found");
	let localhost = host + ":" + &port;

	let mut io = IoHandler::new();
	io.add_method("key_generate", |_params: Params| {
		let key = RsaKey::new();
		Ok(json!({
			"public_key": key.public_key,
			"private_key": key.private_key
		}))
	});

	io.add_method("rsa_public_key_encrypt", |params: Params| {
		let parsed: RsaEncryptPublicKey = params.parse().unwrap();
		let result = RsaEncryptedPublicKey::new(parsed);
		Ok(json!({
			"content": result.content
		}))
	});

	io.add_method("rsa_private_key_decrypt", |params: Params| {
		let parsed: RsaDecryptPrivateKey = params.parse().unwrap();
		let result = RsaDecryptedPrivateKey::new(parsed);
		Ok(json!({
			"content": result.content
		}))
	});

	io.add_method("aes_decrypt", |params: Params| {
		let parsed: AesManager = params.parse().unwrap();
		let result = AesManager::decrypt(parsed);
		Ok(json!({
			"content": result.content
		}))
	});

	io.add_method("aes_encrypt", |params: Params| {
		let parsed: AesContent = params.parse().unwrap();
		let result = AesManager::encrypt(parsed);
		Ok(json!({
			"key": result.key,
			"iv": result.iv,
			"content": result.content
		}))
	});

	let server = ServerBuilder::new(io)
		.threads(3)
		.max_request_body_size(104857600)
		.start_http(&localhost.parse().unwrap())
		.unwrap();

	server.wait();
}
