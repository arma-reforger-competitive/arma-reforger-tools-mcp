//! Reads the list of files in a .pak archive of the game or of a mod, and the content of one file.
//!
//! A .pak file is a FORM PAC1 container of chunks: HEAD, DATA with the file contents, and FILE
//! with a tree of folder and file entries. The `offset` of a file is a position from the start of
//! the .pak file. A compressed file is zlib with a header. Every size, offset, and count is
//! checked against the file length or a limit.

use std::io::{Read, Seek, SeekFrom};

use flate2::read::ZlibDecoder;

use super::{Error, Result};

/// The largest FILE table that the reader loads. The largest in the game has 2.4 MB.
const MAX_TABLE_BYTES: u64 = 16 * 1024 * 1024;

/// The deepest folder. The deepest in the game is 9 folders down.
const MAX_DEPTH: usize = 32;

/// The longest path. The longest in the game has about 150 bytes.
const MAX_PATH_BYTES: usize = 512;

/// The smallest entry: kind and name length (2 bytes), and the 4-byte child count of a folder.
const MIN_ENTRY_BYTES: u64 = 6;

/// The bytes of a file entry after its name: offset, compressed and full size, 4 bytes we do not
/// use, 2 bytes we do not use, compressed flag, compression level, and time stamp.
const FILE_FIELDS_BYTES: usize = 24;

/// The largest full size of a file that the reader unpacks. Without a limit, a damaged FILE table
/// could make the reader reserve up to 4 GB.
pub(super) const MAX_FILE_BYTES: u32 = 64 * 1024 * 1024;

const FOLDER: u8 = 0;
const FILE: u8 = 1;

/// One file in a .pak archive.
pub(super) struct PakFile {
    pub(super) path: String,
    pub(super) data: FileData,
}

/// Where the content of a file lies in its .pak archive. The reader checked that it lies inside.
#[derive(Clone, Copy, Debug)]
pub(super) struct FileData {
    /// The position of the first byte, from the start of the .pak file.
    offset: u64,

    /// The bytes in the .pak file: the compressed size, or the full size of a stored file.
    stored: u32,

    /// The full size after unpacking.
    len: u32,
    compressed: bool,
}

impl FileData {
    #[cfg(test)]
    pub(super) fn stored(self) -> u32 {
        self.stored
    }
}

/// Lists every file in the .pak archive `pak`, which is `len` bytes long.
pub(super) fn list_files<R: Read + Seek>(pak: &mut R, len: u64) -> Result<Vec<PakFile>> {
    let table = read_file_table(pak, len)?;
    let mut table = Table {
        rest: &table,
        pak_len: len,
    };
    let mut files = Vec::new();
    let (kind, _) = table.entry_head()?;
    if kind != FOLDER {
        return Err(Error::RootNotAFolder);
    }
    let mut path = String::new();
    let count = table.child_count(&path)?;
    table.read_children(count, &mut path, 1, &mut files)?;
    Ok(files)
}

/// Reads the full content of the file at `data` in the .pak archive `pak`. A compressed file is
/// unpacked with zlib. A full size above `MAX_FILE_BYTES` is an error before any memory is
/// reserved, and an unpacked size that differs from the full size in the FILE table is an error.
pub(super) fn read_file<R: Read + Seek>(pak: &mut R, data: FileData) -> Result<Vec<u8>> {
    if data.len > MAX_FILE_BYTES {
        return Err(Error::FileTooLarge {
            len: data.len,
            max: MAX_FILE_BYTES,
        });
    }
    pak.seek(SeekFrom::Start(data.offset))?;
    let stored = pak.take(u64::from(data.stored));
    let mut content = Vec::with_capacity(usize::try_from(data.len).unwrap_or_default());
    // One byte more than the full size shows a file that unpacks to more than it says.
    let limit = u64::from(data.len) + 1;
    if data.compressed {
        ZlibDecoder::new(stored)
            .take(limit)
            .read_to_end(&mut content)
            .map_err(Error::Unpack)?;
    } else {
        stored.take(limit).read_to_end(&mut content)?;
    }
    if content.len() != usize::try_from(data.len).unwrap_or(usize::MAX) {
        return Err(Error::WrongSize {
            len: data.len,
            got: content.len(),
        });
    }
    Ok(content)
}

// region:    --- Support

