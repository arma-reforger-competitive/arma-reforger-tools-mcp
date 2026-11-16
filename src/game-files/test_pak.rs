//! Builds small .pak files for the tests, with the layout of the real game files.

use std::collections::BTreeMap;

/// Byte length of the FORM header, the HEAD chunk, and the header of the DATA chunk. The first
/// file starts here, as in every real .pak file.
pub(super) const DATA_START: u32 = 56;

/// A .pak file, built one FILE table entry at a time.
pub(super) struct PakBuilder {
    data: Vec<u8>,
    table: Vec<u8>,
}

impl PakBuilder {
    pub(super) fn new() -> Self {
        Self {
            data: Vec::new(),
            table: Vec::new(),
        }
    }

    /// A pak with one short text file for each of `files`, in folders as the paths say.
    pub(super) fn with_files(files: &[&str]) -> Self {
        let mut root = Folder::default();
        for file in files {
            let mut folder = &mut root;
            let mut parts: Vec<&str> = file.split('/').collect();
            let name = parts.pop().unwrap();
            for part in parts {
                folder = folder.folders.entry(part.to_owned()).or_default();
            }
            folder.files.push(name.to_owned());
        }
        Self::new().folder("", &root)
    }

    /// A folder entry that says it has `children` entries after it.
    pub(super) fn dir(mut self, name: &str, children: u32) -> Self {
        self.entry_head(0, name);
        self.table.extend(children.to_le_bytes());
        self
    }

    /// A file entry with its content stored in the DATA chunk without compression.
    pub(super) fn file(mut self, name: &str, content: &[u8]) -> Self {
        let offset = DATA_START + u32::try_from(self.data.len()).unwrap();
        let size = u32::try_from(content.len()).unwrap();
        self.data.extend(content);
        self.raw_file(name, offset, size, size, false)
    }

    /// A file entry with the numbers given as they are. It adds no content to the DATA chunk.
    pub(super) fn raw_file(
        mut self,
        name: &str,
        offset: u32,
        compressed_len: u32,
        len: u32,
        compressed: bool,
    ) -> Self {
        self.entry_head(1, name);
        for number in [offset, compressed_len, len, 0] {
            self.table.extend(number.to_le_bytes());
        }
        self.table
            .extend([0, 0, u8::from(compressed), 6, 0, 0, 0, 0]);
        self
    }

    /// Adds `len` bytes to the DATA chunk that no file entry points to.
    pub(super) fn data(mut self, len: usize) -> Self {
        self.data.resize(self.data.len() + len, 0);
        self
    }

    /// The bytes of the whole .pak file.
    pub(super) fn build(&self) -> Vec<u8> {
        self.build_with_table_len(u32::try_from(self.table.len()).unwrap())
    }

    /// The bytes of the whole .pak file, with `table_len` as the size in the FILE chunk header.
    pub(super) fn build_with_table_len(&self, table_len: u32) -> Vec<u8> {
        let mut chunks = Vec::new();
        chunk(&mut chunks, *b"HEAD", &[0; 28], 28);
        let data_len = u32::try_from(self.data.len()).unwrap();
        chunk(&mut chunks, *b"DATA", &self.data, data_len);
        chunk(&mut chunks, *b"FILE", &self.table, table_len);
        let mut pak = b"FORM".to_vec();
        pak.extend(u32::try_from(chunks.len() + 4).unwrap().to_be_bytes());
        pak.extend(b"PAC1");
        pak.extend(chunks);
        pak
    }

    fn folder(self, name: &str, folder: &Folder) -> Self {
        let children = u32::try_from(folder.folders.len() + folder.files.len()).unwrap();
        let mut pak = self.dir(name, children);
        for (name, inner) in &folder.folders {
            pak = pak.folder(name, inner);
        }
        for name in &folder.files {
            pak = pak.file(name, b"// test file\n");
        }
        pak
    }

    fn entry_head(&mut self, kind: u8, name: &str) {
        self.table.push(kind);
        self.table.push(u8::try_from(name.len()).unwrap());
        self.table.extend(name.as_bytes());
    }
}

#[derive(Default)]
struct Folder {
    folders: BTreeMap<String, Folder>,
    files: Vec<String>,
}

fn chunk(out: &mut Vec<u8>, magic: [u8; 4], payload: &[u8], len: u32) {
    out.extend(magic);
    out.extend(len.to_be_bytes());
    out.extend(payload);
}
