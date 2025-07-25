use crate::consts::{QOI_OP_DIFF, QOI_OP_LUMA, QOI_OP_RGB, QOI_OP_RGBA};
use crate::error::Result;
use crate::utils::Writer;
use bytemuck::{cast, Pod};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Pixel([u8; 4]);

impl Pixel {
    #[inline]
    pub const fn new() -> Self {
        Self([0; 4])
    }

    #[inline]
    pub fn read(&mut self, s: &[u8]) {
        if s.len() == 4 {
            let mut i = 0;
            while i < 4 {
                self.0[i] = s[i];
                i += 1;
            }
        } else {
            unreachable!();
        }
    }

    #[inline]
    pub fn update(&mut self, px: Pixel) {
        for i in 0..4 {
            self.0[i] = px.0[i];
        }
    }

    #[inline]
    pub fn update_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.0[0] = r;
        self.0[1] = g;
        self.0[2] = b;
    }

    #[inline]
    pub fn update_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.0[0] = r;
        self.0[1] = g;
        self.0[2] = b;
        self.0[3] = a;
    }

    #[inline]
    pub fn update_diff(&mut self, b1: u8) {
        self.0[0] = self.0[0].wrapping_add((b1 >> 4) & 0x03).wrapping_sub(2);
        self.0[1] = self.0[1].wrapping_add((b1 >> 2) & 0x03).wrapping_sub(2);
        self.0[2] = self.0[2].wrapping_add(b1 & 0x03).wrapping_sub(2);
    }

    #[inline]
    pub fn update_luma(&mut self, b1: u8, b2: u8) {
        let vg = (b1 & 0x3f).wrapping_sub(32);
        let vg_8 = vg.wrapping_sub(8);
        let vr = vg_8.wrapping_add((b2 >> 4) & 0x0f);
        let vb = vg_8.wrapping_add(b2 & 0x0f);
        self.0[0] = self.0[0].wrapping_add(vr);
        self.0[1] = self.0[1].wrapping_add(vg);
        self.0[2] = self.0[2].wrapping_add(vb);
    }

    #[inline]
    pub const fn as_rgba(self) -> Pixel {
        let mut i = 0;
        let mut out = Pixel::new();
        while i < 4 {
            out.0[i] = self.0[i];
            i += 1;
        }
        out
    }

    #[inline]
    pub const fn r(self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub const fn g(self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub const fn b(self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub const fn a(self) -> u8 {
        self.0[3]
    }

    #[inline]
    pub const fn with_a(mut self, value: u8) -> Self {
        self.0[3] = value;
        self
    }

    #[inline]
    #[allow(clippy::cast_lossless, clippy::cast_possible_truncation)]
    pub fn hash_index(self) -> u8
    where
        [u8; 4]: Pod,
    {
        // credits for the initial idea: @zakarumych
        let v = u32::from_le_bytes(cast(self.0)) as u64;
        let s = ((v & 0xff00_ff00) << 32) | (v & 0x00ff_00ff);
        (s.wrapping_mul(0x0300_0700_0005_000b_u64) >> 56) as u8 & 63
    }

    #[inline]
    pub fn rgb_add(&mut self, r: u8, g: u8, b: u8) {
        self.0[0] = self.0[0].wrapping_add(r);
        self.0[1] = self.0[1].wrapping_add(g);
        self.0[2] = self.0[2].wrapping_add(b);
    }

    #[inline]
    pub fn encode_into<W: Writer>(&self, px_prev: Self, buf: W) -> Result<W> {
        if self.a() == px_prev.0[3] {
            let vg = self.g().wrapping_sub(px_prev.g());
            let vg_32 = vg.wrapping_add(32);
            if vg_32 | 63 == 63 {
                let vr = self.r().wrapping_sub(px_prev.r());
                let vb = self.b().wrapping_sub(px_prev.b());
                let vg_r = vr.wrapping_sub(vg);
                let vg_b = vb.wrapping_sub(vg);
                let (vr_2, vg_2, vb_2) =
                    (vr.wrapping_add(2), vg.wrapping_add(2), vb.wrapping_add(2));
                if vr_2 | vg_2 | vb_2 | 3 == 3 {
                    buf.write_one(QOI_OP_DIFF | (vr_2 << 4) | (vg_2 << 2) | vb_2)
                } else {
                    let (vg_r_8, vg_b_8) = (vg_r.wrapping_add(8), vg_b.wrapping_add(8));
                    if vg_r_8 | vg_b_8 | 15 == 15 {
                        buf.write_many(&[QOI_OP_LUMA | vg_32, (vg_r_8 << 4) | vg_b_8])
                    } else {
                        buf.write_many(&[QOI_OP_RGB, self.r(), self.g(), self.b()])
                    }
                }
            } else {
                buf.write_many(&[QOI_OP_RGB, self.r(), self.g(), self.b()])
            }
        } else {
            buf.write_many(&[QOI_OP_RGBA, self.r(), self.g(), self.b(), self.a()])
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Pixel> for [u8; 4] {
    #[inline(always)]
    fn from(px: Pixel) -> Self {
        px.0
    }
}
