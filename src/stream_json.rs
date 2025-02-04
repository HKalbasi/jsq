use serde::Deserialize;
use serde_json::{from_reader, Deserializer, StreamDeserializer, Value};
use std::{io::{self, stdin, Read, Stdin}, iter};


pub struct MyReader {
    stdin: Stdin,
    buffer: Vec<u8>,
    pos: usize,
    cap: usize,
}

impl MyReader {
    pub fn new() -> Self {
        Self {
            stdin: stdin(),
            buffer: Vec::new(),
            pos: 0,
            cap: 0,
        }
    }
}

impl Read for MyReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos == self.cap {
            if self.cap == self.buffer.len() {
                self.buffer.extend(iter::repeat_n(0, 1 << 16));
            }
            let buf = &mut self.buffer[self.cap..];
            self.cap += self.stdin.read(buf)?;
        }
        buf[0] = self.buffer[self.pos];
        self.pos += 1;
        Ok(1)
    }
}

pub struct State {
    reader: MyReader,
}

struct SerdeIgnored;

impl<'de> Deserialize<'de> for SerdeIgnored {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(SerdeIgnored)
    }
}

#[derive(Deserialize)]
struct X {
    x: i32,
}

impl State {
    pub fn new() -> Self {
        Self { reader: MyReader::new() }
    }

    pub fn next(&mut self) -> String {
        let mut x = Deserializer::from_reader(&mut self.reader).into_iter::<Value>();

        match x.next() {
            Some(Ok(_)) => {
                let r = String::from_utf8_lossy(&self.reader.buffer[..self.reader.pos]).into_owned();
                self.reader.buffer.drain(..self.reader.pos);
                self.reader.cap -= self.reader.pos;
                self.reader.pos = 0;
                r
            },
            Some(Err(e)) => panic!("{e}"),
            None => {
                panic!("");
            },
        }
    }
}