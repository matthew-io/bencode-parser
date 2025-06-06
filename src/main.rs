use std::fs;

use bencode::decode::Bencode;

mod bencode;

fn main() {
    let contents = fs::read("./debian-12.11.0-amd64-netinst.iso.torrent")
        .expect("couldn't read torrent file");
    let (bencode, _) = bencode::decode::decode(&contents)
        .expect("couldn't decode given torrent");
    
    let dict = match bencode {
        Bencode::Dict(d) => d,
        _ => panic!("couldn't get top-level dictionary from torrent")
    };
    
    println!("{:#?}", dict.get(&b"announce"[..]));
}
