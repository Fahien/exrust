use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, BufReader, BufWriter, Write};
use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom},
};

#[macro_use]
extern crate serde_derive;

// We can use type aliases
type ByteStr = [u8];
type ByteString = Vec<u8>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pair {
    pub key: ByteString,
    pub value: ByteString,
}

/// This structure uses Bitcask as file format.
/// Every record has two sections:
/// - A fixed-length header with 3 values:
///   - checksum (4 bytes)
///   - key-length (4 bytes)
///   - value-length (4 bytes)
/// - A variable length body with 2 values:
///   - key (key-length bytes)
///   - value (value-length bytes).
#[derive(Debug)]
pub struct Store {
    file: std::fs::File,
    pub index: HashMap<ByteString, u64>,
}

impl Store {
    pub fn open(file_path: &std::path::Path) -> io::Result<Store> {
        // The ? operator will return the error if open fails
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)?;
        Ok(Store {
            file,
            index: HashMap::new(),
        })
    }

    fn process_record<R: Read>(file: &mut R) -> io::Result<Pair> {
        // Read checksum and data
        let checksum = file.read_u32::<LittleEndian>()?;
        let key_len = file.read_u32::<LittleEndian>()?;
        let val_len = file.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        file.take(data_len as u64).read_to_end(&mut data)?;
        debug_assert_eq!(data.len(), data_len as usize);

        // Apply checking function to data and compare with the read one
        let checksum_ieee = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let computed_checksum = checksum_ieee.checksum(&data);
        if computed_checksum != checksum {
            panic!(
                "Checksum failed ({:08x} != {:08})",
                computed_checksum, checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(Pair { key, value })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = std::io::BufReader::new(&mut self.file);

        loop {
            let current_position = f.seek(SeekFrom::Current(0))?;

            let maybe_pair = Store::process_record(&mut f);
            let pair = match maybe_pair {
                Ok(pair) => pair,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };

            self.index.insert(pair.key, current_position);
        }

        Ok(())
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            Some(p) => *p,
            None => return Ok(None),
        };

        let mut reader = BufReader::new(&mut self.file);
        reader.seek(SeekFrom::Start(position))?;
        let pair = Store::process_record(&mut reader)?;

        Ok(Some(pair.value))
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

    fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        // Make space for a new record
        let key_len = key.len();
        let val_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + val_len);

        // Store key and value contiguously
        for byte in key {
            tmp.push(*byte);
        }

        for byte in value {
            tmp.push(*byte);
        }

        let mut writer = BufWriter::new(&mut self.file);
        let new_position = writer.seek(SeekFrom::End(0))?;

        // Write header and data
        let checksum_ieee = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let checksum = checksum_ieee.checksum(&tmp);
        writer.write_u32::<LittleEndian>(checksum)?;
        writer.write_u32::<LittleEndian>(key_len as u32)?;
        writer.write_u32::<LittleEndian>(val_len as u32)?;
        writer.write_all(&mut tmp)?;
        writer.flush()?;

        Ok(new_position)
    }

    /// Inserts a new record
    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);

        Ok(())
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn write_numbers_to_file() -> (u32, i8, f64) {
        let mut w = vec![];

        let one: u32 = 1;
        let two: i8 = 2;
        let three: f64 = 3.0;

        w.write_u32::<LittleEndian>(one).unwrap();
        println!("{:?}", &w);

        w.write_i8(two).unwrap();
        println!("{:?}", &w);

        w.write_f64::<LittleEndian>(three).unwrap();
        println!("{:?}", &w);

        (one, two, three)
    }

    fn read_numbers_from_file() -> (u32, i8, f64) {
        let mut r = std::io::Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);

        // Little endian means that bytes at lower positions would go at the end of the number

        // Consider [1, 0, 0, 0]:
        // 1 is at position 0
        // 0 is at position 1
        // 0 is at position 2
        // 0 is at position 3

        // 1 going at the end mean that the resulting u32 layout will be this: 0001

        let one = r.read_u32::<LittleEndian>().unwrap();
        let two = r.read_i8().unwrap();
        let three = r.read_f64::<LittleEndian>().unwrap();

        (one, two, three)
    }

    #[test]
    fn write_read_file() {
        let (w_one, w_two, w_three) = write_numbers_to_file();
        let (r_one, r_two, r_three) = read_numbers_from_file();

        assert_eq!(w_one, r_one);
        assert_eq!(w_two, r_two);
        assert_eq!(w_three, r_three);
    }

    fn parity_bit(bytes: &[u8]) -> u8 {
        let mut one_count = 0;

        for byte in bytes {
            one_count += byte.count_ones();
        }

        (one_count % 2 == 0) as u8
    }

    #[test]
    fn check_parity_bit() {
        let abc = b"abc";
        assert_eq!(abc, &[97u8, 98, 99]);
        assert_eq!(parity_bit(abc), 1);

        let abcd = b"abcd";
        assert_eq!(abcd, &[97u8, 98, 99, 100]);
        assert_eq!(parity_bit(abcd), 0);
    }
}
