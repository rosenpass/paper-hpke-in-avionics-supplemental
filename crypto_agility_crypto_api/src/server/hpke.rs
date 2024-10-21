use std::marker::PhantomData;

use anyhow::{anyhow, bail, Context, Result};
use hpke::aead::Aead;
use hpke::kdf::Kdf;
use hpke::kem::Kem;
use hpke::{Deserializable, OpModeR, OpModeS, Serializable};
use log::debug;

use super::Endpoint;

// type Kem = hpke::kem::X25519Kyber768Dilithium;
// type Aead = hpke::aead::ChaCha20Poly1305;
// type Kdf = hpke::kdf::HkdfSha384;
// type EncappedKey = <Kem as KemTrait>::EncappedKey;

pub struct HpkeEndpoint<
    KEM: Kem = hpke::kem::xyber768dilithium::X25519Kyber768Dilithium,
    AEAD: Aead = hpke::aead::ChaCha20Poly1305,
    KDF: Kdf = hpke::kdf::HkdfSha384,
> {
    info: Vec<u8>,
    _aead: PhantomData<AEAD>,
    _kdf: PhantomData<KDF>,
    public_key: KEM::PublicKey,
    secret_key: KEM::PrivateKey,
}

impl<KEM: Kem, AEAD: Aead, KDF: Kdf> HpkeEndpoint<KEM, AEAD, KDF> {
    pub fn new(info: &[u8]) -> Self {
        let (secret_key, public_key) = KEM::gen_keypair(&mut rand::thread_rng());
        Self {
            info: info.to_vec(),
            secret_key,
            public_key,
            _aead: PhantomData,
            _kdf: PhantomData,
        }
    }

    pub fn new_with_key(info: &[u8], secret_key: &[u8], public_key: &[u8]) -> Result<Self> {
        let secret_key = KEM::PrivateKey::from_bytes(secret_key)
            .map_err(|e| anyhow!("Failed parsing private key: {e:?}"))?;
        let public_key = KEM::PublicKey::from_bytes(public_key)
            .map_err(|e| anyhow!("Failed parsing public key: {e:?}"))?;
        Ok(Self {
            info: info.to_vec(),
            secret_key,
            public_key,
            _aead: PhantomData,
            _kdf: PhantomData,
        })
    }

    pub fn get_private_key(&self) -> Vec<u8> {
        self.secret_key.to_bytes().to_vec()
    }
}

impl<KEM: Kem, AEAD: Aead, KDF: Kdf> HpkeEndpoint<KEM, AEAD, KDF> {
    fn inner_seal(
        &self,
        additional_data: &[u8],
        message: &[u8],
        public_sealion_key: KEM::PublicKey,
    ) -> Result<(KEM::EncappedKey, Vec<u8>)> {
        let mode = OpModeS::Auth((self.secret_key.clone(), self.public_key.clone()));
        hpke::single_shot_seal::<AEAD, KDF, KEM, _>(
            &mode,
            &public_sealion_key,
            &self.info,
            message,
            additional_data,
            &mut rand::thread_rng(),
        )
        .map_err(|_| anyhow!("encyption failed"))
    }

    fn inner_open(
        &self,
        additional_data: &[u8],
        peer_public_key: KEM::PublicKey,
        sealed_message: &EncryptedMessage<KEM>,
    ) -> Result<Vec<u8>> {
        let mode = OpModeR::Auth(peer_public_key);
        hpke::single_shot_open::<AEAD, KDF, KEM>(
            &mode,
            &self.secret_key,
            &sealed_message.encapped_key,
            &self.info,
            &sealed_message.cipher_text,
            additional_data,
        )
        .map_err(|e| anyhow!("openion failed: {e:?}"))
    }
}

impl<KEM: Kem, AEAD: Aead, KDF: Kdf> Endpoint for HpkeEndpoint<KEM, AEAD, KDF> {
    fn get_public_key(&self) -> Vec<u8> {
        self.public_key.to_bytes().to_vec()
    }

    fn seal(
        &self,
        additional_data: &[u8],
        peer_public_key: &[u8],
        message: &[u8],
    ) -> Result<Vec<u8>> {
        debug!("sealing message with {} bytes", message.len());
        let peer_public_key = KEM::PublicKey::from_bytes(peer_public_key)
            .map_err(|e| anyhow!("public key parse failed {e:?}"))?;
        let (encapped_key, cipher_text) =
            self.inner_seal(additional_data, message, peer_public_key)?;
        Ok(EncryptedMessage::<KEM> {
            encapped_key,
            cipher_text,
        }
        .into())
    }

    fn open(
        &self,
        additional_data: &[u8],
        peer_public_key: &[u8],
        sealed_message: &[u8],
    ) -> Result<Vec<u8>> {
        let sealed_message = EncryptedMessage::<KEM>::try_from(sealed_message)?;
        debug!(
            "opening message with {} bytes",
            sealed_message.cipher_text.len()
        );
        let peer_public_key = KEM::PublicKey::from_bytes(peer_public_key)
            .map_err(|_| anyhow!("public key parsing failed"))?;
        let decryted_message =
            self.inner_open(additional_data, peer_public_key, &sealed_message)?;
        Ok(decryted_message)
    }

