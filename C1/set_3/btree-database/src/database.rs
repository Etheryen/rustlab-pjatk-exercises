use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Database<K, V>
where
    K: Ord,
{
    data: BTreeMap<K, V>,
}

impl<K, V> Database<K, V>
where
    K: Serialize + for<'a> Deserialize<'a> + Ord,
    V: Serialize + for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn get<IntoK>(&self, key: IntoK) -> Option<&V>
    where
        IntoK: Into<K>,
    {
        self.data.get(&key.into())
    }

    pub fn insert<IntoK, IntoV>(&mut self, key: IntoK, value: IntoV)
    where
        IntoK: Into<K>,
        IntoV: Into<V>,
    {
        self.data.insert(key.into(), value.into());
    }

    pub fn delete<IntoK>(&mut self, key: IntoK)
    where
        IntoK: Into<K>,
    {
        self.data.remove(&key.into());
    }

    pub fn save_to_file<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let json = serde_json::to_string(&self.data)?;
        fs::write(path, json)
    }

    pub fn load_from_file<P>(&mut self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let json = fs::read_to_string(path)?;
        self.data = serde_json::from_str(json.as_str())?;
        Ok(())
    }
}
