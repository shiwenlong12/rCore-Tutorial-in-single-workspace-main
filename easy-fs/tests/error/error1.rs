
use easy_fs::{BlockDevice, EasyFileSystem};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

/// Use a block size of 512 bytes
const BLOCK_SZ: usize = 512;
const BLOCK_NUM: usize = 131072; //64*2048

/// Wrapper for turning a File into a BlockDevice
struct BlockFile(Mutex<File>);

impl BlockDevice for BlockFile {
    /// Read a block from file
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let mut file = self.0.lock().unwrap();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.read(buf).unwrap(), BLOCK_SZ, "Not a complete block!");
    }
    /// Write a block into file
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        let mut file = self.0.lock().unwrap();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.write(buf).unwrap(), BLOCK_SZ, "Not a complete block!");
    }
}

// pub fn easy_fs_pack(cases: &Vec<String>, target: &str) -> std::io::Result<()> {
//     let block_file = Arc::new(BlockFile(Mutex::new({
//         let f = OpenOptions::new()
//             .read(true)
//             .write(true)
//             .create(true)
//             .open(format!("{}/{}", target, "fs.img"))?;
//         f.set_len(64 * 2048 * 512).unwrap();
//         f
//     })));
//     println!("Packing Testcases...");
//     let efs = EasyFileSystem::create(block_file, 64 * 2048, 1);
//     println!("Packing Testcases...");
//     let root_inode = Arc::new(EasyFileSystem::root_inode(&efs));
//     println!("Packing Testcases...");
//     for case in cases {
//         println!("{}", format!("{}/{}", target, case));
//         // load app data from host file system
//         let mut host_file = File::open(format!("{}/{}", target, case)).unwrap();
//         let mut all_data: Vec<u8> = Vec::new();
//         host_file.read_to_end(&mut all_data).unwrap();
//         // create a file in easy-fs
//         let inode = root_inode.create(case.as_str()).unwrap();
//         // write data to easy-fs
//         inode.write_at(0, all_data.as_slice());
//         // println!("{}", all_data.len());
//     }
//     println!("List Testcases in EFS: ");
//     // list app
//     for case in root_inode.readdir() {
//         println!("{}", case);
//     }
//     Ok(())
// }




#[test]
fn efs_test() -> std::io::Result<()> {
    let block_file = Arc::new(BlockFile(Mutex::new({
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("target/fs.img")?;
        f.set_len((BLOCK_NUM * BLOCK_SZ) as u64).unwrap();
        f
    })));
    let _efs1 = EasyFileSystem::create(block_file.clone()
    ,4096*32// 4096*32
    , 1);
    // let efs = EasyFileSystem::open(block_file.clone());
    // let root_inode = EasyFileSystem::root_inode(&efs);
    // root_inode.create("filea");
    // root_inode.create("fileb");
    // for name in root_inode.ls() {
    //     println!("{}", name);
    // }
    // let filea = root_inode.find("filea").unwrap();
    // let greet_str = "Hello, world!";
    // filea.write_at(0, greet_str.as_bytes());
    // //let mut buffer = [0u8; BLOCK_SZ];
    // let mut buffer = [0u8; 233];
    // let len = filea.read_at(0, &mut buffer);
    // assert_eq!(greet_str, core::str::from_utf8(&buffer[..len]).unwrap(),);

    // let mut random_str_test = |len: usize| {
    //     filea.clear();
    //     assert_eq!(filea.read_at(0, &mut buffer), 0,);
    //     let mut str = String::new();
    //     use rand;
    //     // random digit
    //     for _ in 0..len {
    //         str.push(char::from('0' as u8 + rand::random::<u8>() % 10));
    //     }
    //     filea.write_at(0, str.as_bytes());
    //     let mut read_buffer = [0u8; 127];
    //     let mut offset = 0usize;
    //     let mut read_str = String::new();
    //     loop {
    //         let len = filea.read_at(offset, &mut read_buffer);
    //         if len == 0 {
    //             break;
    //         }
    //         offset += len;
    //         read_str.push_str(core::str::from_utf8(&read_buffer[..len]).unwrap());
    //     }
    //     assert_eq!(str, read_str);
    // };

    // random_str_test(4 * BLOCK_SZ);
    // random_str_test(8 * BLOCK_SZ + BLOCK_SZ / 2);
    // random_str_test(100 * BLOCK_SZ);
    // random_str_test(70 * BLOCK_SZ + BLOCK_SZ / 7);
    // random_str_test((12 + 128) * BLOCK_SZ);
    // random_str_test(400 * BLOCK_SZ);
    // random_str_test(1000 * BLOCK_SZ);
    // random_str_test(2000 * BLOCK_SZ);

    Ok(())
}
