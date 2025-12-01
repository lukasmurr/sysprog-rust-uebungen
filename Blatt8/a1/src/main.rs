use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

const FIRST_NAME_LEN: usize = 40;
const LAST_NAME_LEN: usize = 40;
const ADDRESS_LEN: usize = 240;
const AGE_LEN: usize = 4;
const RECORD_LEN: usize = FIRST_NAME_LEN + LAST_NAME_LEN + ADDRESS_LEN + AGE_LEN;

#[derive(Debug)]
struct Record {
    first_name: String,
    last_name: String,
    address: String,
    age: u32,
}

impl Record {
    fn new(first_name: &str, last_name: &str, address: &str, age: u32) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            address: address.to_string(),
            age,
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != RECORD_LEN {
            return Err(format!("Invalid record length: {}", bytes.len()));
        }

        let first_name = parse_string(&bytes[0..FIRST_NAME_LEN]);
        let last_name = parse_string(&bytes[FIRST_NAME_LEN..FIRST_NAME_LEN + LAST_NAME_LEN]);
        let address = parse_string(
            &bytes[FIRST_NAME_LEN + LAST_NAME_LEN..FIRST_NAME_LEN + LAST_NAME_LEN + ADDRESS_LEN],
        );

        let age_bytes = &bytes[FIRST_NAME_LEN + LAST_NAME_LEN + ADDRESS_LEN..];
        let age = u32::from_le_bytes(age_bytes.try_into().unwrap());

        Ok(Self {
            first_name,
            last_name,
            address,
            age,
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(RECORD_LEN);

        bytes.extend(create_fixed_string(&self.first_name, FIRST_NAME_LEN));
        bytes.extend(create_fixed_string(&self.last_name, LAST_NAME_LEN));
        bytes.extend(create_fixed_string(&self.address, ADDRESS_LEN));
        bytes.extend(self.age.to_le_bytes());

        bytes
    }
}

fn parse_string(bytes: &[u8]) -> String {
    // Find the first null byte or take the whole slice
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[0..end]).to_string()
}

fn create_fixed_string(s: &str, len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    let s_bytes = s.as_bytes();
    let copy_len = std::cmp::min(s_bytes.len(), len);
    bytes[0..copy_len].copy_from_slice(&s_bytes[0..copy_len]);
    // The rest is already 0 due to vec! initialization
    bytes
}

fn read_records<P: AsRef<Path>>(path: P) -> io::Result<Vec<Record>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut records = Vec::new();
    for chunk in buffer.chunks(RECORD_LEN) {
        if chunk.len() == RECORD_LEN {
            if let Ok(record) = Record::from_bytes(chunk) {
                records.push(record);
            }
        }
    }
    Ok(records)
}

fn write_records<P: AsRef<Path>>(path: P, records: &[Record]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for record in records {
        file.write_all(&record.to_bytes())?;
    }
    Ok(())
}

fn create_dummy_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let records = vec![
        Record::new("John", "Doe", "123 Main St", 30),
        Record::new("Jane", "Smith", "456 Oak Ave", 25),
        Record::new("Bob", "Johnson", "789 Pine Rd", 40),
    ];
    write_records(path, &records)
}

fn main() -> io::Result<()> {
    let filename = "pgu.dat";

    if !Path::new(filename).exists() {
        println!("{} not found. Creating dummy file...", filename);
        create_dummy_file(filename)?;
    }

    println!("Reading records from {}...", filename);
    let mut records = read_records(filename)?;

    for (i, record) in records.iter().enumerate() {
        println!("Record {}: {:?}", i, record);
    }

    println!("\nIncrementing age of all records by 1...");
    for record in &mut records {
        record.age += 1;
    }

    for (i, record) in records.iter().enumerate() {
        println!("Modified Record {}: {:?}", i, record);
    }

    println!("\nWriting records back to {}...", filename);
    write_records(filename, &records)?;

    println!("Done.");
    Ok(())
}
