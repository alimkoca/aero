pub mod elf;
pub mod process;
pub mod scheduler;

/// Initialize userland.
pub fn init() {}

extern "C" {
    fn jump_userland(address: u64);
}