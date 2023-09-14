use aead::{AeadCore, AeadInPlace};
use crypto_common::{BlockSizeUser, KeyInit, KeySizeUser, OutputSizeUser};
use digest::{FixedOutput, HashMarker};
use hpke::{
    aead::Aead,
    generic_array::{
        typenum::{self, U0},
        GenericArray,
    },
    kdf::Kdf,
    kem::SharedSecret,
    Deserializable, Kem, Serializable,
};

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde_impls", derive(serde::Serialize, serde::Deserialize))]
pub struct EmptyKey;

impl Deserializable for EmptyKey {
    fn from_bytes(_encoded: &[u8]) -> Result<Self, hpke::HpkeError> {
        Ok(EmptyKey)
    }
}
impl Serializable for EmptyKey {
    type OutputSize = U0;

    fn to_bytes(&self) -> hpke::generic_array::GenericArray<u8, Self::OutputSize> {
        GenericArray::<u8, U0>::default()
    }
}

pub struct EmptyKem;

impl Kem for EmptyKem {
    type PublicKey = EmptyKey;

    type PrivateKey = EmptyKey;

    fn sk_to_pk(_sk: &Self::PrivateKey) -> Self::PublicKey {
        EmptyKey
    }

    type EncappedKey = EmptyKey;

    type NSecret = typenum::U64;

    const KEM_ID: u16 = 0x32;

    fn derive_keypair(_ikm: &[u8]) -> (Self::PrivateKey, Self::PublicKey) {
        (EmptyKey, EmptyKey)
    }

    fn decap(
        _sk_recip: &Self::PrivateKey,
        _pk_sender_id: Option<&Self::PublicKey>,
        _encapped_key: &Self::EncappedKey,
    ) -> Result<hpke::kem::SharedSecret<Self>, hpke::HpkeError> {
        Ok(<SharedSecret<Self> as Default>::default())
    }

    fn encap<R: hpke::rand_core::CryptoRng + hpke::rand_core::RngCore>(
        _pk_recip: &Self::PublicKey,
        _sender_id_keypair: Option<(&Self::PrivateKey, &Self::PublicKey)>,
        _csprng: &mut R,
    ) -> Result<(hpke::kem::SharedSecret<Self>, Self::EncappedKey), hpke::HpkeError> {
        Ok((<SharedSecret<Self> as Default>::default(), EmptyKey))
    }
}

pub struct EmptyKdf;

#[derive(Clone, Debug, Default)]
pub struct EmptyHash;

impl BlockSizeUser for EmptyHash {
    type BlockSize = U0;
}

impl OutputSizeUser for EmptyHash {
    type OutputSize = U0;
}

impl digest::Update for EmptyHash {
    fn update(&mut self, _data: &[u8]) {}
}

impl HashMarker for EmptyHash {}

impl FixedOutput for EmptyHash {
    fn finalize_into(self, _out: &mut crypto_common::Output<Self>) {}
}

impl Kdf for EmptyKdf {
    type HashImpl = EmptyHash;

    const KDF_ID: u16 = 0x16;
}

pub struct EmptyAead;

#[derive(Clone, Debug, Default)]
pub struct EmptyAeadImpl;

impl AeadCore for EmptyAeadImpl {
    type NonceSize = U0;

    type TagSize = U0;

    type CiphertextOverhead = U0;
}

impl AeadInPlace for EmptyAeadImpl {
    fn encrypt_in_place_detached(
        &self,
        _nonce: &aead::Nonce<Self>,
        _associated_data: &[u8],
        _buffer: &mut [u8],
    ) -> aead::Result<aead::Tag<Self>> {
        Ok(aead::Tag::<Self>::default())
    }

    fn decrypt_in_place_detached(
        &self,
        _nonce: &aead::Nonce<Self>,
        _associated_data: &[u8],
        _buffer: &mut [u8],
        _tag: &aead::Tag<Self>,
    ) -> aead::Result<()> {
        Ok(())
    }
}

impl KeySizeUser for EmptyAeadImpl {
    type KeySize = U0;
}

impl KeyInit for EmptyAeadImpl {
    fn new(_key: &crypto_common::Key<Self>) -> Self {
        Self
    }
}

impl Aead for EmptyAead {
    type AeadImpl = EmptyAeadImpl;

    const AEAD_ID: u16 = 0x67;
}
