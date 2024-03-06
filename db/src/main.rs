use std::convert::TryFrom;
use std::{fs, path::Path};

use parquet::file::reader::SerializedFileReader;
use parquet::file::writer::SerializedFileWriter;
use parquet::record::RecordWriter;
use parquet_derive::ParquetRecordWriter;

const PARQUET_FILEPATH: &str = "./target/sample.parquet";

#[derive(ParquetRecordWriter)]
struct ACompleteRecord<'a> {
    pub a_bool: bool,
    pub a_str: &'a str,
}

fn write() {
    let path = Path::new(PARQUET_FILEPATH);
    let file = fs::File::create(path).unwrap();
    let samples = vec![
        ACompleteRecord {
            a_bool: true,
            a_str: "I'm true",
        },
        ACompleteRecord {
            a_bool: false,
            a_str: "I'm false",
        },
    ];
    let schema = samples.as_slice().schema().unwrap();
    let mut writer = SerializedFileWriter::new(file, schema, Default::default()).unwrap();
    let mut row_group = writer.next_row_group().unwrap();
    samples
        .as_slice()
        .write_to_row_group(&mut row_group)
        .unwrap();
    row_group.close().unwrap();
    writer.close().unwrap();
}

fn read() {
    let rows = [PARQUET_FILEPATH]
        .iter()
        .map(|p| SerializedFileReader::try_from(*p).unwrap())
        .flat_map(|r| r.into_iter());

    for row in rows {
        println!("{}", row.unwrap());
    }
}

fn main() {
    write();
    read();
}