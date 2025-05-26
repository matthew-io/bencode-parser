mod bencode;

fn main() {
    let str = b"i34e";
    bencode::decode::parse_int(str);
}
