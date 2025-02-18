//! Implements an writable instance of a GRF File
//! This way, we can provide a temporal storage to files before the final write
//! occurs while keeping it outside of the
//! [`NativeArchive`](super::NativeArchive) implementation
use std::path::{Path, PathBuf};

use yazi::{compress, CompressionLevel, Format};

use super::assettable::AssetTable;
use super::filetablerow::FileTableRow;
use super::header::Header;
use super::{FileTable, MAGIC_BYTES};
use crate::loaders::archive::Writable;
use crate::loaders::ByteConvertable;

pub struct NativeArchiveBuilder {
    os_file_path: PathBuf,
    file_table: FileTable,
    data: Vec<u8>,
}

impl NativeArchiveBuilder {
    pub fn from_path(path: &Path) -> Self {
        Self {
            os_file_path: PathBuf::from(path),
            file_table: FileTable::new(),
            data: Vec::new(),
        }
    }
}

impl Writable for NativeArchiveBuilder {
    fn add_file(&mut self, path: &str, asset: Vec<u8>) {
        let compressed = compress(&asset, Format::Zlib, CompressionLevel::Default).unwrap();

        let compressed_size = compressed.len() as u32;
        let compressed_size_aligned = compressed_size;
        let uncompressed_size = asset.len() as u32;
        let flags = 1;
        let offset = self.data.len() as u32;

        let file_information = FileTableRow {
            file_name: String::from(path),
            compressed_size,
            compressed_size_aligned,
            uncompressed_size,
            flags,
            offset,
        };

        self.data.extend_from_slice(&compressed);
        self.file_table.insert(String::from(path), file_information);
    }

    fn save(&self) {
        let file_table_offset = self.data.len() as u32;
        let reserved_files = 0;
        let raw_file_count = self.file_table.len() as u32 + 7;
        let version = 0x200;
        let file_header = Header::new(file_table_offset, reserved_files, raw_file_count, version);

        let mut bytes = Vec::new();

        bytes.extend_from_slice(MAGIC_BYTES);
        bytes.extend_from_slice(&file_header.to_bytes(None));
        bytes.extend_from_slice(&self.data);

        let mut file_table_data = Vec::new();

        for file_information in self.file_table.values() {
            file_table_data.extend_from_slice(&file_information.to_bytes(None));
        }

        let compressed_file_information_data = compress(&file_table_data, Format::Zlib, CompressionLevel::Default).unwrap();
        let file_table = AssetTable::new(compressed_file_information_data.len() as u32, file_table_data.len() as u32);

        bytes.extend_from_slice(&file_table.to_bytes(None));
        bytes.extend_from_slice(&compressed_file_information_data);

        std::fs::write(&self.os_file_path, bytes).expect("unable to write file");
    }
}
