use bzip2::bufread::BzDecoder;
use serde::Deserialize;
use sha1_smol::Sha1;
use std::{
    fmt::{self, Display},
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

#[macro_use]
extern crate thiserror;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported target triplet: {0}")]
    UnsupportedTarget(String),
    #[error("HTTP request error: {0}")]
    Request(#[from] ureq::Error),
    #[error("Version not found: {0}")]
    VersionNotFound(String),
    #[error("Missing Content-Length header")]
    MissingContentLength,
    #[error("Invalid Content-Length header: {0}")]
    InvalidContentLength(String),
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unexpected file size: downloaded {downloaded} expected {expected}")]
    UnexpectedFileSize { downloaded: u64, expected: u64 },
    #[error("Bad SHA1 file hash: {0}")]
    CorruptedFile(String),
    #[error("Invalid archive file path: {0}")]
    InvalidArchiveFile(String),
}

pub type Result<T> = std::result::Result<T, Error>;

const DOWNLOAD_TEMPLATE: &str = "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

pub const LINUX_TARGETS: &[&str] = &[
    "x86_64-unknown-linux-gnu",
    "arm-unknown-linux-gnueabi",
    "aarch64-unknown-linux-gnu",
];

pub const MACOS_TARGETS: &[&str] = &["aarch64-apple-darwin", "x86_64-apple-darwin"];

pub const WINDOWS_TARGETS: &[&str] = &[
    "aarch64-pc-windows-msvc",
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
];

const URL: &str = "https://cef-builds.spotifycdn.com";

#[derive(Deserialize)]
struct CefIndex {
    macosarm64: CefPlatform,
    macosx64: CefPlatform,
    windows32: CefPlatform,
    windows64: CefPlatform,
    windowsarm64: CefPlatform,
    linux64: CefPlatform,
    linuxarm: CefPlatform,
    linuxarm64: CefPlatform,
}

impl CefIndex {
    fn platform(&self, target: &str) -> Result<&CefPlatform> {
        match target {
            "aarch64-apple-darwin" => Ok(&self.macosarm64),
            "x86_64-apple-darwin" => Ok(&self.macosx64),
            "i686-pc-windows-msvc" => Ok(&self.windows32),
            "x86_64-pc-windows-msvc" => Ok(&self.windows64),
            "aarch64-pc-windows-msvc" => Ok(&self.windowsarm64),
            "x86_64-unknown-linux-gnu" => Ok(&self.linux64),
            "arm-unknown-linux-gnueabi" => Ok(&self.linuxarm),
            "aarch64-unknown-linux-gnu" => Ok(&self.linuxarm64),
            v => Err(Error::UnsupportedTarget(v.to_string())),
        }
    }
}

#[derive(Deserialize)]
struct CefPlatform {
    versions: Vec<CefVersion>,
}

#[derive(Deserialize)]
struct CefVersion {
    channel: String,
    cef_version: String,
    files: Vec<CefFile>,
}

#[derive(Deserialize)]
struct CefFile {
    #[serde(rename = "type")]
    file_type: String,
    name: String,
    sha1: String,
}

pub fn download_target_archive<P>(
    target: &str,
    cef_version: &str,
    location: P,
    show_progress: bool,
) -> Result<PathBuf>
where
    P: AsRef<Path>,
{
    if show_progress {
        println!("Downloading CEF archive for {target}...");
    }

    let index: CefIndex = ureq::get(&format!("{URL}/index.json"))
        .call()?
        .into_json()?;
    let platform = index.platform(target)?;
    let version_prefix = format!("{cef_version}+");

    let (file, sha) = platform
        .versions
        .iter()
        .find_map(|v| {
            if v.channel == "stable" && v.cef_version.starts_with(&version_prefix) {
                v.files.iter().find_map(|f| {
                    if f.file_type == "minimal" {
                        Some((f.name.as_str(), f.sha1.as_str()))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .ok_or_else(|| Error::VersionNotFound(cef_version.to_string()))?;

    fs::create_dir_all(&location)?;
    let download_file = location.as_ref().join(file);

    if download_file.exists() {
        if calculate_file_sha1(&download_file) == sha {
            if show_progress {
                println!("Verified archive: {}", download_file.display());
            }
            return Ok(download_file);
        }

        if show_progress {
            println!("Cleaning corrupted archive: {}", download_file.display());
        }
        let corrupted_file = location.as_ref().join(format!("corrupted_{file}"));
        fs::rename(&download_file, &corrupted_file)?;
        fs::remove_file(&corrupted_file)?;
    }

    let cef_url = format!("{URL}/{file}");
    if show_progress {
        println!("Using archive url: {cef_url}");
    }

    let mut file = File::create(&download_file)?;

    let resp = ureq::get(&cef_url).call()?;
    let expected = resp
        .header("Content-Length")
        .ok_or(Error::MissingContentLength)?;
    let expected = expected
        .parse::<u64>()
        .map_err(|_| Error::InvalidContentLength(expected.to_owned()))?;

    let downloaded = if show_progress {
        let bar = indicatif::ProgressBar::new(expected);
        bar.set_style(
            indicatif::ProgressStyle::with_template(DOWNLOAD_TEMPLATE)
                .expect("invalid template")
                .progress_chars("##-"),
        );
        bar.set_message("Downloading");
        std::io::copy(&mut bar.wrap_read(resp.into_reader()), &mut file)
    } else {
        let mut reader = resp.into_reader();
        std::io::copy(&mut reader, &mut file)
    }?;

    if downloaded != expected {
        return Err(Error::UnexpectedFileSize {
            downloaded,
            expected,
        });
    }

    if show_progress {
        println!("Verifying SHA1 hash: {sha}...");
    }
    if calculate_file_sha1(&download_file) != sha {
        return Err(Error::CorruptedFile(download_file.display().to_string()));
    }

    if show_progress {
        println!("Downloaded archive: {}", download_file.display());
    }
    Ok(download_file)
}

pub fn extract_target_archive<P, Q>(
    target: &str,
    archive: P,
    location: Q,
    show_progress: bool,
) -> Result<PathBuf>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    if show_progress {
        println!("Extracting archive: {}", archive.as_ref().display());
    }
    let decoder = BzDecoder::new(BufReader::new(File::open(&archive)?));
    tar::Archive::new(decoder).unpack(&location)?;

    let extracted_dir = archive.as_ref().display().to_string();
    let extracted_dir = extracted_dir
        .strip_suffix(".tar.bz2")
        .map(PathBuf::from)
        .ok_or(Error::InvalidArchiveFile(extracted_dir))?;

    let os_and_arch = OsAndArch::try_from(target)?;
    let OsAndArch { os, arch } = os_and_arch;
    let cef_dir = os_and_arch.to_string();
    let cef_dir = location.as_ref().join(cef_dir);

    if cef_dir.exists() {
        let old_dir = location.as_ref().join(format!("old_{os}_{arch}"));
        if show_progress {
            println!("Cleaning up: {}", old_dir.display());
        }
        fs::rename(&cef_dir, &old_dir)?;
        fs::remove_dir_all(old_dir)?;
    }
    const RELEASE_DIR: &str = "Release";
    fs::rename(extracted_dir.join(RELEASE_DIR), &cef_dir)?;

    if os != "macos" {
        let resources = extracted_dir.join("Resources");

        for entry in fs::read_dir(&resources)? {
            let entry = entry?;
            fs::rename(entry.path(), cef_dir.join(entry.file_name()))?;
        }
    }

    const CMAKE_LISTS_TXT: &str = "CMakeLists.txt";
    fs::rename(
        extracted_dir.join(CMAKE_LISTS_TXT),
        cef_dir.join(CMAKE_LISTS_TXT),
    )?;
    const CMAKE_DIR: &str = "cmake";
    fs::rename(extracted_dir.join(CMAKE_DIR), cef_dir.join(CMAKE_DIR))?;
    const INCLUDE_DIR: &str = "include";
    fs::rename(extracted_dir.join(INCLUDE_DIR), cef_dir.join(INCLUDE_DIR))?;
    const LIBCEF_DLL_DIR: &str = "libcef_dll";
    fs::rename(
        extracted_dir.join(LIBCEF_DLL_DIR),
        cef_dir.join(LIBCEF_DLL_DIR),
    )?;

    if show_progress {
        println!("Moved contents to: {}", cef_dir.display());
    }

    // Cleanup whatever is left in the extracted directory.
    let old_dir = extracted_dir
        .parent()
        .map(|parent| parent.join(format!("extracted_{os}_{arch}")))
        .ok_or_else(|| Error::InvalidArchiveFile(extracted_dir.display().to_string()))?;
    if show_progress {
        println!("Cleaning up: {}", old_dir.display());
    }
    fs::rename(&extracted_dir, &old_dir)?;
    fs::remove_dir_all(old_dir)?;

    Ok(cef_dir)
}

fn calculate_file_sha1(path: &Path) -> String {
    use std::io::Read;
    let mut file = BufReader::new(File::open(path).unwrap());
    let mut sha1 = Sha1::new();
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        sha1.update(&buffer[..count]);
    }

    sha1.digest().to_string()
}

pub struct OsAndArch {
    pub os: &'static str,
    pub arch: &'static str,
}

impl Display for OsAndArch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let os = self.os;
        let arch = self.arch;
        write!(f, "cef_{os}_{arch}")
    }
}

impl TryFrom<&str> for OsAndArch {
    type Error = Error;

    fn try_from(target: &str) -> Result<Self> {
        match target {
            "aarch64-apple-darwin" => Ok(OsAndArch {
                os: "macos",
                arch: "aarch64",
            }),
            "x86_64-apple-darwin" => Ok(OsAndArch {
                os: "macos",
                arch: "x86_64",
            }),
            "x86_64-pc-windows-msvc" => Ok(OsAndArch {
                os: "windows",
                arch: "x86_64",
            }),
            "aarch64-pc-windows-msvc" => Ok(OsAndArch {
                os: "windows",
                arch: "aarch64",
            }),
            "i686-pc-windows-msvc" => Ok(OsAndArch {
                os: "windows",
                arch: "x86",
            }),
            "x86_64-unknown-linux-gnu" => Ok(OsAndArch {
                os: "linux",
                arch: "x86_64",
            }),
            "aarch64-unknown-linux-gnu" => Ok(OsAndArch {
                os: "linux",
                arch: "aarch64",
            }),
            "arm-unknown-linux-gnueabi" => Ok(OsAndArch {
                os: "linux",
                arch: "arm",
            }),
            v => Err(Error::UnsupportedTarget(v.to_string())),
        }
    }
}
