//! `Database::declared_kdf_config` reads the KDF parameters a KDBX4 header
//! declares without deriving a key, so a caller can refuse an unreasonable
//! declared cost before `open` spends it.

#[allow(
    missing_docs,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::indexing_slicing
)]
mod declared_kdf_config {
    use keepass::{
        config::KdfConfig,
        db::{Database, DatabaseOpenError},
    };

    use std::{fs, path::Path};

    #[test]
    fn reads_the_kdf_of_a_kdbx4_file_without_a_key() {
        let bytes = fs::read(Path::new("tests/resources/test_db_kdbx4_with_password_aes.kdbx")).unwrap();
        let kdf = Database::declared_kdf_config(&bytes).expect("header parses");
        assert!(
            matches!(kdf, KdfConfig::Aes { .. }),
            "fixture is AES-KDF, got {:?}",
            kdf
        );
    }

    #[test]
    fn refuses_formats_other_than_kdbx4() {
        let bytes = fs::read(Path::new("tests/resources/test_db_with_password.kdbx")).unwrap();
        let err = Database::declared_kdf_config(&bytes).expect_err("KDBX3 has no KDBX4 header");
        assert!(
            matches!(err, DatabaseOpenError::UnsupportedVersion),
            "got {:?}",
            err
        );
    }

    #[test]
    fn a_truncated_header_is_an_error_not_a_panic() {
        let bytes = fs::read(Path::new("tests/resources/test_db_kdbx4_with_password_aes.kdbx")).unwrap();
        for cut in [12usize, 20, 40] {
            assert!(
                Database::declared_kdf_config(&bytes[..cut]).is_err(),
                "a header cut at {} bytes must not parse",
                cut
            );
        }
    }

    #[cfg(feature = "save_kdbx4")]
    #[test]
    fn reports_exactly_what_was_written() {
        use keepass::{config::DatabaseConfig, DatabaseKey};

        let mut config = DatabaseConfig::default();
        config.kdf_config = KdfConfig::Aes { rounds: 123 };
        let db = Database::with_config(config);
        let mut bytes = Vec::new();
        db.save(&mut bytes, DatabaseKey::new().with_password("demopass"))
            .unwrap();

        let kdf = Database::declared_kdf_config(&bytes).unwrap();
        assert!(matches!(kdf, KdfConfig::Aes { rounds: 123 }), "got {:?}", kdf);
    }
}
