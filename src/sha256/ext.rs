use digest::{HashMarker, Output, block_buffer::Eager, consts::*, core_api::*};
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct ExtSha256Core {
    sha256: fn(input: &[u8], output: &mut [u8; 32]),
    buffer: Vec<u8>,
}

impl ExtSha256Core {
    pub fn new(sha256: fn(input: &[u8], output: &mut [u8; 32])) -> CoreWrapper<Self> {
        CoreWrapper::from_core(Self {
            sha256,
            buffer: Vec::with_capacity(256),
        })
    }
}

impl HashMarker for ExtSha256Core {}

impl BlockSizeUser for ExtSha256Core {
    type BlockSize = U64;
}

impl BufferKindUser for ExtSha256Core {
    type BufferKind = Eager;
}
impl UpdateCore for ExtSha256Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        for block in blocks {
            self.buffer.extend_from_slice(block.as_slice());
        }
    }
}

impl OutputSizeUser for ExtSha256Core {
    type OutputSize = U32;
}

impl FixedOutputCore for ExtSha256Core {
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        self.buffer.extend_from_slice(buffer.get_data());
        (self.sha256)(self.buffer.as_slice(), out.as_mut())
    }
}

impl Reset for ExtSha256Core {
    fn reset(&mut self) {
        self.buffer.clear();
    }
}

impl AlgorithmName for ExtSha256Core {
    fn write_alg_name(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sha256")
    }
}
