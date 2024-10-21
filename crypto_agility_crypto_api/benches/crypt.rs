use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crypto_agility_crypto_api::server::{hpke::HpkeEndpoint, Endpoint};
use memory_bench::{
    empty, hdkf_sha256, hdkf_sha512, kyber768, kyber768dilithium, xyber1024dilithium_oqs,
    xyber768_oqs, xyber768_oqs_ghp, xyber768dilithium_oqs, xyber768dilithium_oqs_ghp,
};

pub type AeadType = hpke::aead::ChaCha20Poly1305;

pub type KdfType = hpke::kdf::HkdfSha384;

pub const ADDITIONAL_DATA: &[u8] = b"Foo";
pub const MESSAGE: &[u8] = b"Hello, World!";
pub const INFO: &[u8] = &[1, 2, 3, 4];

pub fn empty(c: &mut Criterion) {
    let empty_endpoint: HpkeEndpoint<empty::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(INFO, empty::SECRET_KEY, empty::PUBLIC_KEY).unwrap();
    bench("empty", empty_endpoint, empty::PUBLIC_KEY, c);
}

pub fn hdkf_sha256(c: &mut Criterion) {
    let hdkf_sha256_endpoint: HpkeEndpoint<hdkf_sha256::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(INFO, hdkf_sha256::SECRET_KEY, hdkf_sha256::PUBLIC_KEY).unwrap();
    bench(
        "hdkf_sha256",
        hdkf_sha256_endpoint,
        hdkf_sha256::PUBLIC_KEY,
        c,
    );
}

pub fn hdkf_sha512(c: &mut Criterion) {
    let hdkf_sha512_endpoint: HpkeEndpoint<hdkf_sha512::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(INFO, hdkf_sha512::SECRET_KEY, hdkf_sha512::PUBLIC_KEY).unwrap();
    bench(
        "hdkf_sha512",
        hdkf_sha512_endpoint,
        hdkf_sha512::PUBLIC_KEY,
        c,
    );
}

pub fn kyber768(c: &mut Criterion) {
    let kyber768_endpoint: HpkeEndpoint<kyber768::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(INFO, kyber768::SECRET_KEY, kyber768::PUBLIC_KEY).unwrap();
    bench("kyber768", kyber768_endpoint, kyber768::PUBLIC_KEY, c);
}

pub fn kyber768dilithium(c: &mut Criterion) {
    let kyber768dilithium_endpoint: HpkeEndpoint<kyber768dilithium::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(
            INFO,
            kyber768dilithium::SECRET_KEY,
            kyber768dilithium::PUBLIC_KEY,
        )
        .unwrap();
    bench(
        "kyber768dilithium",
        kyber768dilithium_endpoint,
        kyber768dilithium::PUBLIC_KEY,
        c,
    );
}

pub fn xyber1024dilithium_oqs(c: &mut Criterion) {
    let xyber1024dilithium_oqs_endpoint: HpkeEndpoint<
        xyber1024dilithium_oqs::KemType,
        AeadType,
        KdfType,
    > = HpkeEndpoint::new_with_key(
        INFO,
        xyber1024dilithium_oqs::SECRET_KEY,
        xyber1024dilithium_oqs::PUBLIC_KEY,
    )
    .unwrap();
    bench(
        "xyber1024dilithium_oqs",
        xyber1024dilithium_oqs_endpoint,
        xyber1024dilithium_oqs::PUBLIC_KEY,
        c,
    );
}

pub fn xyber768_oqs(c: &mut Criterion) {
    let xyber768_oqs_endpoint: HpkeEndpoint<xyber768_oqs::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(INFO, xyber768_oqs::SECRET_KEY, xyber768_oqs::PUBLIC_KEY)
            .unwrap();
    bench(
        "xyber768_oqs",
        xyber768_oqs_endpoint,
        xyber768_oqs::PUBLIC_KEY,
        c,
    );
}

pub fn xyber768_oqs_ghp(c: &mut Criterion) {
    let xyber768_oqs_ghp_endpoint: HpkeEndpoint<xyber768_oqs_ghp::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(
            INFO,
            xyber768_oqs_ghp::SECRET_KEY,
            xyber768_oqs_ghp::PUBLIC_KEY,
        )
        .unwrap();
    bench(
        "xyber768_oqs_ghp",
        xyber768_oqs_ghp_endpoint,
        xyber768_oqs_ghp::PUBLIC_KEY,
        c,
    );
}

pub fn xyber768dilithium_oqs(c: &mut Criterion) {
    let xyber768dilithium_oqs_endpoint: HpkeEndpoint<
        xyber768dilithium_oqs::KemType,
        AeadType,
        KdfType,
    > = HpkeEndpoint::new_with_key(
        INFO,
        xyber768dilithium_oqs::SECRET_KEY,
        xyber768dilithium_oqs::PUBLIC_KEY,
    )
    .unwrap();
    bench(
        "xyber768dilithium_oqs",
        xyber768dilithium_oqs_endpoint,
        xyber768dilithium_oqs::PUBLIC_KEY,
        c,
    );
}

pub fn xyber768dilithium_oqs_ghp(c: &mut Criterion) {
    let xyber768dilithium_oqs_ghp_endpoint: HpkeEndpoint<
        xyber768dilithium_oqs_ghp::KemType,
        AeadType,
        KdfType,
    > = HpkeEndpoint::new_with_key(
        INFO,
        xyber768dilithium_oqs_ghp::SECRET_KEY,
        xyber768dilithium_oqs_ghp::PUBLIC_KEY,
    )
    .unwrap();
    bench(
        "xyber768dilithium_oqs_ghp",
        xyber768dilithium_oqs_ghp_endpoint,
        xyber768dilithium_oqs_ghp::PUBLIC_KEY,
        c,
    );
}

pub fn bench<T: Endpoint>(name: &str, endpoint: T, pub_key: &[u8], c: &mut Criterion) {
    c.bench_function(&format!("{name} seal"), |b| {
        b.iter(|| {
            endpoint
                .seal(
                    black_box(ADDITIONAL_DATA),
                    black_box(pub_key),
                    black_box(MESSAGE),
                )
                .unwrap()
        })
    });

    c.bench_function(&format!("{name} open"), |b| {
        b.iter_batched(
            || endpoint.seal(ADDITIONAL_DATA, pub_key, MESSAGE).unwrap(),
            |msg| {
                endpoint
                    .open(
                        black_box(ADDITIONAL_DATA),
                        black_box(pub_key),
                        black_box(&msg),
                    )
                    .unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    empty,
    hdkf_sha256,
    hdkf_sha512,
    kyber768,
    kyber768dilithium,
    // xyber1024dilithium_oqs, // Error
    xyber768_oqs,
    xyber768_oqs_ghp,
    xyber768dilithium_oqs,
    xyber768dilithium_oqs_ghp
);
criterion_main!(benches);
