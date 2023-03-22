use crate::{
    Error,
    GuestConfig
};
use core::{
    fmt,
    marker::PhantomData
};
use serde::{
    Deserialize,
    Serialize,
    ser::SerializeMap,
    de::{
        Error as _,
        MapAccess,
        Visitor
    }
};
use rusqlite::{
    Connection,
    OpenFlags
};

#[derive(Clone)]
pub struct ConfigStorage {
    uri: String
}

pub struct Labeled<T> {
    label: String,
    item: T
}

impl<T> Labeled<T> {

    pub fn new<S: AsRef<str>>(label: S, item: T) -> Self {
        Self {
            label: label.as_ref().to_string(),
            item
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    #[allow(unused)]
    pub fn item(&self) -> &T {
        &self.item
    }

}

impl<T> Serialize for Labeled<T>
where
    T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry(&self.label, &self.item)?;
            map.end()
    }
}

impl<'de, T> Deserialize<'de> for Labeled<T> 
where
    T: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            struct LabelVisitor<T>(PhantomData<T>);
            use std::any::type_name;

            impl<'de, T> Visitor<'de> for LabelVisitor<T>
            where
                T: Deserialize<'de>
            {

                type Value = Labeled<T>;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "an labeled {}", type_name::<T>())
                }

                fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let mut labeled = None;

                    while let Some((key, value)) = access.next_entry::<String, _>()? {
                        if labeled.is_none() {
                            labeled = Some(Labeled::new(key, value));
                        } else {
                            return Err(M::Error::custom("Too many labels"))
                        }
                    }
                    
                    labeled.ok_or(M::Error::custom(format!("Missing labeled {}", type_name::<T>())))
                }

            }

            deserializer.deserialize_map(LabelVisitor(PhantomData))
        }
}

impl ConfigStorage {

    /// Create storage backed by path
    pub fn new<S: AsRef<str>>(path: S) -> Result<Self, Error> {
        let connection = Connection::open_with_flags(path.as_ref(), OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        let _ = connection.execute("CREATE TABLE IF NOT EXISTS guest (
            id INTEGER PRIMARY KEY,
            name VARCHAR NOT NULL,
            config TEXT NOT NULL
        )", ())?;
        let _ = connection.execute("CREATE UNIQUE INDEX IF NOT EXISTS \
            idx_guest_name ON guest (name)", ())?;
        Ok(Self {
            uri: path.as_ref().to_string()
        })
    }

    /// Lookup ID of a guest name
    pub fn lookup_id(&self, name: &str) -> Result<isize, Error> {
        let connection = Connection::open_with_flags(&self.uri, OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        connection.query_row("SELECT id FROM guest WHERE name = ?", [name], |row| {
            Ok(row.get::<usize, isize>(0)?)
        }).map_err(|err| err.into())
    }

    pub fn get(&self, id: usize) -> Result<GuestConfig, Error> {
        let connection = Connection::open_with_flags(&self.uri, OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        connection.query_row("SELECT config FROM guest WHERE id = ?", [id], |row| {
            Ok(row.get::<usize, String>(0)?)
        }).map_err(|err| err.into())
        .and_then(|config| serde_json::from_str(&config).map_err(|err| err.into()))
    }

    pub fn list(&self, offset: Option<isize>, limit: Option<isize>)
        -> Result<Vec<Labeled<isize>>, Error> {
            let connection = Connection::open_with_flags(&self.uri, OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_READ_ONLY)?;
            let offset = offset.unwrap_or(0);
            let limit: isize = limit.map(|limit| limit as isize).unwrap_or(-1);
            let mut prepare = connection.prepare("SELECT name, id FROM guest LIMIT ? OFFSET ?")?;
            let guests = prepare.query_map([limit, offset], |row| {
                Ok(Labeled::new::<String>(row.get(0)?, row.get(1)?))
            })?.collect::<Result<Vec<_>, _>>().map_err(|err| err.into());
            guests
    }

    pub fn insert(&self, name: &str, config: GuestConfig) -> Result<(), Error> {
        let connection = Connection::open_with_flags(&self.uri, OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        let config = serde_json::to_string(&config)?;
        connection.execute("INSERT INTO guest (name, config) VALUES (?, ?)", 
            [name.to_string(), config])?;
        Ok(())
    }

    pub fn remove(&self, id: isize) -> Result<(), Error>  {
        let connection = Connection::open_with_flags(&self.uri, OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_READ_WRITE)?;
        connection.execute("DELETE FROM guest WHERE id = ?", [id])?;
        Ok(())
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn insert_get_list_remove_list() {
        let uri = "file:memdb1?mode=memory&cache=shared";
        // This is just to keep the database in memory.
        // TODO: Proc-macro for these types of tests?
        let _db = Connection::open_with_flags(uri, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_URI).unwrap();
        let store = ConfigStorage::new(uri).unwrap();
        let config = GuestConfig::new("aarch64".into(), 512);
        store.insert("test", config).unwrap();
    }

}
