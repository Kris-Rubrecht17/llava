//! Output of the LLVM bitcode format.

use super::prelude::*;

unsafe extern "C" {
    /// Write a module to the specified path.
    ///
    /// Returns 0 on success.
    pub fn LLVMWriteBitcodeToFile(M: LLVMModuleRef, Path: *const c_char) -> c_int;
    /// Write a module to an open file descriptor.
    ///
    /// Returns 0 on success.
    pub fn LLVMWriteBitcodeToFD(
        M: LLVMModuleRef,
        FD: c_int,
        ShouldClose: c_int,
        Unbuffered: c_int,
    ) -> c_int;
    /// Deprecated: use LLVMWriteBitcodeToFD
    pub fn LLVMWriteBitcodeToFileHandle(M: LLVMModuleRef, Handle: c_int) -> c_int;
    /// Writes a module to a new memory buffer.
    pub fn LLVMWriteBitcodeToMemoryBuffer(M: LLVMModuleRef) -> LLVMMemoryBufferRef;
}
