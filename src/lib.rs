use std::io;

use io::Read;
use io::Seek;

use io::BufWriter;
use io::Write;

use std::fs::File;

use der::asn1::OctetString;

pub use der;
pub use zip;

use zip::ZipArchive;
use zip::read::ZipFile;

use der::Decode;

#[derive(Debug, PartialEq, Eq, Clone, Copy, der::Enumerated)]
#[repr(u8)]
pub enum CompressionMethod {
    Unspecified = 0,
    Store = 100,
    Deflate = 108,
}

impl Default for CompressionMethod {
    fn default() -> Self {
        Self::Unspecified
    }
}

#[derive(Default, der::Sequence)]
pub struct ZipMeta {
    pub filename: String,
    pub comment: String,
    pub modified_unixtime: u32,
    pub compression: CompressionMethod,
    pub is_dir: bool,
}

#[derive(der::Sequence)]
pub struct ZipItem {
    pub meta: ZipMeta,
    pub data: OctetString,
}

impl ZipItem {
    pub fn slice2items(s: &[u8]) -> Result<Vec<Self>, io::Error> {
        <Vec<Self> as Decode>::from_der(s).map_err(io::Error::other)
    }

    pub fn rdr2items<R>(rdr: &mut R, buf: &mut Vec<u8>) -> Result<Vec<Self>, io::Error>
    where
        R: Read,
    {
        buf.clear();
        io::copy(rdr, buf)?;
        Self::slice2items(buf)
    }
}

impl ZipItem {
    pub fn items2data2concat2writer<W>(items: &[Self], wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        for item in items {
            let dat: &[u8] = item.data.as_bytes();
            wtr.write_all(dat)?;
        }
        Ok(())
    }
}

pub fn zip2files2items2writer<R, W>(
    mut z: ZipArchive<R>,
    mut wtr: W,
    buf: &mut Vec<u8>,
) -> Result<(), io::Error>
where
    R: Read + Seek,
    W: Write,
{
    let sz: usize = z.len();
    for i in 0..sz {
        let mut zfile: ZipFile<_> = z.by_index(i).map_err(io::Error::other)?;
        let items: Vec<ZipItem> = ZipItem::rdr2items(&mut zfile, buf)?;
        ZipItem::items2data2concat2writer(&items, &mut wtr)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn zipfile2items2stdout(zfile: File) -> Result<(), io::Error> {
    let za: ZipArchive<_> = ZipArchive::new(zfile).map_err(io::Error::other)?;
    let mut buf: Vec<u8> = vec![];

    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    zip2files2items2writer(za, bw, &mut buf)?;

    ol.flush()
}
