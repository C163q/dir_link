use std::{
    cell::UnsafeCell,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub struct Debugger {
    out: UnsafeCell<BufWriter<File>>,
}

impl Debugger {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let out = UnsafeCell::new(BufWriter::new(File::create(path)?));
        Ok(Self { out })
    }

    /// ## Safety
    ///
    /// 需要保证不会同时有多个同一个对象的引用调用`write`方法
    pub unsafe fn write(&self, msg: &str) -> io::Result<()> {
        writeln!(unsafe { &mut *self.out.get() }, "{}", msg)?;
        Ok(())
    }
}
