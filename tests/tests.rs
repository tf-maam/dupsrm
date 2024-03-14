#[cfg(test)]
mod tests {

    // use super::sha256sum
    use dupsrm::hasher::sha256sum;
    use serial_test::serial;

    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*;
    use std::fs;
    use std::process::Command; // Used for writing assertions
    use std::{
        io::Write,
        path::{Path, PathBuf},
    };

    pub struct CliTestCase {
        pub root_dir_path: PathBuf,
        pub reference_dir_path: PathBuf,
        pub file_path_1: PathBuf,
        pub file_path_2: PathBuf,
    }

    trait CliTest {
        fn new() -> Self;
        fn startup(&self);
        fn teardown(&self);
    }

    impl CliTest for CliTestCase {
        fn new() -> Self {
            let root_dir_path = PathBuf::from("./test/test_root/");
            let reference_dir_path = PathBuf::from("./test/test_reference/");
            fs::create_dir(&reference_dir_path).unwrap_or(());
            let k = 6;
            let file_path_1: PathBuf = reference_dir_path
                .clone()
                .join(format!("file_test_{}.txt", k));
            let k = 9;
            let file_path_2: PathBuf = reference_dir_path.join(format!("file_test_{}.txt", k));

            CliTestCase {
                root_dir_path: root_dir_path,
                reference_dir_path: reference_dir_path,
                file_path_1: file_path_1,
                file_path_2: file_path_2,
            }
        }

        fn startup(&self) {
            // Create file structure
            fs::create_dir(&self.root_dir_path).unwrap_or(());
            for i in 0..10 {
                let dir_path: PathBuf = self.root_dir_path.join(format!("dir_{}/", i)).to_owned();
                fs::create_dir(&dir_path).unwrap_or(());
                for j in 0..10 {
                    let file_path: PathBuf = dir_path.join(format!("file_{}.txt", j));
                    let mut file = fs::File::create(&file_path).unwrap();
                    let _ = file.write_all(format!("test {} {}", i, j).as_bytes());
                }
            }
            fs::create_dir(&self.reference_dir_path).unwrap_or(());
            let i = 5;
            let j = 2;
            let mut file = fs::File::create(&self.file_path_1).unwrap();
            let _ = file.write_all(format!("test {} {}", i, j).as_bytes());
            let i = 50;
            let j = 200;
            let mut file = fs::File::create(&self.file_path_2).unwrap();
            let _ = file.write_all(format!("test {} {}", i, j).as_bytes());
        }

        fn teardown(&self) {
            // Cleanup
            std::fs::remove_dir_all(&self.root_dir_path).unwrap_or(());
            std::fs::remove_dir_all(&self.reference_dir_path).unwrap_or(());
            assert!(!self.root_dir_path.exists());
            assert!(!self.reference_dir_path.exists());
        }
    }

    #[test]
    fn empty_file_exists() {
        let path: &str = "test/test_empty.txt";
        let result = sha256sum(Path::new(path)).unwrap();
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn file_exists() {
        let path: &str = "test/test.txt";
        let result = sha256sum(Path::new(path)).unwrap();
        assert_eq!(
            result,
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn file_does_not_exists() {
        let path: &str = "test/test2.txt";
        let result = sha256sum(Path::new(path));
        assert!(result.is_err());
    }

    #[test]
    // #[ignore]
    #[serial]
    fn dry_run() {
        let test_case = CliTestCase::new();
        test_case.startup();

        // Check prerequisites
        assert!(test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());
        assert!(test_case.root_dir_path.exists());
        assert!(test_case.reference_dir_path.exists());

        // Execute program
        let mut cmd = match Command::cargo_bin("dupsrm") {
            Err(err) => panic!("{}", err),
            Ok(cmd) => cmd,
        };
        cmd.arg(&test_case.reference_dir_path)
            .arg(&test_case.root_dir_path)
            .arg("-d");
        cmd.assert().success();

        // Check results
        assert!(test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());

        test_case.teardown();
    }

    #[test]
    #[serial]
    fn duplicates_removed() {
        let test_case = CliTestCase::new();
        test_case.startup();

        // Check prerequisites
        assert!(test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());
        assert!(test_case.root_dir_path.exists());
        assert!(test_case.reference_dir_path.exists());

        // Execute program
        let mut cmd = match Command::cargo_bin("dupsrm") {
            Err(err) => panic!("{}", err),
            Ok(cmd) => cmd,
        };
        cmd.arg(&test_case.reference_dir_path)
            .arg(&test_case.root_dir_path);
        cmd.assert().success();

        // Check results
        assert!(!test_case.file_path_1.exists());
        assert!(test_case.file_path_2.exists());

        test_case.teardown();
    }

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("dupsrm")?;

        cmd.arg("./dsdfgdf").arg(".");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("No such file or directory"));

        Ok(())
    }

    #[test]
    fn same_reference_and_root() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("dupsrm")?;

        cmd.arg("./").arg(".");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Reference directory must not be identical to root directory",
        ));

        Ok(())
    }
}
