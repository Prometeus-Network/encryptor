# Encryptor Service

Encryptor service is responsible for cryptographic functions in Prometheus, the service will operate using the JSON-RPC 2.0 protocol.
The service is an integral part of Data Validator node and Data Mart node and automatically set during standard nodes deploymet procedure.

Encryptor service supports the following encryption algorithms:

- RSA
- AES-128

It provides these functions:

- Public and private keys generation;
- Asymmetric data encryption and decryption;
- Symmectrtic data encryption and decryption.

Although Encryptor service designed to be automatically set during nodes deploymet procedure, it can also be installed as standalone service.
To start the service, you must install the Rust programming language. Link for installing Rust https://www.rust-lang.org/tools/install
Link for installing Rust https://www.rust-lang.org/tools/install

The service includes the following functionality:
1. Public and private key generation (RSA algorithm)
2. Asymmetric encryption and decryption (RSA algorithm)
3. Symmetric encryption and decryption (AES-128-CBC algorithm)

## Launch of the project

### With the installation of Rust
In terminal:
```
$ make start_rust_install
```

### If Rust is already installed
```
$ make start
```

## Method summary

```
key_generate - public and private key generation

rsa_public_key_encrypt - asymmetric public key encryption

rsa_private_key_decrypt - private key asymmetric decryption

aes_encrypt - symmetric encryption

aes_decrypt - symmetric decryption
```
