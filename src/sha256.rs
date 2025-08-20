use digest::{Digest, DynDigest, HashMarker, Output, block_buffer::Eager, consts::*, core_api::*};
use std::{
    fmt::Formatter,
    sync::atomic::{AtomicUsize, Ordering},
};

static STATE: AtomicUsize = AtomicUsize::new(0);
const UNINITIALIZED: usize = 0;
const INITIALIZING: usize = 1;
const INITIALIZED: usize = 2;

static mut MAKE_DIGEST: &dyn Fn() -> Box<dyn DynDigest> = &|| Box::new(sha2::Sha256::new());

#[derive(Debug)]
pub struct SetDigestMakerError(());

/// Set an alternative digest provider. e.g. zkvm precompile
pub fn set_digest_provider<F>(make_digest: F) -> Result<(), SetDigestMakerError>
where
    F: Fn() -> Box<dyn DynDigest> + 'static,
{
    match STATE.compare_exchange(
        UNINITIALIZED,
        INITIALIZING,
        Ordering::Acquire,
        Ordering::Relaxed,
    ) {
        Ok(UNINITIALIZED) => {
            unsafe {
                MAKE_DIGEST = Box::leak(Box::new(make_digest));
            }
            STATE.store(INITIALIZED, Ordering::Release);
            Ok(())
        }
        Err(INITIALIZING) => {
            while STATE.load(Ordering::Relaxed) == INITIALIZING {
                std::hint::spin_loop();
            }
            Err(SetDigestMakerError(()))
        }
        _ => Err(SetDigestMakerError(())),
    }
}

pub mod ext;

pub(crate) type Sha256 = CoreWrapper<Sha256Core>;

pub(crate) struct Sha256Core {
    inner: Box<dyn DynDigest>,
}

impl Default for Sha256Core {
    fn default() -> Self {
        // Acquire memory ordering guarantees that current thread would see any
        // memory writes that happened before store of the value
        // into `STATE` with memory ordering `Release` or stronger.
        //
        // Since the value `INITIALIZED` is written only after `LOGGER` was
        // initialized, observing it after `Acquire` load here makes both
        // write to the `LOGGER` static and initialization of the logger
        // internal state synchronized with current thread.
        let inner = if STATE.load(Ordering::Acquire) != INITIALIZED {
            Box::new(sha2::Sha256::new())
        } else {
            unsafe { MAKE_DIGEST() }
        };

        Self { inner }
    }
}

impl Clone for Sha256Core {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.box_clone(),
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
            self.inner.update(block.as_slice());
        }
    }
}

impl OutputSizeUser for Sha256Core {
    type OutputSize = U32;
}

impl FixedOutputCore for Sha256Core {
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        self.inner.update(buffer.get_data());
        self.inner.finalize_into_reset(out.as_mut_slice()).unwrap();
    }
}

impl Reset for Sha256Core {
    fn reset(&mut self) {
        self.inner.reset();
    }
}

impl AlgorithmName for Sha256Core {
    fn write_alg_name(f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Sha256")
    }
}
