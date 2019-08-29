use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::*;
use std::thread;
use std::process;

/// TODO: too large array cause stackoverflow
const BLOCK_SIZE:u64 = 1024;

fn main() {
    copy("foo.txt", "foo2.txt", 2).unwrap();
}

fn copy(source: &str, dest: &str, thread_num: u64) -> Result<()> {
    let source_file = OpenOptions::new().read(true).open(source)?;
    let dest_file = OpenOptions::new().create_new(true).write(true).open(dest)?;
    let file_size = source_file.metadata().unwrap().len();
    if file_size == 0 {
        println!("finished");
        process::exit(0);
    }
    dest_file.set_len(file_size).unwrap();
    let mut thread_handlers = Vec::new();
    for i in 0..thread_num {
        println!("{}", i);
        let mut s_file = OpenOptions::new().read(true).open(source)?;
        let mut dest_file = OpenOptions::new().write(true).open(dest)?;
        let mut buf = [0 as u8; BLOCK_SIZE as usize];
        let h = thread::spawn(move || {
            let mut index = i * BLOCK_SIZE;
            loop {
                if index > file_size {break;}
                s_file.seek(SeekFrom::Start(index)).unwrap();
                let size = s_file.read(buf.as_mut()).unwrap();
                dest_file.seek(SeekFrom::Start(index)).unwrap();
                dest_file.write(&buf[..size]).unwrap();
                index += thread_num * BLOCK_SIZE;
                println!("thread{} : {} finish", i, index)
            }
        });
        thread_handlers.push(h);
    }
    loop {
        if let Some(h) = thread_handlers.pop() {
            h.join().unwrap();
        } else {
            break;
        }
    }
    Ok(())
}
