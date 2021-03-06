/*
 * Copyright (c) 2014 Matthew Iselin
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use std::fmt;
use std::slice::Vector;

use std::string;

pub struct MemBuffer {
    buf: Vec<u8>,
}

impl MemBuffer {
    pub fn new() -> MemBuffer {
        MemBuffer::with_capacity(128)
    }

    pub fn with_capacity(n: uint) -> MemBuffer {
        MemBuffer{buf: Vec::with_capacity(n)}
    }

    pub fn get_ref<'a>(&'a self) -> &'a [u8] { self.buf.as_slice() }

    pub fn unwrap(self) -> Vec<u8> { self.buf }
}

impl fmt::FormatWriter for MemBuffer {
    fn write(&mut self, buf: &[u8]) -> fmt::Result {
        self.buf.push_all(buf);
        Ok(())
    }
}

pub fn fmt(args: &fmt::Arguments) -> string::String {
    let mut output = MemBuffer::new();
    let _ = fmt::write(&mut output, args);
    string::String::from_utf8(output.unwrap()).unwrap()
}
