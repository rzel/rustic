/*
 * Copyright (c) 2013 Matthew Iselin
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

use mach::{IrqHandler, TimerHandlers, IoPort};

use machine;

static BaseFrequency: uint = 1193180;

pub struct Pit {
    timer_hz: uint,
}

impl Pit {
    pub fn new() -> Pit{
        Pit{timer_hz: 0}
    }

    pub fn init(hz: uint) -> Pit {
        let mut state = Pit::new();

        state.timer_hz = hz;

        // Program periodic mode, with our desired divisor for the given
        // frequency (in hertz).
        let div = BaseFrequency / state.timer_hz;
        machine().outport(0x43, 0x36u8);
        machine().outport(0x40, (div & 0xFF) as u8);
        machine().outport(0x40, ((div >> 8) & 0xFF) as u8);

        state
    }

    pub fn irq_num() -> uint {
        0
    }
}

impl IrqHandler for Pit {
    fn irq(&mut self, _: uint) {
        machine().timer_fired(1000 / self.timer_hz);
    }
}
