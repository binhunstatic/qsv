use std::{
    borrow::ToOwned,
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use log::{debug, info, warn};
use qsv_sniffer::{SampleSize, Sniffer};
use serde::de::{Deserialize, Deserializer, Error};

use crate::{
    index::Indexed,
    select::{SelectColumns, Selection},
    util, CliResult,
};

// rdr default is 8k in csv crate, we're doubling it
const DEFAULT_RDR_BUFFER_CAPACITY: usize = 16 * (1 << 10);
// previous wtr default in xsv is 32k, we're doubling it
pub const DEFAULT_WTR_BUFFER_CAPACITY: usize = 64 * (1 << 10);

// number of rows for qsv_sniffer to sample
const DEFAULT_SNIFFER_SAMPLE: usize = 100;

// for files, number of bytes to check for UTF8 encoding
const DEFAULT_UTF8_CHECK_BUFFER_LEN: usize = 8192;
const UTF8_ERROR_MSG: &str = "is not UTF-8 encoded. Use the input command to transcode to UTF-8.";

// file size at which we warn user that a large file has not been indexed
const NO_INDEX_WARNING_FILESIZE: u64 = 100_000_000; // 100MB

#[derive(Clone, Copy, Debug)]
pub struct Delimiter(pub u8);

/// Delimiter represents values that can be passed from the command line that
/// can be used as a field delimiter in CSV data.
///
/// Its purpose is to ensure that the Unicode character given decodes to a
/// valid ASCII character as required by the CSV parser.
impl Delimiter {
    pub const fn as_byte(self) -> u8 {
        self.0
    }

    fn decode_delimiter(s: &str) -> Result<Delimiter, String> {
        if s == r"\t" {
            return Ok(Delimiter(b'\t'));
        }

        if s.len() != 1 {
            return fail_format!("Could not convert '{s}' to a single ASCII character.");
        }

        let c = s.chars().next().unwrap();
        if c.is_ascii() {
            Ok(Delimiter(c as u8))
        } else {
            fail_format!("Could not convert '{c}' to ASCII delimiter.")
        }
    }
}

impl<'de> Deserialize<'de> for Delimiter {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Delimiter, D::Error> {
        let s = String::deserialize(d)?;
        match Delimiter::decode_delimiter(&s) {
            Ok(delim) => Ok(delim),
            Err(msg) => Err(D::Error::custom(msg)),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    path:              Option<PathBuf>, // None implies <stdin>
    idx_path:          Option<PathBuf>,
    select_columns:    Option<SelectColumns>,
    delimiter:         u8,
    pub no_headers:    bool,
    flexible:          bool,
    terminator:        csv::Terminator,
    pub quote:         u8,
    quote_style:       csv::QuoteStyle,
    double_quote:      bool,
    escape:            Option<u8>,
    quoting:           bool,
    pub preamble_rows: u64,
    trim:              csv::Trim,
    autoindex:         bool,
    checkutf8:         bool,
    prefer_dmy:        bool,
}

// Empty trait as an alias for Seek and Read that avoids auto trait errors
pub trait SeekRead: io::Seek + io::Read {}
impl<T: io::Seek + io::Read> SeekRead for T {}

impl Config {
    pub fn new(path: &Option<String>) -> Config {
        let default_delim = match env::var("QSV_DEFAULT_DELIMITER") {
            Ok(delim) => Delimiter::decode_delimiter(&delim).unwrap().as_byte(),
            _ => b',',
        };
        let (path, mut delim) = match *path {
            None => (None, default_delim),
            Some(ref s) if &**s == "-" => (None, default_delim),
            Some(ref s) => {
                let path = PathBuf::from(s);
                let file_extension = path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_lowercase();
                let delim = if file_extension == "tsv" || file_extension == "tab" {
                    b'\t'
                } else if file_extension == "csv" {
                    b','
                } else {
                    default_delim
                };
                (Some(path), delim)
            }
        };
        let sniff =
            env::var("QSV_SNIFF_DELIMITER").is_ok() || env::var("QSV_SNIFF_PREAMBLE").is_ok();
        let mut preamble = 0_u64;
        if sniff && path.is_some() {
            let sniff_path = path.as_ref().unwrap().to_str().unwrap();

            match Sniffer::new()
                .sample_size(SampleSize::Records(DEFAULT_SNIFFER_SAMPLE))
                .sniff_path(sniff_path)
            {
                Ok(metadata) => {
                    delim = metadata.dialect.delimiter;
                    preamble = metadata.dialect.header.num_preamble_rows as u64;
                    info!(
                        "sniffed delimiter {} and {preamble} preamble rows",
                        delim as char
                    );
                }
                Err(e) => {
                    // we only warn, as we don't want to stop processing the file
                    // if sniffing doesn't work
                    warn!("sniff error: {e}");
                }
            }
        }

        Config {
            path,
            idx_path: None,
            select_columns: None,
            delimiter: delim,
            no_headers: false,
            flexible: false,
            terminator: csv::Terminator::Any(b'\n'),
            quote: b'"',
            quote_style: csv::QuoteStyle::Necessary,
            double_quote: true,
            escape: None,
            quoting: true,
            preamble_rows: preamble,
            trim: csv::Trim::None,
            autoindex: env::var("QSV_AUTOINDEX").is_ok(),
            checkutf8: env::var("QSV_SKIPUTF8_CHECK").is_err(),
            prefer_dmy: env::var("QSV_PREFER_DMY").is_ok(),
        }
    }

    pub const fn delimiter(mut self, d: Option<Delimiter>) -> Config {
        if let Some(d) = d {
            self.delimiter = d.as_byte();
        }
        self
    }

    pub const fn get_delimiter(&self) -> u8 {
        self.delimiter
    }

    pub const fn get_dmy_preference(&self) -> bool {
        self.prefer_dmy
    }

    pub fn no_headers(mut self, mut yes: bool) -> Config {
        if env::var("QSV_TOGGLE_HEADERS").unwrap_or_else(|_| "0".to_owned()) == "1" {
            yes = !yes;
        }
        if env::var("QSV_NO_HEADERS").is_ok() {
            self.no_headers = true;
        } else {
            self.no_headers = yes;
        }
        self
    }

    pub const fn flexible(mut self, yes: bool) -> Config {
        self.flexible = yes;
        self
    }

    #[cfg(any(feature = "full", feature = "lite"))]
    pub const fn crlf(mut self, yes: bool) -> Config {
        if yes {
            self.terminator = csv::Terminator::CRLF;
        } else {
            self.terminator = csv::Terminator::Any(b'\n');
        }
        self
    }

    #[cfg(any(feature = "full", feature = "lite"))]
    pub const fn terminator(mut self, term: csv::Terminator) -> Config {
        self.terminator = term;
        self
    }

    pub const fn quote(mut self, quote: u8) -> Config {
        self.quote = quote;
        self
    }

    #[cfg(any(feature = "full", feature = "lite"))]
    pub const fn quote_style(mut self, style: csv::QuoteStyle) -> Config {
        self.quote_style = style;
        self
    }

    pub const fn double_quote(mut self, yes: bool) -> Config {
        self.double_quote = yes;
        self
    }

    pub const fn escape(mut self, escape: Option<u8>) -> Config {
        self.escape = escape;
        self
    }

    pub const fn quoting(mut self, yes: bool) -> Config {
        self.quoting = yes;
        self
    }

    pub const fn trim(mut self, trim_type: csv::Trim) -> Config {
        self.trim = trim_type;
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn select(mut self, sel_cols: SelectColumns) -> Config {
        self.select_columns = Some(sel_cols);
        self
    }

    pub const fn is_stdin(&self) -> bool {
        self.path.is_none()
    }

    pub const fn checkutf8(mut self, yes: bool) -> Config {
        self.checkutf8 = yes;
        self
    }

    pub fn selection(&self, first_record: &csv::ByteRecord) -> Result<Selection, String> {
        match self.select_columns {
            None => fail!("Config has no 'SelectColumns'. Did you call Config::select?"),
            Some(ref sel) => sel.selection(first_record, !self.no_headers),
        }
    }

    pub fn write_headers<R: io::Read, W: io::Write>(
        &self,
        r: &mut csv::Reader<R>,
        w: &mut csv::Writer<W>,
    ) -> csv::Result<()> {
        if !self.no_headers {
            let r = r.byte_headers()?;
            if !r.is_empty() {
                w.write_record(r)?;
            }
        }
        Ok(())
    }

    pub fn writer(&self) -> io::Result<csv::Writer<Box<dyn io::Write + 'static>>> {
        Ok(self.from_writer(self.io_writer()?))
    }

    pub fn reader(&self) -> io::Result<csv::Reader<Box<dyn io::Read + Send + 'static>>> {
        Ok(self.from_reader(self.io_reader()?))
    }

    pub fn reader_file(&self) -> io::Result<csv::Reader<fs::File>> {
        match self.path {
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot use <stdin> here",
            )),
            Some(ref p) => {
                if !self.is_utf8_encoded()? {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{p:?} {UTF8_ERROR_MSG}"),
                    ));
                }
                fs::File::open(p).map(|f| self.from_reader(f))
            }
        }
    }

    pub fn reader_file_stdin(&self) -> io::Result<csv::Reader<Box<dyn SeekRead + 'static>>> {
        Ok(match self.path {
            None => {
                // Create a buffer in memory when stdin needs to be indexed
                let mut buffer: Vec<u8> = Vec::new();
                let stdin = io::stdin();
                stdin.lock().read_to_end(&mut buffer)?;
                // check if its utf8-encoded
                if self.checkutf8 {
                    debug!("checking stdin encoding...");
                    // get first 8k of buffer
                    if buffer.is_empty() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "<stdin> is empty!".to_string(),
                        ));
                    }
                    let buffer_check = buffer
                        .chunks_exact(std::cmp::min(DEFAULT_UTF8_CHECK_BUFFER_LEN, buffer.len()))
                        .next()
                        .unwrap();
                    let s = std::str::from_utf8(buffer_check);
                    if s.is_err() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("<stdin> {UTF8_ERROR_MSG}"),
                        ));
                    }
                }
                self.from_reader(Box::new(io::Cursor::new(buffer)))
            }
            Some(ref p) => {
                if !self.is_utf8_encoded()? {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{p:?} {UTF8_ERROR_MSG}"),
                    ));
                }
                self.from_reader(Box::new(fs::File::open(p).unwrap()))
            }
        })
    }

    // qsv only works safely with utf8 encoded files
    // check first DEFAULT_UTF8_CHECK_BUFFER_LEN bytes
    // of file to quickly check if its utf8
    fn is_utf8_encoded(&self) -> io::Result<bool> {
        if !self.checkutf8 {
            return Ok(true);
        }
        if let Some(path_buf) = &self.path {
            debug!("checking encoding...");
            let mut f = match fs::File::open(path_buf) {
                Ok(x) => x,
                Err(err) => {
                    let msg = format!("failed to open {}: {err}", path_buf.display());
                    return Err(io::Error::new(io::ErrorKind::NotFound, msg));
                }
            };

            let fsize = f.metadata().unwrap().len() as usize;
            let mut buffer = vec![0; std::cmp::min(DEFAULT_UTF8_CHECK_BUFFER_LEN, fsize)];
            if f.read_exact(&mut buffer).is_ok() {
                let s = std::str::from_utf8(&buffer);
                return Ok(s.is_ok());
            }
        }
        Ok(false)
    }

    fn autoindex_file(&self) {
        use io::prelude::*;

        // autoindex_file should never panic. It should silently fail as its a "convenience fn"
        // that's why we have a lot of let-else returns, in lieu of unwraps
        let Some(path_buf) = &self.path else { return };

        let pidx = util::idx_path(Path::new(path_buf));
        let Ok(idxfile) = fs::File::create(pidx) else { return };
        let Ok(mut rdr) = self.reader_file() else { return };
        let mut wtr = io::BufWriter::new(idxfile);
        match csv_index::RandomAccessSimple::create(&mut rdr, &mut wtr) {
            Ok(_) => {
                let Ok(_) = wtr.flush() else { return };
                debug!("autoindex of {path_buf:?} successful.");
            }
            Err(e) => debug!("autoindex of {path_buf:?} failed: {e}"),
        }
    }

    pub fn index_files(&self) -> io::Result<Option<(csv::Reader<fs::File>, fs::File)>> {
        let (csv_file, idx_file) = match (&self.path, &self.idx_path) {
            (&None, &None) => return Ok(None),
            (&None, &Some(_)) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Cannot use <stdin> with indexes",
                ));
            }
            (Some(p), &None) => {
                // We generally don't want to report an error here, since we're
                // passively trying to find an index, so we just log the warning...
                let idx_file = match fs::File::open(util::idx_path(p)) {
                    Err(e) => {
                        if self.autoindex {
                            // however, if QSV_AUTOINDEX is set, we create the index automatically
                            self.autoindex_file();
                            fs::File::open(util::idx_path(p)).unwrap()
                        } else {
                            warn!("No index file found - {p:?}: {e}");
                            return Ok(None);
                        }
                    }
                    Ok(f) => f,
                };
                (fs::File::open(p)?, idx_file)
            }
            (Some(p), Some(ip)) => (fs::File::open(p)?, fs::File::open(ip)?),
        };
        // If the CSV data was last modified after the index file was last
        // modified, then return an error and demand the user regenerate the index.
        // Unless QSV_AUTOINDEX is set, in which case, we'll recreate the
        // stale index automatically
        let (data_modified, data_fsize) = util::file_metadata(&csv_file.metadata()?);
        let (idx_modified, _) = util::file_metadata(&idx_file.metadata()?);
        if data_modified > idx_modified {
            if self.autoindex {
                info!("index stale... autoindexing...");
                self.autoindex_file();
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The CSV file was modified after the index file. Please re-create the index.",
                ));
            }
        }
        // If the CSV file is larger than NO_INDEX_WARNING_FILESIZE,
        // log a warning that the user should consider creating an index file for faster access.
        if data_fsize > NO_INDEX_WARNING_FILESIZE {
            use thousands::Separable;

            warn!(
                "The {} MB CSV file is larger than the {} MB NO_INDEX_WARNING_FILESIZE threshold. \
                 Consider creating an index file for faster access.",
                (data_fsize * 100).separate_with_commas(),
                (NO_INDEX_WARNING_FILESIZE * 100).separate_with_commas()
            );
        }
        let csv_rdr = self.from_reader(csv_file);
        Ok(Some((csv_rdr, idx_file)))
    }

    pub fn indexed(&self) -> CliResult<Option<Indexed<fs::File, fs::File>>> {
        match self.index_files()? {
            None => Ok(None),
            Some((r, i)) => Ok(Some(Indexed::open(r, i)?)),
        }
    }

    pub fn io_reader(&self) -> io::Result<Box<dyn io::Read + Send + 'static>> {
        Ok(match self.path {
            None => {
                if self.checkutf8 {
                    let stdin_reader = io::stdin();
                    let mut buffer: Vec<u8> = Vec::new();
                    stdin_reader.lock().read_to_end(&mut buffer)?;
                    if buffer.is_empty() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "<stdin> is empty!".to_string(),
                        ));
                    }
                    // check if its utf8-encoded
                    let buffer_check = buffer
                        .chunks_exact(std::cmp::min(DEFAULT_UTF8_CHECK_BUFFER_LEN, buffer.len()))
                        .next()
                        .unwrap();
                    let s = std::str::from_utf8(buffer_check);
                    if s.is_err() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("<stdin> {UTF8_ERROR_MSG}"),
                        ));
                    }
                    Box::new(io::Cursor::new(buffer))
                } else {
                    Box::new(io::stdin())
                }
            }
            Some(ref p) => {
                if !self.is_utf8_encoded()? {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{p:?} {UTF8_ERROR_MSG}"),
                    ));
                }
                match fs::File::open(p) {
                    Ok(x) => Box::new(x),
                    Err(err) => {
                        let msg = format!("failed to open {}: {err}", p.display());
                        return Err(io::Error::new(io::ErrorKind::NotFound, msg));
                    }
                }
            }
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_reader<R: Read>(&self, rdr: R) -> csv::Reader<R> {
        let rdr_capacitys = env::var("QSV_RDR_BUFFER_CAPACITY")
            .unwrap_or_else(|_| DEFAULT_RDR_BUFFER_CAPACITY.to_string());
        let rdr_buffer: usize = rdr_capacitys.parse().unwrap_or(DEFAULT_RDR_BUFFER_CAPACITY);

        let rdr_comment: Option<u8> = env::var("QSV_COMMENT_CHAR")
            .ok()
            .map(|s| s.as_bytes().first().unwrap().to_owned());

        csv::ReaderBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .has_headers(!self.no_headers)
            .quote(self.quote)
            .quoting(self.quoting)
            .escape(self.escape)
            .buffer_capacity(rdr_buffer)
            .comment(rdr_comment)
            .trim(self.trim)
            .from_reader(rdr)
    }

    pub fn io_writer(&self) -> io::Result<Box<dyn io::Write + 'static>> {
        Ok(match self.path {
            None => Box::new(io::stdout()),
            Some(ref p) => {
                let p_str = p.as_os_str();
                if p_str == "sink" {
                    // sink is /dev/null
                    Box::new(io::sink())
                } else {
                    Box::new(fs::File::create(p)?)
                }
            }
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_writer<W: io::Write>(&self, wtr: W) -> csv::Writer<W> {
        let wtr_capacitys = env::var("QSV_WTR_BUFFER_CAPACITY")
            .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string());
        let wtr_buffer: usize = wtr_capacitys.parse().unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY);

        csv::WriterBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .terminator(self.terminator)
            .quote(self.quote)
            .quote_style(self.quote_style)
            .double_quote(self.double_quote)
            .escape(self.escape.unwrap_or(b'\\'))
            .buffer_capacity(wtr_buffer)
            .from_writer(wtr)
    }
}
