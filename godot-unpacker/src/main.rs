use binrw::{
    io::{Cursor},
    BinRead, BinWrite,
};

#[derive(BinRead, BinWrite, Debug)]
#[brw(little, magic = b"GDPC")]
struct ZipHeader {
    version_pack: u32,
    version_godot_major: u32,
    version_godot_minor: u32,
    version_godot_patch: u32,
}

fn main() {
    let mut reader = Cursor::new(include_bytes!(
        "../DialogueTest.pck"
    ));
    let servers = ZipHeader::read(&mut reader).unwrap();
    println!("{servers:?}");
}
