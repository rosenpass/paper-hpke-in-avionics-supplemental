use crypto_agility_crypto_api::server::hpke::*;
use crypto_agility_crypto_api::server::Endpoint;

// use crypto_test::empty::*;
// use crypto_test::hdkf_sha256::*;
// use crypto_test::kyber768::*;
use crypto_test::dilithium::*;

pub type AeadType = hpke::aead::ChaCha20Poly1305;
// type AeadType = empty_cipher::EmptyAead;

pub type KdfType = hpke::kdf::HkdfSha384;
// type KdfType = empty_cipher::EmptyKdf;

fn main() {
    let info = [1, 2, 3, 4];
    let endpoint: HpkeEndpoint<KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, SECRET_KEY, PUBLIC_KEY).unwrap();
    // encrypt(endpoint)
    decrypt(endpoint)
}

#[allow(dead_code)]
fn old_main() {
    let info = [1, 2, 3, 4];
    let endpoint: HpkeEndpoint<KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, SECRET_KEY, PUBLIC_KEY).unwrap();

    let additional_data = b"Foo".to_vec();
    let message = b"Hello, World!".to_vec();

    let public_encryption_key = endpoint.get_public_key();
    let encrypted_message: EncryptedMessage<KemType> = endpoint
        .seal(&additional_data, &public_encryption_key, &message)
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();

    let public_key = endpoint.get_public_key();
    let encrypted_message_buf: Vec<_> = encrypted_message.into();
    let decrypted_message = endpoint
        .open(&additional_data, &public_key, &encrypted_message_buf)
        .unwrap();

    assert_eq!(decrypted_message, message);
}

#[allow(dead_code)]
fn encrypt(endpoint: HpkeEndpoint<KemType, AeadType, KdfType>) {
    let additional_data = b"Foo".to_vec();
    let message = b"Hello, World!".to_vec();

    let public_encryption_key = endpoint.get_public_key();
    let encrypted_message: EncryptedMessage<KemType> = endpoint
        .seal(&additional_data, &public_encryption_key, &message)
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();
    drop(encrypted_message);
}

#[allow(dead_code)]
fn decrypt(endpoint: HpkeEndpoint<KemType, AeadType, KdfType>) {
    let additional_data = b"Foo".to_vec();

    let public_key = endpoint.get_public_key();
    let encrypted_message_buf: Vec<_> = ENCRYPTED_MESSAGE.into();
    let decrypted_message = endpoint
        .open(&additional_data, &public_key, &encrypted_message_buf)
        .unwrap();
    drop(decrypted_message);
}
