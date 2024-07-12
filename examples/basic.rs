use bytes::{BufMut, BytesMut};

pub fn main() {
    let mut buf = BytesMut::with_capacity(128);

    println!("len: {:?}", &buf.len());

    // buf traintのメソッド
    buf.put_u8(b'h');

    println!("len: {:?}", &buf.len());

    // 単純に中身のbytesをコピーする.
    let _s = buf.clone();
}
