use camino::{Utf8Path, Utf8PathBuf};
use flate2::write::GzEncoder;
use flate2::Compression;
use glob::glob;
use phf_codegen::Map;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::process::Command;

#[cfg(windows)]
static NPM_CMD: &str = "npm.cmd";
#[cfg(not(windows))]
static NPM_CMD: &str = "npm";

static RUSTFMT_CMD: &str = "rustfmt";

fn install_npm_dependencies(path: &Utf8Path) {
    let root_path = path.join("..").join("..");
    println!("Installing npm dependencies for libraries/web");
    let npm_exit_status = Command::new(NPM_CMD)
        .args(["install"])
        .current_dir(root_path)
        .status()
        .unwrap();

    if !npm_exit_status.success() {
        panic!("Failed to install npm dependencies for libraries/web")
    }
}

fn build_npm(path: &Utf8Path) {
    println!("Building npm project for libraries/lib-web.");
    let npm_exit_status = Command::new(NPM_CMD)
        .args(["run", "build"])
        .current_dir(path)
        .status()
        .unwrap();

    if !npm_exit_status.success() {
        panic!("Failed to build npm portion of libraries/lib-web")
    }
}

fn get_uncompressed_data(path: &Utf8PathBuf) -> Vec<u8> {
    let metadata = path.metadata().expect("Unable to read metadata.");
    let mut file_data = vec![0; metadata.len() as usize];
    let mut file = File::open(path).expect("Unable to open file.");
    let _ = file.read(&mut file_data).expect("File buffer overflow.");

    file_data
}

fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

fn generate_dist_map(path: &Utf8Path) {
    let input_path = path.join("dist");
    let input_path_str = input_path.as_str().to_owned() + "/";

    let output_directory_string = env::var("OUT_DIR").unwrap();
    let output_directory = Utf8Path::new(&output_directory_string);
    let output_path = output_directory.join("web_codegen.rs");
    {
        let mut output_file =
            BufWriter::new(File::create(output_path).expect("Failed to open output file."));

        let mut map = Map::new();

        let mut index_string = String::new();

        for path in glob(&format!("{}/**/*", input_path_str))
            .expect("Failed to read glob pattern.")
            .flatten()
        {
            let local_path = Utf8PathBuf::from_path_buf(path).expect("Invalid UTF-8 path.");
            if local_path.is_dir() {
                continue;
            }

            let file_data_uncompressed = get_uncompressed_data(&local_path);
            let file_data_compressed = compress_data(&file_data_uncompressed);
            let mime_type = mime_guess::from_path(local_path.clone()).first_or_octet_stream();

            let s = format!(
                "Resource {{data_uncompressed: &{:?}, data_gzip: &{:?}, mime_type: &{:?}}}",
                file_data_uncompressed, file_data_compressed, mime_type
            );

            let reduced_path = local_path.to_string().replace(input_path_str.as_str(), "");
            if reduced_path == "index.html" {
                index_string = s.clone();
            }
            map.entry(reduced_path, s.as_str());
        }

        writeln!(&mut output_file, "pub struct Resource<'a> {{").unwrap();
        writeln!(&mut output_file, "    pub data_uncompressed: &'a [u8],").unwrap();
        writeln!(&mut output_file, "    pub data_gzip: &'a [u8],").unwrap();
        writeln!(&mut output_file, "    pub mime_type: &'a str,").unwrap();
        writeln!(&mut output_file, "}}").unwrap();

        if !index_string.is_empty() {
            writeln!(&mut output_file).unwrap();
            write!(
                &mut output_file,
                "static INDEX_DATA: Resource<'static> = {};",
                index_string
            )
            .unwrap();
        }

        writeln!(&mut output_file).unwrap();
        write!(
            &mut output_file,
            "static FILES: phf::Map<&'static str, Resource<'static>> = {}",
            map.build()
        )
        .unwrap();
        writeln!(&mut output_file, ";").unwrap();
    }
    println!("Formatting output files");
    let rustfmt_exit_status = Command::new(RUSTFMT_CMD)
        .args(["web_codegen.rs"])
        .current_dir(output_directory)
        .status()
        .unwrap();

    if !rustfmt_exit_status.success() {
        panic!("Failed to format output of libraries/lib-web")
    }
}

fn main() {
    println!("cargo:rerun-if-changed=public");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=index.html");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=tsconfig.app.json");
    println!("cargo:rerun-if-changed=tsconfig.json");
    println!("cargo:rerun-if-changed=vite.config.ts");

    let current_path_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let current_path = Utf8Path::new(&current_path_string);
    install_npm_dependencies(current_path);
    build_npm(current_path);
    generate_dist_map(current_path);
}
