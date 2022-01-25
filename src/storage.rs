use std::io::Write;
use std::process::Command;

pub trait Storage {
    fn get(&self, key: &str) -> Result<String, Error>;
    fn set(&mut self, key: &str, value: &str) -> Result<(), Error>;
    fn list(&self, prefix: &str) -> Result<Vec<String>, Error>;
    fn remove(&mut self, key: &str) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct GpgStorage {
    pub base_path: String,
}

impl Storage for GpgStorage {
    fn get(&self, key: &str) -> Result<String, Error> {
        let mut cmd = Command::new("gpg");
        cmd.arg("--decrypt");
        cmd.arg("--output");
        cmd.arg("-");
        cmd.arg("--default-recipient-self");
        cmd.arg(self.base_path.to_owned() + "/" + key + ".gpg");
        let output = cmd.output()?;
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(Error::InvalidKey(String::from_utf8(output.stderr).unwrap()))
        }
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        self.ensure_dir(key)?;

        let mut cmd = Command::new("gpg");
        cmd.arg("--encrypt");
        cmd.arg("--output");
        cmd.arg(self.base_path.to_owned() + "/" + key + ".gpg");
        cmd.arg("--default-recipient-self");
        cmd.arg("-");
        cmd.stdin(std::process::Stdio::piped());
        let mut child = cmd.spawn()?;
        child
            .stdin
            .as_mut()
            .ok_or(Error::IO("Failed to open child stdin".to_string()))?
            .write_all(value.as_bytes())?;

        let output = child.wait_with_output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(Error::InvalidKey(String::from_utf8(output.stderr).unwrap()))
        }
    }

    fn remove(&mut self, key: &str) -> Result<(), Error> {
        std::fs::remove_file(self.base_path.to_owned() + "/" + key + ".gpg")?;
        Ok(())
    }

    fn list(&self, dir: &str) -> Result<Vec<String>, Error> {
        Ok(walkdir::WalkDir::new(self.base_path.to_owned() + "/" + dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                e.path().to_str().unwrap().to_string()[self.base_path.len() + dir.len() + 1..]
                    .to_string()
            })
            .map(|e| e.replace(".gpg", ""))
            .collect())
    }
}

impl GpgStorage {
    pub fn new(base_path: &str) -> GpgStorage {
        GpgStorage {
            base_path: base_path.to_string(),
        }
    }

    fn ensure_dir(&mut self, path: &str) -> Result<(), Error> {
        let path = std::path::Path::new(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(self.base_path.to_owned() + "/" + parent.to_str().unwrap())?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidKey(String),
    IO(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidKey(s) => write!(f, "Invalid key: {}", s),
            Error::IO(s) => write!(f, "IO error: {}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e.to_string())
    }
}

impl std::error::Error for Error {}

mod test {

    #[test]
    fn test_gpg_storage() {
        use super::{GpgStorage, Storage};

        let dir = tempfile::tempdir().unwrap();
        let mut storage = GpgStorage::new(dir.path().to_str().unwrap());
        let key = "foo/bar/test_key";
        let value = "test_value";
        storage.set(key, value).unwrap();
        let result = storage.get(key).unwrap();
        assert_eq!(result, value);
        storage.remove(key).unwrap();
        let result = storage.get(key);
        assert!(result.is_err());
    }
}

pub struct GitStorage<S: Storage> {
    storage: S,
    base_path: String,
}

impl<S: Storage> GitStorage<S> {
    pub fn new(storage: S, base_path: &str) -> GitStorage<S> {
        // check if git is initialized
        let mut cmd = Command::new("git");
        cmd.arg("status");
        let output = cmd.output().unwrap();
        if !output.status.success() {
            // git is not initialized, initialize it
            let mut cmd = Command::new("git");
            cmd.current_dir(base_path);
            cmd.arg("init");
            cmd.arg("-b");
            cmd.arg("main");
            cmd.output().unwrap();
        }
        GitStorage {
            storage,
            base_path: base_path.to_string(),
        }
    }

    fn commit(&mut self, msg: &str) -> Result<(), Error> {
        let mut cmd = Command::new("git");
        cmd.current_dir(self.base_path.to_owned());
        cmd.arg("commit");
        cmd.arg("-a");
        cmd.arg("-m");
        cmd.arg(msg);
        cmd.output()?;
        Ok(())
    }
}

impl<S> Storage for GitStorage<S>
where
    S: Storage,
{
    fn get(&self, key: &str) -> Result<String, Error> {
        self.storage.get(key)
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        self.storage.set(key, value)?;
        self.commit(&("saved ".to_string() + key))
    }

    fn list(&self, dir: &str) -> Result<Vec<String>, Error> {
        self.storage.list(dir)
    }

    fn remove(&mut self, key: &str) -> Result<(), Error> {
        self.storage.remove(key)?;
        self.commit(&("removed ".to_string() + key))
    }
}
