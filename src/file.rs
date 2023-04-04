use core::fmt;
use std::{
    time::SystemTime,
    path::{
        Path,
        PathBuf
    }
};
use url::Url;
use serde::{
    Serialize,
    Deserialize,
    Deserializer,
    de::{self, Visitor}
};
use curl::easy::Easy2;
use crate::{
    Error,
    curl::Collector,
    crypto::MessageDigest
};

#[derive(Clone, Debug, PartialEq)]
pub enum File {
    Local(PathBuf),
    Remote(Url)
}

struct Hex<'a>(&'a [u8]);

impl fmt::Display for Hex<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:X}")?;
        }
        Ok(())
    }
}

impl File {

    /// Converts remote files to local files
    pub fn path<P: AsRef<Path>>(&self, local_storage: P) -> Result<PathBuf, Error> {
        match self {
            Self::Local(path) => Ok(path.to_path_buf()),
            Self::Remote(url) => {
                
                let mut md = MessageDigest::new("SHA3-256")?;
                md.update(format!("{url}").as_bytes())?;
                let hashed_filename = md.r#final()?;
                let mut local_filename = local_storage.as_ref().to_path_buf();
                local_filename.push(format!("{}", Hex(hashed_filename.as_ref())));   

                let mut easy = Easy2::<Collector>::new(Collector(Vec::new()));
                easy.url(&format!("{url}"))?;
           
                if let Some(filetime) = easy.filetime()? {
                    let metadata = std::fs::metadata(&local_filename)?;
                    let modified = metadata.modified()?
                        .duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
                    if filetime as u64 > modified {
                        easy.perform()?;
                        std::fs::write(&local_filename, &easy.get_ref().0)?;
                    }
                } else {
                    easy.perform()?;
                    std::fs::write(&local_filename, &easy.get_ref().0)?;
                }


                Ok(local_filename)
            }
        }
    }

}

impl<'de> Deserialize<'de> for File {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
            struct FileVisitor;

            impl<'de> Visitor<'de> for FileVisitor {

                type Value = File;

                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "path or uri")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error {
                        let file = TryInto::<Url>::try_into(v)
                            .map(|url| File::Remote(url))
                            .unwrap_or(File::Local(v.into()));
                        Ok(file)
                }
            }

            deserializer.deserialize_str(FileVisitor)
        }
}

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match self {
                Self::Remote(url) => serializer.serialize_str(&format!("{url}")),
                Self::Local(path) => serializer.serialize_str(&format!("{}", path.display())),
            }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn local_file() {
        let file = File::Local(format!("/path/to/file").into());
        assert_tokens(&file, &[Token::Str("/path/to/file")]);
    }

    #[test]
    fn remote_file() {
        let file = File::Remote("http://localhost/path/to/file".try_into().unwrap());
        assert_tokens(&file, &[Token::Str("http://localhost/path/to/file")]);
    }

}
