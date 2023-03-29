//!An easy file system isolated from the kernel
#![no_std]
// #![deny(missing_docs)]
extern crate alloc;
mod bitmap;
mod block_cache;
mod block_dev;
mod efs;
mod file;
mod layout;
mod vfs;
/// Use a block size of 512 bytes
pub const BLOCK_SZ: usize = 512;
use bitmap::Bitmap;
use block_cache::{block_cache_sync_all, get_block_cache};
pub use block_dev::BlockDevice;
pub use efs::EasyFileSystem;
pub use file::*;
use layout::*;
pub use vfs::Inode;


# [cfg(test)]
mod tests{


    use crate::Bitmap;
    //use crate::BlockDevice;
    use crate::file::{UserBuffer};


    // use alloc::sync::Arc;
    // use spin::Mutex;


    // pub struct VirtIOBlk<'a, H: Hal> {
    //     header: &'static mut VirtIOHeader,
    //     queue: VirtQueue<'a, H>,
    //     capacity: usize,
    // }
    // struct VirtIOBlock(Mutex<VirtIOBlk<'static, VirtioHal>>);

    // impl BlockDevice for VirtIOBlock {
    //     fn read_block(&self, block_id: usize, buf: &mut [u8]) {
    //         self.0
    //             .lock()
    //             .read_block(block_id, buf)
    //             .expect("Error when reading VirtIOBlk");
    //     }
    //     fn write_block(&self, block_id: usize, buf: &[u8]) {
    //         self.0
    //             .lock()
    //             .write_block(block_id, buf)
    //             .expect("Error when writing VirtIOBlk");
    //     }
    // }



    #[test]
    fn test_bitmap() {
        // let mut a = 10;
        // decomposition(a);
        let bitmap1 = Bitmap::new(0,10);
    }

    #[test]
    fn test_block_cache() {
        
    }

    #[test]
    fn test_efs() {
        
    }

    #[test]
    fn test_file() {
        // let mut points_buf : Vec<u8> = Vec::with_capacity(points.len() * point::POINT_SIZE);
        // for _ in (0..points_buf.capacity()) {
        //     points_buf.push(0);
        // }
        // file.read(&mut points_buf[..]).unwrap();

        // let mut buffer = [0u8; 512];
        // //let mut v: Vec<u8> = Vec![0; 512];
        // //let buffers = v.extend_from_slice(&buffer[..512]);
        // UserBuffer::new(v);
    }

    use crate::layout::{SuperBlock};
    #[test]
    fn test_layout() {
        let a = SuperBlock{
            magic: EFS_MAGIC,
            total_blocks: 512,
            inode_bitmap_blocks: 1,
            inode_area_blocks: 1,
            data_bitmap_blocks: 1,
            data_area_blocks: 1,
        };
        SuperBlock::initialize(a,EFS_MAGIC,);
    }

    #[test]
    fn test_vfs() {
        
    }
}