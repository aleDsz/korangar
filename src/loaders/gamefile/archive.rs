use std::collections::HashMap;

use derive_new::new;
use procedural::ByteConvertable;
use yazi::{compress, decompress, CompressionLevel, Format};

#[cfg(feature = "debug")]
use crate::debug::*;
use crate::loaders::{ByteConvertable, ByteStream};

#[derive(Clone, ByteConvertable, new)]
struct FileHeader {
    #[new(default)]
    encryption: [u8; 14],
    file_table_offset: u32,
    reserved_files: u32,
    raw_file_count: u32,
    version: u32,
}

impl FileHeader {
    pub fn validate_version(&self) {
        assert_eq!(self.version, 0x200, "invalid grf version");
    }

    pub fn get_file_table_offset(&self) -> usize {
        self.file_table_offset as usize
    }

    pub fn get_file_count(&self) -> usize {
        (self.raw_file_count - self.reserved_files) as usize - 7
    }
}

#[derive(Clone, ByteConvertable, new)]
struct FileTable {
    compressed_size: u32,
    uncompressed_size: u32,
}

impl FileTable {
    pub fn get_compressed_size(&self) -> usize {
        self.compressed_size as usize
    }
}

#[derive(Clone, Debug, ByteConvertable)]
struct FileInformation {
    file_name: String,
    compressed_size: u32,
    compressed_size_aligned: u32,
    uncompressed_size: u32,
    flags: u8,
    offset: u32,
}

#[derive(Clone, Default)]
pub struct GameArchive {
    cache: HashMap<String, Vec<u8>>,
    files: HashMap<String, FileInformation>,
    data: Vec<u8>,
}

impl GameArchive {
    pub(super) fn load(path: &str, lua_files: &mut Vec<String>) -> Self {
        #[cfg(feature = "debug")]
        let timer = Timer::new_dynamic(format!("load game data from {MAGENTA}{path}{NONE}"));

        let bytes = std::fs::read(path).unwrap_or_else(|_| panic!("failed to load archive from {path}"));
        let mut byte_stream = ByteStream::new(&bytes);

        assert!(
            String::from_bytes(&mut byte_stream, Some(16)).as_str() == "Master of Magic",
            "failed to read magic number"
        ); // TODO: change failed to invalid

        let file_header = FileHeader::from_bytes(&mut byte_stream, None);
        file_header.validate_version();

        byte_stream.skip(file_header.get_file_table_offset());
        let file_table = FileTable::from_bytes(&mut byte_stream, None);

        let compressed = byte_stream.slice(file_table.get_compressed_size());
        let (decompressed, _checksum) = decompress(&compressed, Format::Zlib).unwrap();

        let file_count = file_header.get_file_count();

        let mut byte_stream = ByteStream::new(&decompressed);
        let mut files = HashMap::with_capacity(file_count);

        for _index in 0..file_count {
            let file_information = FileInformation::from_bytes(&mut byte_stream, None);
            let file_name = file_information.file_name.to_lowercase();

            if file_name.contains(".lub") {
                lua_files.push(file_name.clone());
            }

            files.insert(file_name, file_information);
        }

        #[cfg(feature = "debug")]
        timer.stop();

        // TODO: only take 64..? bytes so that loaded game archives can be extended
        // aswell
        Self {
            files,
            data: bytes,
            ..Default::default()
        }
    }

    pub(super) fn save(&mut self, file_name: &str) {
        let file_table_offset = self.data.len() as u32;
        let reserved_files = 0;
        let raw_file_count = self.files.len() as u32 + 7;
        let version = 0x200;
        let file_header = FileHeader::new(file_table_offset, reserved_files, raw_file_count, version);

        let mut bytes = Vec::new();

        // TODO: implement ByteConvertable for &str
        bytes.extend_from_slice(&"Master of Magic".to_string().to_bytes(None));
        bytes.extend_from_slice(&file_header.to_bytes(None));
        bytes.extend_from_slice(&self.data);

        let mut file_information_data = Vec::new();

        for file_information in self.files.values() {
            file_information_data.extend_from_slice(&file_information.to_bytes(None));
        }

        let compressed_file_information_data = compress(&file_information_data, Format::Zlib, CompressionLevel::Default).unwrap();
        let file_table = FileTable::new(
            compressed_file_information_data.len() as u32,
            file_information_data.len() as u32,
        );

        bytes.extend_from_slice(&file_table.to_bytes(None));
        bytes.extend_from_slice(&compressed_file_information_data);

        std::fs::write(file_name, bytes).expect("unable to write file");
    }

    fn load_data(&self, file_path: &str) -> Option<Vec<u8>> {
        let file_information = self.files.get(file_path)?;

        let mut byte_stream = ByteStream::new(&self.data);
        byte_stream.skip(file_information.offset as usize + 46);

        // TODO: Figure out what the GRF_FLAG_MIXCRYPT flag actually means and load the
        // file correctly
        if file_information.flags > 1 {
            return None;
        }

        let compressed = byte_stream.slice(file_information.compressed_size_aligned as usize);
        let (uncompressed, _checksum) = decompress(&compressed, Format::Zlib).unwrap();

        Some(uncompressed)
    }

    pub(super) fn get(&mut self, path: &str) -> Option<Vec<u8>> {
        match self.cache.get(path) {
            Some(data) => data.clone().into(),
            None => self.load_data(path),
        }
    }

    pub(super) fn add_file(&mut self, path: String, data: Vec<u8>) {
        let compressed = compress(&data, Format::Zlib, CompressionLevel::Default).unwrap();

        let file_name = path.clone();
        let compressed_size = compressed.len() as u32;
        let compressed_size_aligned = compressed_size;
        let uncompressed_size = data.len() as u32;
        let flags = 1;
        let offset = self.data.len() as u32;

        let file_information = FileInformation {
            file_name,
            compressed_size,
            compressed_size_aligned,
            uncompressed_size,
            flags,
            offset,
        };

        self.data.extend_from_slice(&compressed);
        self.files.insert(path, file_information);
    }
}