    fn public_key_size(&self) -> usize {
        KEM::PublicKey::size()
    }
}

pub struct EncryptedMessage<K: Kem> {
    pub encapped_key: K::EncappedKey,
    pub cipher_text: Vec<u8>,
}

impl<K: Kem> Clone for EncryptedMessage<K> {
    fn clone(&self) -> Self {
        EncryptedMessage {
            encapped_key: self.encapped_key.clone(),
            cipher_text: self.cipher_text.clone(),
        }
    }
}

impl<K: Kem> PartialEq for EncryptedMessage<K> {
    fn eq(&self, other: &Self) -> bool {
        if !self.cipher_text.eq(&other.cipher_text) {
            return false;
        }
        self.encapped_key
            .to_bytes()
            .eq(&other.encapped_key.to_bytes())
    }
}

impl<K: Kem> std::fmt::Debug for EncryptedMessage<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptedMessage")
            .field("encapped_key", &"<Secret>".to_string())
            .field("cipher_text", &self.cipher_text)
            .finish()
    }
}

impl<K: Kem> EncryptedMessage<K> {
    fn extract_encapped_key(buffer: &[u8]) -> Result<(K::EncappedKey, &[u8])> {
        let encapped_key_size = K::EncappedKey::size();
        if buffer.len() < encapped_key_size {
            bail!(
                "got remaining sealed message with length: {}, expected at least {encapped_key_size} for encapped key",
                buffer.len(),
            );
        }
        let (encapped_key_buf, rest) = buffer.split_at(encapped_key_size);
        let encapped_key = K::EncappedKey::from_bytes(encapped_key_buf)
            .ok()
            .context("encapped key parsing failed")?;
        Ok((encapped_key, rest))
    }
}

impl<K: Kem> TryFrom<&[u8]> for EncryptedMessage<K> {
    type Error = anyhow::Error;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let (encapped_key, cipher_text_buf) = Self::extract_encapped_key(buffer)?;
        let cipher_text = cipher_text_buf.to_vec();
        Ok(EncryptedMessage {
            encapped_key,
            cipher_text,
        })
    }
}

impl<K: Kem> From<EncryptedMessage<K>> for Vec<u8> {
    fn from(value: EncryptedMessage<K>) -> Self {
        let encapped_key_size = K::EncappedKey::size();
        let mut buffer = Vec::with_capacity(encapped_key_size + value.cipher_text.len());
        buffer.extend(value.encapped_key.to_bytes());
        buffer.extend(value.cipher_text);
        buffer
    }
}

#[cfg(test)]
#[macro_use]
mod tests {
    macro_rules! generate_hpke_tests {
        ($name:ident, $kem_type:ty) => {
            mod $name {
                use crate::server::hpke::*;

                type KemType = $kem_type;

                #[test]
                fn test_seal_open() {
                    let salt = [1, 2, 3, 4];
                    let endpoint: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);

                    let additional_data = b"Foo".to_vec();
                    let message = b"Hello, World!".to_vec();

                    let public_sealion_key = endpoint.get_public_key();
                    let sealed_message: EncryptedMessage<KemType> = endpoint
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap()
                        .as_slice()
                        .try_into()
                        .unwrap();

                    let public_key = endpoint.get_public_key();
                    let sealed_message_buf: Vec<_> = sealed_message.into();
                    let opened_message = endpoint
                        .open(&additional_data, &public_key, &sealed_message_buf)
                        .unwrap();

                    assert_eq!(opened_message, message);
                }

                #[test]
                fn test_sealion_between_endpoints() {
                    let salt = [1, 2, 3, 4];
                    let sender: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);
                    let receiver: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);

                    let additional_data = b"Foo".to_vec();
                    let message = b"Hello, Receiver!".to_vec();

                    let public_sealion_key = receiver.get_public_key();
                    let sealed_message: EncryptedMessage<KemType> = sender
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap()
                        .as_slice()
                        .try_into()
                        .unwrap();

                    let public_key = sender.get_public_key();
                    let sealed_message_buf: Vec<_> = sealed_message.into();
                    let opened_message = receiver
                        .open(&additional_data, &public_key, &sealed_message_buf)
                        .unwrap();

                    assert_eq!(opened_message, message);
                }

                #[test]
                fn test_empty_message() {
                    let salt = [1, 2, 3, 4];
                    let endpoint: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);

                    let additional_data = b"Foo".to_vec();
                    let message: Vec<u8> = Vec::new(); // Empty message

                    let public_sealion_key = endpoint.get_public_key();
                    let sealed_message: EncryptedMessage<KemType> = endpoint
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap()
                        .as_slice()
                        .try_into()
                        .unwrap();

                    let public_key = endpoint.get_public_key();
                    let sealed_message_buf: Vec<_> = sealed_message.into();
                    let opened_message = endpoint
                        .open(&additional_data, &public_key, &sealed_message_buf)
                        .unwrap();

