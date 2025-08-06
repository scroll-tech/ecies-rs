use digest::{block_buffer::Eager, consts::*, core_api::*, *};
use openvm_sha2::set_sha256;
use std::fmt::Formatter;

pub type Sha256 = CoreWrapper<Sha256Core>;

#[derive(Debug, Clone)]
pub struct Sha256Core {
    buffer: Vec<u8>,
}

impl Default for Sha256Core {
    fn default() -> Self {
        Self {
            buffer: Vec::with_capacity(64),
        }
    }
}

impl HashMarker for Sha256Core {}

impl BlockSizeUser for Sha256Core {
    type BlockSize = U64;
}

impl BufferKindUser for Sha256Core {
    type BufferKind = Eager;
}
impl UpdateCore for Sha256Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        for block in blocks {
            self.buffer.extend_from_slice(block.as_slice());
        }
    }
}

impl OutputSizeUser for Sha256Core {
    type OutputSize = U32;
}

impl FixedOutputCore for Sha256Core {
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        self.buffer.extend_from_slice(buffer.get_data());
        set_sha256(self.buffer.as_slice(), out.as_mut())
    }
}

impl Reset for Sha256Core {
    fn reset(&mut self) {
        self.buffer.clear();
    }
}

impl AlgorithmName for Sha256Core {
    fn write_alg_name(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sha256")
    }
}
