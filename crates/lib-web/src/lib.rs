pub struct Resource {
    pub data_uncompressed: &'static [u8],
    pub data_gzip: &'static [u8],
    pub mime_type: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/web_codegen.rs"));

#[cfg(feature = "list")]
use itertools::Itertools;

pub fn get_file_data(file_path: &str) -> Option<&'static Resource> {
    if let Some(file_data) = FILES.get(file_path) {
        Some(file_data)
    } else {
        None
    }
}

pub fn get_index_data() -> &'static Resource {
    &INDEX_DATA
}

#[cfg(feature = "list")]
pub fn get_file_list() -> Vec<&'static &'static str> {
    FILES.keys().sorted().collect_vec()
}
