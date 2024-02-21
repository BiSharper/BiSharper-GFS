use std::sync::Arc;
use crate::GfsResult;
use crate::io::{GfsFile, WritableFile, ReadableFile};
use crate::path::{GfsPath, OwnedGfsPath, PathLike};

pub const GFS_SEPARATOR: char = '/';

pub trait GfsEntryMeta : Copy + Clone + Default {

}

pub trait GfsSnapshot<T: GfsEntryMeta> : Sized {
    fn root(&self) -> &OwnedGfsPath<T, Self>;

    fn create_path(&self, path: &GfsPath) -> OwnedGfsPath<T, Self> { self.root().join(&self.normalize_path(path.to_string())) }

    fn normalize_path(&self, path: String) -> String;

    fn read_root(&self) -> Box<[OwnedGfsPath<T, Self>]> { self.read_dir(self.root().as_str()) }

    fn read_meta(&self, path: &GfsPath) -> Option<T>;

    fn read_data(&self, path: &GfsPath) -> Option<Arc<Vec<u8>>>;

    fn read_dir(&self, path: &str) -> Box<[OwnedGfsPath<T, Self>]>;

    fn read_entry(&self, path: &GfsPath) -> Option<GfsFile<T>> {
        Some(GfsFile::create(self.read_meta(path)?, self.read_data(path)?))
    }

    fn entry_reader(&self, path: &GfsPath) -> Option<ReadableFile<T>> {
        Some(ReadableFile::from(self.read_entry(path)?))
    }
}

pub trait GFS<T: GfsEntryMeta> : GfsSnapshot<T> {

    fn rename_entry(&self, path: &GfsPath, new_path: &GfsPath) -> GfsResult<()>;

    fn drop_entry(&self, path: &GfsPath) -> GfsResult<GfsFile<T>>;

    fn remove_entry(&self, path: &GfsPath) -> GfsResult<()> { self.drop_entry(path)?; Ok(()) }

    fn entry_writer(&self, path: &GfsPath) -> WritableFile<T, Self> {
        let owned_path = self.create_path(path);
        if let Some(entry) = self.read_entry(path) {
            return WritableFile::from_owned(&owned_path, entry.metadata, entry.contents.to_vec())
        }
        WritableFile::from_owned(&owned_path, Default::default(), vec![])
    }

    fn insert_entry(&self, path: &GfsPath, metadata: T, data: Arc<Vec<u8>>) -> GfsResult<&GfsFile<T>>;

}