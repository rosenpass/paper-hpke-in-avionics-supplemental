use crypto_agility_crypto_api::server::hpke::*;
use crypto_agility_crypto_api::server::Endpoint;

#[cfg(feature = "dilithium")]
use memory_bench::dilithium::*;
#[cfg(feature = "empty")]
use memory_bench::empty::*;
#[cfg(feature = "hdkf")]
use memory_bench::hdkf_sha256::*;
#[cfg(feature = "kyber")]
use memory_bench::kyber768::*;

pub type AeadType = hpke::aead::ChaCha20Poly1305;

pub type KdfType = hpke::kdf::HkdfSha384;

fn main() {
    let info = [1, 2, 3, 4];
    let endpoint: HpkeEndpoint<KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, SECRET_KEY, PUBLIC_KEY).unwrap();
    #[cfg(feature = "seal")]
    seal(endpoint);
    #[cfg(feature = "open")]
    open(endpoint);
}

#[allow(dead_code)]
fn seal(endpoint: HpkeEndpoint<KemType, AeadType, KdfType>) {
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
fn open(endpoint: HpkeEndpoint<KemType, AeadType, KdfType>) {
    let additional_data = b"Foo".to_vec();

    let public_key = endpoint.get_public_key();
    let encrypted_message_buf: Vec<_> = ENCRYPTED_MESSAGE.into();
    let decrypted_message = endpoint
        .open(&additional_data, &public_key, &encrypted_message_buf)
        .unwrap();
    drop(decrypted_message);
}
