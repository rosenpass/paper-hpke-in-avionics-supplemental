use crypto_agility_crypto_api::server::hpke::*;
use crypto_agility_crypto_api::server::Endpoint;

#[cfg(feature = "empty")]
use memory_bench::empty::*;
#[cfg(feature = "hdkf256")]
use memory_bench::hdkf_sha256::*;
#[cfg(feature = "hdkf512")]
use memory_bench::hdkf_sha512::*;
#[cfg(feature = "xyber1024dilithium")]
use memory_bench::xyber1024dilithium::*;
#[cfg(feature = "xyber1024dilithium_oqs")]
use memory_bench::xyber1024dilithium_oqs::*;
#[cfg(feature = "xyber768")]
use memory_bench::xyber768::*;
#[cfg(feature = "xyber768_oqs")]
use memory_bench::xyber768_oqs::*;
#[cfg(feature = "xyber768_oqs_ghp")]
use memory_bench::xyber768_oqs_ghp::*;
#[cfg(feature = "xyber768dilithium")]
use memory_bench::xyber768dilithium::*;
#[cfg(feature = "xyber768dilithium_oqs")]
use memory_bench::xyber768dilithium_oqs::*;
#[cfg(feature = "xyber768dilithium_oqs_ghp")]
use memory_bench::xyber768dilithium_oqs_ghp::*;

// pub type KemType = memory_bench::xyber1024dilithium::KemType;
pub type AeadType = hpke::aead::ChaCha20Poly1305;

pub type KdfType = hpke::kdf::HkdfSha384;

pub const ADDITIONAL_DATA: &[u8] = b"Foo";
pub const MESSAGE: &[u8] = b"Hello, World!";

fn main() {
    let info = [1, 2, 3, 4];
    // let endpoint = HpkeEndpoint::<KemType, AeadType, KdfType>::new(&info);
    let endpoint: HpkeEndpoint<KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, SECRET_KEY, PUBLIC_KEY).unwrap();
    // println!("Public: {:?}", endpoint.get_public_key());
    // println!("Private: {:?}", endpoint.get_private_key());

    #[cfg(feature = "seal")]
    seal(endpoint);
    #[cfg(feature = "open")]
    open(endpoint);
}

#[allow(dead_code)]
fn seal(endpoint: HpkeEndpoint<KemType, AeadType, KdfType>) {
    let public_sealion_key = endpoint.get_public_key();
    let sealed_message: EncryptedMessage<KemType> = endpoint
        .seal(ADDITIONAL_DATA, &public_sealion_key, MESSAGE)
        .unwrap()
        .as_slice()
        .try_into()
        .unwrap();
    // let msg: Vec<u8> = sealed_message.clone().into();
    // println!("Message: {msg:?}");
    drop(sealed_message);
}

#[allow(dead_code)]
fn open(endpoint: HpkeEndpoint<KemType, AeadType, KdfType>) {
    let public_key = endpoint.get_public_key();
    let sealed_message_buf: Vec<_> = ENCRYPTED_MESSAGE.into();
    let opened_message = endpoint
        .open(ADDITIONAL_DATA, &public_key, &sealed_message_buf)
        .unwrap();
    drop(opened_message);
}