                    assert_eq!(opened_message, message);
                }

                #[test]
                fn test_empty_salt() {
                    let salt = [];
                    let endpoint: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);

                    let additional_data = b"Foo".to_vec();
                    let message = b"Hello, World!".to_vec();

                    let public_sealion_key = endpoint.get_public_key();
                    let sealed_message: EncryptedMessage<KemType> = endpoint
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap()
                        .as_slice()
                        .try_into()
                        .unwrap();

                    let public_key = endpoint.get_public_key();
                    let sealed_message_buf: Vec<_> = sealed_message.into();
                    let opened_message = endpoint
                        .open(&additional_data, &public_key, &sealed_message_buf)
                        .unwrap();

                    assert_eq!(opened_message, message);
                }

                #[test]
                fn test_different_salts() {
                    let sender_salt = [1, 2, 3, 4];
                    let receiver_salt = [5, 2, 3, 4];
                    let sender = HpkeEndpoint::<KemType>::new(&sender_salt);
                    let receiver = HpkeEndpoint::<KemType>::new(&receiver_salt);

                    let additional_data = b"Foo".to_vec();
                    let message = b"Hello, Receiver!".to_vec();

                    let public_sealion_key = receiver.get_public_key();
                    let sealed_message = sender
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap();
                    let public_key = sender.get_public_key();
                    let opened_message =
                        receiver.open(&additional_data, &public_key, &sealed_message);

                    assert!(opened_message.is_err());
                }

                #[test]
                fn test_random_invalid_data() {
                    // Test with random invalid data (different sizes, unexpected bytes)
                    let salt = [1, 2, 3, 4];
                    let endpoint: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);

                    let additional_data = b"Foo".to_vec();
                    let message = b"Hello, World!".to_vec();

                    let public_sealion_key = endpoint.get_public_key();
                    let sealed_message = endpoint
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap();
                    let mut invalid_data = sealed_message;
                    // Introduce a random byte at the end of the ciphertext
                    invalid_data.push(rand::random::<u8>());

                    let public_key = endpoint.get_public_key();
                    let opened_message =
                        endpoint.open(&additional_data, &public_key, &invalid_data);

                    assert!(opened_message.is_err());
                }

                #[test]
                fn test_invalid_openion() {
                    let salt = [1, 2, 3, 4];
                    let endpoint: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);

                    let additional_data = b"Foo".to_vec();
                    let message = b"Hello, World!".to_vec();

                    let public_sealion_key = endpoint.get_public_key();
                    let sealed_message = endpoint
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap();

                    // Use a different endpoint for openion
                    let different_endpoint: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);
                    let public_key = endpoint.get_public_key();
                    let opened_message =
                        different_endpoint.open(&additional_data, &public_key, &sealed_message);

                    assert!(opened_message.is_err());
                }

                #[test]
                fn test_sealed_message_conversion() {
                    let salt = [1, 2, 3, 4];
                    let endpoint: HpkeEndpoint<KemType> = HpkeEndpoint::new(&salt);

                    let additional_data = b"Foo".to_vec();
                    let message = b"Hello, World!".to_vec();

                    let public_sealion_key = endpoint.get_public_key();
                    let sealed_message_bytes = endpoint
                        .seal(&additional_data, &public_sealion_key, &message)
                        .unwrap();
                    let sealed_message: EncryptedMessage<KemType> =
                        sealed_message_bytes.as_slice().try_into().unwrap();

                    let into_bytes_conversion = Vec::<u8>::from(sealed_message.clone());
                    let from_bytes_conversion =
                        EncryptedMessage::try_from(into_bytes_conversion.as_slice()).unwrap();
                    assert_eq!(from_bytes_conversion, sealed_message);
                }
            }
        };
    }

    generate_hpke_tests!(x25519_hkdf_sha256, hpke::kem::X25519HkdfSha256);
    generate_hpke_tests!(x448_hkdf_sha512, hpke::kem::X448HkdfSha512);
    generate_hpke_tests!(x25519_kyber768, hpke::kem::X25519Kyber768);
    generate_hpke_tests!(x25519_kyber768dilithium, hpke::kem::X25519Kyber768Dilithium);
    generate_hpke_tests!(
        x448_xyber1024dilithium_oqs,
        hpke::kem::xyber1024dilithium_oqs::X448Kyber1024Dilithium
    );
    generate_hpke_tests!(x25519_xyber768_oqs, hpke::kem::xyber768_oqs::X25519Kyber768);
    generate_hpke_tests!(
        x25519_xyber768_ghp,
        hpke::kem::xyber768_oqs_ghp::X25519Kyber768
    );
    generate_hpke_tests!(
        x25519_xyber768dilithium_oqs,
        hpke::kem::xyber768dilithium_oqs::X25519Kyber768Dilithium
    );
    generate_hpke_tests!(
        x25519_xyber768dilithium_oqs_ghp,
        hpke::kem::xyber768dilithium_oqs_ghp::X25519Kyber768Dilithium
    );
}
