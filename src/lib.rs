use llvm_ffi::prelude::*;
use llvm_ffi::core::*;





#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn create_context() {
        let ctx = unsafe {
            llvm_ffi::core::LLVMContextCreate()
        };
        assert_ne!(ctx,std::ptr::null_mut());
    }
}