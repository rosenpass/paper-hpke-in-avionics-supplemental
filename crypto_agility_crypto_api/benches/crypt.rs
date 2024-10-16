use std::{vec::Vec, any::type_name};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crypto_agility_crypto_api::server::{hpke::HpkeEndpoint, Endpoint};
use memory_bench::{dilithium, empty, hdkf_sha256, kyber768};

pub type AeadType = hpke::aead::ChaCha20Poly1305;

pub type KdfType = hpke::kdf::HkdfSha384;

pub fn benchmark_kem<Kem: hpke::Kem>(c: &mut Criterion, info: &[u8], ad: &[u8], pt: &[u8]) {
    use criterion::black_box as bb;

    let endpoint: HpkeEndpoint<Kem, AeadType, KdfType> = HpkeEndpoint::new(info);
    let pk = endpoint.get_public_key();

    let black_box_seal = || -> Vec<u8> {
        black_box(&endpoint)
            .seal(bb(ad), bb(&pk), bb(pt))
            .unwrap()
    };
    let black_box_open = |ct: Vec<u8>| {
        bb(&endpoint)
            .open(bb(ad), bb(&pk), bb(&ct))
            .unwrap()
    };

    c.bench_function(&format!("{} seal", type_name::<Kem>()), |b| {
        b.iter(black_box_seal)
    });

    c.bench_function(&format!("{} open", type_name::<Kem>()), |b| {
        b.iter_batched(
            black_box_seal,
            black_box_open,
            criterion::BatchSize::SmallInput)
    });
}

macro_rules! benchmark_these_kems {
    ($criterion:expr, $info:expr, $ad:expr, $pt:expr, []) => {};
    ($criterion:expr, $info:expr, $ad:expr, $pt:expr, [$($kem:ty),+$(,)?]) => {{
        let (c, info, ad, pt) = ($criterion, $info, $ad, $pt);
        $(benchmark_kem::<$kem>(c, info, ad, pt));+
    }};
}

pub fn benchmark_main(c: &mut Criterion) {
    let info = b"\x01\x02\x03\x04";
    let ad = b"Foo";
    let pt = b"Hello, World!";

    benchmark_these_kems!(c, info, ad, pt, [
        // Controls
        memory_bench::empty_cipher::EmptyKem,
        hpke::kem::X25519HkdfSha256,
        hpke::kem::X448HkdfSha512,
        // KEMs introduced in the original preprint
        // These are pure-rust implementations of kyber and dilithium
        // from before the NIST standards where published.
        // We keep them as a reference to see if the performance
        // characteristics of our KEMs changed when switching
        // to the OQS variants of ML-KEM and ML-DSA (which are the
        // names for kyber and dilithium as standardized by NIST)
        hpke::kem::X25519Kyber768,
        hpke::kem::X25519Kyber768Dilithium,
        // ML-KEM/ML-DSA implementations as standardized, based
        // on liboqs implementations
        hpke::kem::xyber768_oqs::X25519Kyber768,
        hpke::kem::xyber768dilithium_oqs::X25519Kyber768Dilithium,
        // xyber768 variants using the canonical GHP combiner implemented
        // using ML-KEM/ML-DSA from OQS; mostly included out of pure
        // curiosity
        hpke::kem::xyber768_oqs_ghp::X25519Kyber768,
        hpke::kem::xyber768dilithium_oqs_ghp::X25519Kyber768Dilithium,
        // Variant of our combiner using x448 and the level-5 (high security)
        // variants of ML-DSA/ML-KEM
        hpke::kem::xyber1024dilithium::X448Kyber1024Dilithium,
        hpke::kem::xyber1024dilithium_oqs::X448Kyber1024Dilithium,
    ])
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = benchmark_main
}
criterion_main!(benches);
