
use easy_fs::{BlockDevice, EasyFileSystem, FileHandle};
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



#[test]
fn test_efs_vfs() -> std::io::Result<()> {
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
    ,4096*32
    , 1);
    let efs = EasyFileSystem::open(block_file.clone());
    //let efs2 = efs.lock().unwarp();
    //创建目录,获取文件系统的根索引节点 
    let root_inode = EasyFileSystem::root_inode(&efs);
    //测试vfs的inode结构的find、create、readdir、write_at、read_at、clear
    //创建文件
    root_inode.create("filea");
    root_inode.create("fileb");
    root_inode.create("filec");
    let dir = root_inode.readdir();
    assert_eq!(dir, ["filea","fileb","filec"]);

    // 找到filea文件，并向其中写和从中读数据到缓冲区
    let filea = root_inode.find("filea").unwrap();
    let greet_str = "Hello, world!";
    // as_bytes()将字符串转化成u8数组
    filea.write_at(0, greet_str.as_bytes());
    let mut buffer = [0u8; 233];
    //文件长度
    let len = filea.read_at(0, &mut buffer);
    assert_eq!(len, 13);
    //调用标准库中std::str::from_utf8函数将含有ASCII码的数组转化成字符串
    //比较读出来的数据
    assert_eq!(greet_str, core::str::from_utf8(&buffer[..len]).unwrap(),);

    //清除当前索引节点中的数据
    filea.clear();
    // 从当前索引节点读取数据，返回0表示数据清空
    assert_eq!(filea.read_at(0, &mut buffer), 0,);
    
    // 测试filehandle
    // 测试empty、readable、writable,根据参数的不同设置四种测例
    let handle1 =FileHandle::empty(true, true);
    assert_eq!(handle1.readable(), true);
    assert_eq!(handle1.writable(), true);
    let handle1 =FileHandle::empty(true, false);
    assert_eq!(handle1.readable(), true);
    assert_eq!(handle1.writable(), false);
    let handle1 =FileHandle::empty(false, true);
    assert_eq!(handle1.readable(), false);
    assert_eq!(handle1.writable(), true);
    let handle1 =FileHandle::empty(false, false);
    assert_eq!(handle1.readable(), false);
    assert_eq!(handle1.writable(), false);

    let handle2 = FileHandle::new(true, true, filea);
    assert_eq!(handle2.readable(), true);
    assert_eq!(handle2.writable(), true);



    Ok(())
}



// const ARR:[u8; 5] = [1,2,5,1,4];
// use easy_fs::{UserBuffer, OpenFlags, FileHandle};
// #[test]
// fn test_file() -> std::io::Result<()> {
//     let mut a = [1,2,5,1,4];
//     let b = &'static mut ARR[..3];
//     let buffers = vec![b];
//     UserBuffer::new(buffers);
//     Ok(())
// }
