// Based on https://github.com/pingcap/talent-plan/tree/master/courses/rust

use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use super::MissueEngine;
use crate::{MissueError, Result, Issue, Status};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;
pub struct KvStore {
    //path: PathBuf,
    reader: BufReaderWithPos<File>,
    writer: BufWriterWithPos<File>,
    index: BTreeMap<String, CommandPos>,
    uncompacted: u64,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        new_log_file(&path)?;
        let mut index = BTreeMap::new();
        let mut reader = BufReaderWithPos::new(File::open(log_path(&path))?)?;
        let uncompacted = load(reader.by_ref(), &mut index)?;
        let writer = new_log_file(&path)?;
        Ok(KvStore {
            reader,
            writer,
            index,
            uncompacted,
        })
    }

    pub fn compact(&mut self) -> Result<()> {
        //self.writer = clear_log_file(&self.path)?;
        unimplemented!("No compact function yet")
    }
}

impl MissueEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        let pos = self.writer.pos;
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        if let Command::Set { key, .. } = cmd {
            if let Some(old_cmd) = self.index.insert(key, (pos..self.writer.pos).into()) {
                self.uncompacted += old_cmd.len;
            }
        }

        if self.uncompacted > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    /// Checks if a key is in the index
    fn has(&self, key: &String) -> bool {
        let issue = self.index.get(key);
        match issue {
            Some(_) => true,
            None => false
        }
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            self.reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            let cmd_reader = self.reader.by_ref().take(cmd_pos.len);
            if let Command::Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(MissueError::UnexpectedCommandType)
            }
        } else {
            Ok(None)
        }
    }

    // return the list of all the issues
    fn all(&mut self) -> Result<Vec<String>> {
        let mut list: Vec<String> = Vec::new();
        let keys_iterator: Vec<String> = self.index.keys().cloned().collect();
        for key in keys_iterator {
            let val = self.get(key)?;
            list.push(val.unwrap());
        }
        Ok(list)
    }

    fn open(&mut self) -> Result<Vec<String>> {
        let mut list: Vec<String> = Vec::new();
        let keys_iterator: Vec<String> = self.index.keys().cloned().collect();
        for key in keys_iterator {
            let val = self.get(key)?.unwrap();
            let issue: Issue = serde_json::from_str(&val)?;
            match issue.status {
                Status::CLOSED => {},
                _ =>  list.push(val)
            }
        }
        Ok(list)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let cmd = Command::remove(key);
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            if let Command::Remove { key } = cmd {
                let old_cmd = self.index.remove(&key).expect("key not found");
                self.uncompacted += old_cmd.len;
            }
            Ok(())
        } else {
            Err(MissueError::KeyNotFound)
        }
    }
}

fn new_log_file(path: &Path) -> Result<BufWriterWithPos<File>> {
    let path = log_path(&path);
    let writer = BufWriterWithPos::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;
    Ok(writer)
}

fn load(
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    let mut uncompacted = 0; 
    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {
                if let Some(old_cmd) = index.insert(key, (pos..new_pos).into()) {
                    uncompacted += old_cmd.len;
                }
            }
            Command::Remove { key } => {
                if let Some(old_cmd) = index.remove(&key) {
                    uncompacted += old_cmd.len;
                }
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }
    Ok(uncompacted)
}

fn log_path(dir: &Path) -> PathBuf {
    dir.join(format!("{}.log", "missue"))
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

struct CommandPos {
    pos: u64,
    len: u64,
}

impl From<Range<u64>> for CommandPos {
    fn from(range: Range<u64>) -> Self {
        CommandPos {
            pos: range.start,
            len: range.end - range.start,
        }
    }
}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}
