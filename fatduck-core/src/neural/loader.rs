use crate::{pblczero, utils};
use flate2::read::GzDecoder;
use prost::Message;
use std::{env, ffi, fs, fs::File, io, io::Read, path::PathBuf};
use thiserror::Error;

pub(crate) struct WeightFile(pblczero::Net);

impl WeightFile {
    pub fn from_filepath(file_path: PathBuf) -> Result<Self, WeightFileError> {
        let buffer = Self::decompress_gzip(file_path)?;

        if buffer.len() < 2 {
            return Err(WeightFileError::TooSmall);
        }

        match (buffer.as_bytes()[0] as char, buffer.as_bytes()[1] as char) {
            ('1', '\n') => Err(WeightFileError::UnsupportedVersion("2".to_string())),
            ('2', '\n') => Err(WeightFileError::TextFormat),
            _ => Self::from_pb_buffer_string(buffer),
        }
    }

    pub fn discover_weights_file() -> Result<PathBuf, WeightFileError> {
        let current_dir = env::current_dir().map_err(WeightFileError::IoError)?;

        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(ffi::OsStr::new("pb.gz")) {
                return Ok(path);
            }
        }

        Err(WeightFileError::NoWeightFileFound)
    }

    // ParseWeightProto
    fn from_pb_buffer_string(pb_buffer: String) -> Result<Self, WeightFileError> {
        let net: pblczero::Net = Message::decode(pb_buffer.as_bytes())?;
        let weight_magic = 0x1c0_u32;

        if net.magic() != weight_magic {
            return Err(WeightFileError::BadHeader);
        }

        // TODO: Check if min_version is a required field
        let mv = net
            .min_version
            .clone()
            .ok_or(WeightFileError::MissingMinVersion)?;
        let min_version = utils::get_version_string(mv.major(), mv.major(), mv.patch(), "", "");
        let lc0_version = utils::get_version_int(0, 30, 0);
        let net_version = utils::get_version_int(mv.major(), mv.minor(), mv.patch());

        if net_version != 0x05c9_9972_u32 && net_version > lc0_version {
            return Err(WeightFileError::UnsupportedVersion(min_version));
        }

        if net.weights.is_some()
            && net
                .format
                .clone()
                .ok_or(WeightFileError::MissingFormat)?
                .weights_encoding()
                != pblczero::format::Encoding::Linear16
        {
            return Err(WeightFileError::UnsupportedWeightEncoding);
        }

        Ok(Self(net))
    }

    pub fn weights(&self) -> &Option<pblczero::Weights> {
        &self.0.weights
    }

    fn decompress_gzip(file_path: PathBuf) -> Result<String, WeightFileError> {
        let mut d = GzDecoder::new(File::open(file_path)?);
        let mut content = String::new();
        d.read_to_string(&mut content)?;
        Ok(content)
    }
}

#[derive(Error, Debug)]
pub enum WeightFileError {
    #[error("No files with extension '.pb.gz' found in the binary directory")]
    NoWeightFileFound,
    #[error("Invalid weight file: Bad Header")]
    BadHeader,
    #[error("Invalid weight file: Unsupported encoding")]
    UnsupportedWeightEncoding,
    #[error("Invalid weight file: Lc0 version >= {0} required")]
    UnsupportedVersion(String),
    #[error("Invalid weight file: Too small")]
    TooSmall,
    #[error("Invalid weight file: Text format weight files are no longer supported. Use a command line tool to convert it to the new format")]
    TextFormat,
    #[error("Invalid weight file: Missing min_version field")]
    MissingMinVersion,
    #[error("Invalid weight file: Missing format field")]
    MissingFormat,
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ProtobufError(#[from] prost::DecodeError),
}
