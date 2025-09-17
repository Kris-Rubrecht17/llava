use super::prelude::*;

unsafe extern "C" {
    pub fn LLVMLoadLibraryPermanently(Filename: *const c_char) -> LLVMBool;
    pub fn LLVMParseCommandLineOptions(
        argc: c_int,
        argv: *const *const c_char,
        Overview: *const c_char,
    );
    /// Search all previously loaded dynamic libraries for the named symbol.
    ///
    /// Returns its address if found, otherwise null.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMSearchForAddressOfSymbol(symbolName: *const c_char) -> *mut c_void;
    /// Permanently add the named symbol with the provided value.
    ///
    /// Symbols added this way are searched before any libraries.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMAddSymbol(symbolName: *const c_char, symbolValue: *mut c_void);
}
