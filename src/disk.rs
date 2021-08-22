use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

use zerocopy::{AsBytes, FromBytes};

pub const PAGE_SIZE: usize = 4096;

pub struct PageId(pub u64);

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    // constructor
    pub fn new(heap_file: File) -> io::Result<Self> {
        // get file size
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
        Self::new(heap_file);
    }

    // number a new page ID
    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        // caluculate offset
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // seek move page top
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // read data
        self.heap_file.read_exact(data)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        // caluculate offset
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        // seek move page top
        self.heap_file.seek(SeekFrom::Start(offset))?;
        // write data
        self.heap_file.write_all(data)
    }
}