/// Walks the chunks after the FORM header and returns the payload of the FILE chunk.
fn read_file_table<R: Read + Seek>(pak: &mut R, len: u64) -> Result<Vec<u8>> {
    let mut header = [0; 12];
    if len < 12 {
        return Err(Error::NotAPak);
    }
    pak.seek(SeekFrom::Start(0))?;
    pak.read_exact(&mut header)?;
    if header.get(..4) != Some(b"FORM") || header.get(8..) != Some(b"PAC1") {
        return Err(Error::NotAPak);
    }
    let mut at: u64 = 12;
    while len - at >= 8 {
        let mut chunk = [0; 8];
        pak.seek(SeekFrom::Start(at))?;
        pak.read_exact(&mut chunk)?;
        let [m0, m1, m2, m3, s0, s1, s2, s3] = chunk;
        let size = u64::from(u32::from_be_bytes([s0, s1, s2, s3]));
        let payload = at + 8;
        if size > len - payload {
            return Err(Error::ChunkOutsidePak { at });
        }
        if [m0, m1, m2, m3] == *b"FILE" {
            if size > MAX_TABLE_BYTES {
                return Err(Error::FileTableTooLarge {
                    len: size,
                    max: MAX_TABLE_BYTES,
                });
            }
            let mut table = vec![0; usize::try_from(size).map_err(|_| Error::NoFileTable)?];
            pak.read_exact(&mut table)?;
            return Ok(table);
        }
        at = payload + size;
    }
    Err(Error::NoFileTable)
}

/// The part of the FILE table that is not read yet.
struct Table<'a> {
    rest: &'a [u8],
    pak_len: u64,
}

impl<'a> Table<'a> {
    /// Reads `count` entries into `files`. `path` is the path of their folder, `depth` its depth.
    fn read_children(
        &mut self,
        count: u32,
        path: &mut String,
        depth: usize,
        files: &mut Vec<PakFile>,
    ) -> Result<()> {
        for _ in 0..count {
            let (kind, name) = self.entry_head()?;
            let folder_len = path.len();
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(&String::from_utf8_lossy(name));
            if path.len() > MAX_PATH_BYTES {
                return Err(Error::PathTooLong {
                    path: path.clone(),
                    max: MAX_PATH_BYTES,
                });
            }
            match kind {
                FOLDER => {
                    if depth >= MAX_DEPTH {
                        return Err(Error::TooDeep {
                            path: path.clone(),
                            max: MAX_DEPTH,
                        });
                    }
                    let count = self.child_count(path)?;
                    self.read_children(count, path, depth + 1, files)?;
                }
                FILE => {
                    let data = self.file_data(path)?;
                    files.push(PakFile {
                        path: path.clone(),
                        data,
                    });
                }
                kind => {
                    return Err(Error::BadEntryKind {
                        path: path.clone(),
                        kind,
                    });
                }
            }
            path.truncate(folder_len);
        }
        Ok(())
    }

    /// Reads the kind and the name of an entry.
    fn entry_head(&mut self) -> Result<(u8, &'a [u8])> {
        let [kind, name_len] = *self.take::<2>()?;
        let (name, rest) = self
            .rest
            .split_at_checked(usize::from(name_len))
            .ok_or(Error::FileTableEnds)?;
        self.rest = rest;
        Ok((kind, name))
    }

    /// Reads the child count of the folder `path`. Each child needs at least 6 bytes of the
    /// table, so a count that the rest of the table cannot hold is an error.
    fn child_count(&mut self, path: &str) -> Result<u32> {
        let count = u32::from_le_bytes(*self.take::<4>()?);
        let rest = u64::try_from(self.rest.len()).unwrap_or(u64::MAX);
        if u64::from(count) * MIN_ENTRY_BYTES > rest {
            return Err(Error::TooManyChildren {
                path: path.to_owned(),
                count,
            });
        }
        Ok(count)
    }

    /// Reads the fields of the file `path` and checks that its data lies inside the .pak file.
    fn file_data(&mut self, path: &str) -> Result<FileData> {
        let [
            o0,
            o1,
            o2,
            o3,
            c0,
            c1,
            c2,
            c3,
            l0,
            l1,
            l2,
            l3,
            _,
            _,
            _,
            _,
            _,
            _,
            compressed,
            _,
            _,
            _,
            _,
            _,
        ] = *self.take::<FILE_FIELDS_BYTES>()?;
        let offset = u64::from(u32::from_le_bytes([o0, o1, o2, o3]));
        let len = u32::from_le_bytes([l0, l1, l2, l3]);
        let compressed = compressed != 0;
        let stored = if compressed {
            u32::from_le_bytes([c0, c1, c2, c3])
        } else {
            len
        };
        let end = offset + u64::from(stored);
        if end > self.pak_len {
            return Err(Error::FileOutsidePak {
                path: path.to_owned(),
                end,
                len: self.pak_len,
            });
        }
        Ok(FileData {
            offset,
            stored,
            len,
            compressed,
        })
    }

    fn take<const N: usize>(&mut self) -> Result<&'a [u8; N]> {
        let (bytes, rest) = self
            .rest
            .split_first_chunk::<N>()
            .ok_or(Error::FileTableEnds)?;
        self.rest = rest;
        Ok(bytes)
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "pak_tests.rs"]
mod tests;

// endregion: --- Tests
