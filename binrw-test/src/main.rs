use binrw::{binrw, io::Cursor, BinRead, BinWrite, BinWriterExt};
use encoding::{all::ASCII, DecoderTrap, EncoderTrap, Encoding};
use std::fmt;

#[derive(Debug)]
#[binrw]
#[br(little)]
struct TerrariaServerList {
    version: u32,

    // 10 entries, [00 00 00 00 00 00] if empty (null name, null url, port = 0)
    #[br(count = 10)]
    servers: Vec<TerrariaServerListEntry>,
}

#[derive(Debug)]
#[binrw]
#[br(little)]
struct TerrariaServerListEntry {
    /*name_length: u8,
    #[br(count = name_length)]
    name: Vec<u8>,*/
    //
    //#[br(parse_with = read_string)]
    //#[bw(write_with = write_string)]
    //name: String,
    name: PascalString,

    /*url_length: u8,
    #[br(count = url_length)]
    url: Vec<u8>,*/
    //
    //#[br(parse_with = read_string)]
    //#[bw(write_with = write_string)]
    //url: String,
    url: PascalString,

    port: u32,
}

const TERRARIA_SERVER_LIST: [u8; 89] = [
    0x17, 0x01, 0x00, 0x00, 0x06, 0x52, 0x65, 0x4D, 0x6F, 0x56, 0x65, 0x13, 0x72, 0x65, 0x64, 0x61,
    0x63, 0x74, 0x65, 0x64, 0x2E, 0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x2E, 0x63, 0x6F, 0x6D, 0x61,
    0x1E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

impl fmt::Display for TerrariaServerList {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Terraria Server List (v{})\n{}", self.version, {
            let mut output = String::new();

            for (index, entry) in self.servers.iter().enumerate() {
                if entry.port != 0 {
                    output.push_str(&format!(
                        "#{}: {} ({}:{})\n",
                        index + 1,
                        entry.name,
                        entry.url,
                        entry.port
                    ));
                } else {
                    output.push_str(&format!("#{}: N/A\n", index + 1));
                }
            }

            output
        })
    }
}

fn main() {
    let file = TERRARIA_SERVER_LIST;
    println!("{file:?}\n");

    let mut reader = Cursor::new(file);
    let servers = TerrariaServerList::read(&mut reader).unwrap();
    println!("{servers:?}\n\n{servers}");

    let mut writer = Cursor::new(Vec::<u8>::new());
    writer.write_le(&servers).unwrap();
    println!("{:?}", writer.into_inner());

    // Deref syntax sugar
    //let a = &servers.servers[0].url;
    //let b = a.as_bytes();
    println!("");

    //let a = PascalString(vec![1, 2, 3]);
    //let b = a.to_string();

    //println!("{:?}", AsciiString::from_ascii(*b"asdf\x80"));
    //println!("{:?}", AsciiString::from(String::from("asdfñü")));

    /*println!(
        "Encode = {:?}\nDecode = {:?}",
        ASCII.encode("cafe", EncoderTrap::Strict),
        ASCII.decode(&[99, 97, 102, 101], DecoderTrap::Strict)
    );*/

    // stream & map_stream test
    let mut out = Cursor::new(vec![]);
    //Test { a: 0x201, b: 0x403 }.write(&mut out).unwrap();
    // The map_stream/stream combo iterates over every byte of Test, but not TestParent.
    TestParent {
        version: 42069,
        yeet: Test { a: 0x201, b: 0x403 },
    }
    .write(&mut out)
    .unwrap();
    println!("\n{:?}", out.into_inner());
}

use binrw::{parser, writer, BinResult};

// The only example use of map_stream and stream is from a unit test
// https://github.com/jam1garner/binrw/blob/master/binrw/tests/derive/write/stream.rs

struct Checksum<T> {
    inner: T,
    check: core::num::Wrapping<u8>,
}

impl<T> Checksum<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            check: core::num::Wrapping(0),
        }
    }

    fn check(&self) -> u8 {
        self.check.0
    }
}

impl<T: Write> Write for Checksum<T> {
    fn write(&mut self, buf: &[u8]) -> binrw::io::Result<usize> {
        for b in buf {
            print!("0x{b:X} ");
            self.check += b;
        }
        self.inner.write(buf)
    }

    fn flush(&mut self) -> binrw::io::Result<()> {
        self.inner.flush()
    }
}

impl<T: Seek> Seek for Checksum<T> {
    fn seek(&mut self, pos: binrw::io::SeekFrom) -> binrw::io::Result<u64> {
        self.inner.seek(pos)
    }
}

#[binrw::binwrite]
#[bw(little, stream = writer, map_stream = Checksum::new)]
struct Test {
    a: u16,
    b: u16,
    #[bw(calc(writer.check()))]
    c: u8,
}

#[binrw::binwrite]
#[bw(little)]
struct TestParent {
    version: u32,
    yeet: Test,
}

// Generated via:
/*
println!("const ASCII: [char; 256] = [");
for byte in 0u8..=255 {
    println!("    {:?}, // {byte} / 0x{byte:X}", byte as char);
}
println!("];");
*/

#[parser(reader, endian)]
fn read_string() -> BinResult<String> {
    let mut output = String::new();
    let count = <u8>::read_options(reader, endian, ())?;

    for _ in 0..count {
        let byte = <u8>::read_options(reader, endian, ())?;
        output.push(byte as char);
    }

    Ok(output)
}

#[writer(writer, endian)]
fn write_string(input: &String) -> BinResult<()> {
    let count: u8 = input
        .len()
        .try_into()
        .expect(&format!("Strings must be less than {} in length!", u8::MAX));
    count.write_options(writer, endian, ())?;

    for byte in input.bytes() {
        byte.write_options(writer, endian, ())?;
    }

    Ok(())
}

use binrw::{
    io::{Read, Seek, Write},
    Endian,
};

struct PascalString(String);

impl BinRead for PascalString {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        (): Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut bytes = vec![];
        let count = <u8>::read_options(reader, endian, ())?;

        for _ in 0..count {
            let byte = <u8>::read_options(reader, endian, ())?;
            bytes.push(byte);
        }

        Ok(Self(ASCII.decode(&bytes, DecoderTrap::Strict).expect(
            "Invalid ASCII string found while decoding/reading.",
        )))
    }
}

impl BinWrite for PascalString {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let count: u8 = self
            .0
            .len()
            .try_into()
            .expect("The length of a PascalString must not exceed a byte!");

        count.write_options(writer, endian, args)?;

        ASCII
            .encode(&self.0, EncoderTrap::Strict)
            .expect("Invalid ASCII string found while encoding/writing.")
            .write_options(writer, endian, args)?;

        Ok(())
    }
}

impl fmt::Debug for PascalString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "\"{}\"", self.0)
    }
}

impl fmt::Display for PascalString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::ops::Deref for PascalString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PascalString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
