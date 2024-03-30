use std::io::{self, Write};

fn compress(input: &[u8]) -> Vec<u8> {
    zstd::encode_all(input, 0).expect("Compression failed")
}

fn decompress(input: &[u8]) -> Vec<u8> {
    zstd::decode_all(input).expect("Decompression failed")
}

fn main() {
    // 输入数据
    let input_data = b"Hello, this is a sample input for compression and decompression using zstd.";

    // 压缩
    let compressed_data = compress(input_data);
    println!("Compressed data: {:?}", compressed_data);

    // 解压缩
    let decompressed_data = decompress(&compressed_data);
    println!("Decompressed data: {:?}", decompressed_data);

    // 输出解压后的数据
    io::stdout()
        .write_all(&decompressed_data)
        .expect("Failed to write decompressed data to stdout");
}
