use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crypto_agility_crypto_api::server::{hpke::HpkeEndpoint, Endpoint};
use memory_bench::{dilithium, empty, hdkf_sha256, kyber768};

pub type AeadType = hpke::aead::ChaCha20Poly1305;

pub type KdfType = hpke::kdf::HkdfSha384;

pub fn seal(c: &mut Criterion) {
    let info = [1, 2, 3, 4];

    let empty_endpoint: HpkeEndpoint<empty::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, empty::SECRET_KEY, empty::PUBLIC_KEY).unwrap();
    let empty_public_key = empty_endpoint.get_public_key();

    let hdkf_sha256_endpoint: HpkeEndpoint<hdkf_sha256::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, hdkf_sha256::SECRET_KEY, hdkf_sha256::PUBLIC_KEY)
            .unwrap();
    let hdkf_sha256_public_key = hdkf_sha256_endpoint.get_public_key();

    let dilithium_endpoint: HpkeEndpoint<dilithium::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, dilithium::SECRET_KEY, dilithium::PUBLIC_KEY).unwrap();
    let dilithium_public_key = dilithium_endpoint.get_public_key();

    let kyber768_endpoint: HpkeEndpoint<kyber768::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, kyber768::SECRET_KEY, kyber768::PUBLIC_KEY).unwrap();
    let kyber768_public_key = kyber768_endpoint.get_public_key();

    let additional_data = b"Foo".to_vec();
    let message = b"Hello, World!".to_vec();

    c.bench_function("empty seal", |b| {
        b.iter(|| {
            empty_endpoint
                .seal(
                    black_box(&additional_data),
                    black_box(&empty_public_key),
                    black_box(&message),
                )
                .unwrap()
        })
    });
    c.bench_function("hdkf_sha256 seal", |b| {
        b.iter(|| {
            hdkf_sha256_endpoint
                .seal(
                    black_box(&additional_data),
                    black_box(&hdkf_sha256_public_key),
                    black_box(&message),
                )
                .unwrap()
        })
    });
    c.bench_function("dilithium seal", |b| {
        b.iter(|| {
            dilithium_endpoint
                .seal(
                    black_box(&additional_data),
                    black_box(&dilithium_public_key),
                    black_box(&message),
                )
                .unwrap()
        })
    });
    c.bench_function("kyber768 seal", |b| {
        b.iter(|| {
            kyber768_endpoint
                .seal(
                    black_box(&additional_data),
                    black_box(&kyber768_public_key),
                    black_box(&message),
                )
                .unwrap()
        })
    });
}

pub fn open(c: &mut Criterion) {
    let info = [1, 2, 3, 4];

    let empty_endpoint: HpkeEndpoint<empty::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, empty::SECRET_KEY, empty::PUBLIC_KEY).unwrap();
    let empty_public_key = empty_endpoint.get_public_key();

    let hdkf_sha256_endpoint: HpkeEndpoint<hdkf_sha256::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, hdkf_sha256::SECRET_KEY, hdkf_sha256::PUBLIC_KEY)
            .unwrap();
    let hdkf_sha256_public_key = hdkf_sha256_endpoint.get_public_key();

    let dilithium_endpoint: HpkeEndpoint<dilithium::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, dilithium::SECRET_KEY, dilithium::PUBLIC_KEY).unwrap();
    let dilithium_public_key = dilithium_endpoint.get_public_key();

    let kyber768_endpoint: HpkeEndpoint<kyber768::KemType, AeadType, KdfType> =
        HpkeEndpoint::new_with_key(&info, kyber768::SECRET_KEY, kyber768::PUBLIC_KEY).unwrap();
    let kyber768_public_key = kyber768_endpoint.get_public_key();

    let additional_data = b"Foo".to_vec();
    let message = b"Hello, World!".to_vec();

    c.bench_function("empty open", |b| {
        b.iter_batched(
            || {
                empty_endpoint
                    .seal(
                        black_box(&additional_data),
                        black_box(&empty_public_key),
                        black_box(&message),
                    )
                    .unwrap()
            },
            |msg| {
                empty_endpoint
                    .open(&additional_data, &empty_public_key, &msg)
                    .unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("hdkf_sha256 open", |b| {
        b.iter_batched(
            || {
                hdkf_sha256_endpoint
                    .seal(
                        black_box(&additional_data),
                        black_box(&hdkf_sha256_public_key),
                        black_box(&message),
                    )
                    .unwrap()
            },
            |msg| {
                hdkf_sha256_endpoint
                    .open(&additional_data, &hdkf_sha256_public_key, &msg)
                    .unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("dilithium open", |b| {
        b.iter_batched(
            || {
                dilithium_endpoint
                    .seal(
                        black_box(&additional_data),
                        black_box(&dilithium_public_key),
                        black_box(&message),
                    )
                    .unwrap()
            },
            |msg| {
                dilithium_endpoint
                    .open(&additional_data, &dilithium_public_key, &msg)
                    .unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("kyber768 open", |b| {
        b.iter_batched(
            || {
                kyber768_endpoint
                    .seal(
                        black_box(&additional_data),
                        black_box(&kyber768_public_key),
                        black_box(&message),
                    )
                    .unwrap()
            },
            |msg| {
                kyber768_endpoint
                    .open(&additional_data, &kyber768_public_key, &msg)
                    .unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = seal, open
}
criterion_main!(benches);
