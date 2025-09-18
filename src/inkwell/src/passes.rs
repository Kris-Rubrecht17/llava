
use llvm_sys::core::{
    LLVMCreateFunctionPassManagerForModule, LLVMCreatePassManager, LLVMDisposePassManager,
    LLVMFinalizeFunctionPassManager, LLVMInitializeFunctionPassManager, LLVMRunFunctionPassManager, LLVMRunPassManager,
};
use llvm_sys::transforms::pass_builder::{
    LLVMCreatePassBuilderOptions, LLVMDisposePassBuilderOptions, LLVMPassBuilderOptionsRef,
    LLVMPassBuilderOptionsSetCallGraphProfile, LLVMPassBuilderOptionsSetDebugLogging,
    LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll, LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap,
    LLVMPassBuilderOptionsSetLicmMssaOptCap, LLVMPassBuilderOptionsSetLoopInterleaving,
    LLVMPassBuilderOptionsSetLoopUnrolling, LLVMPassBuilderOptionsSetLoopVectorization,
    LLVMPassBuilderOptionsSetMergeFunctions, LLVMPassBuilderOptionsSetSLPVectorization,
    LLVMPassBuilderOptionsSetVerifyEach,
};
use llvm_sys::prelude::LLVMPassManagerRef;
use crate::module::Module;
use crate::values::{AsValueRef, FunctionValue};

use std::borrow::Borrow;
use std::marker::PhantomData;

// REVIEW: Opt Level might be identical to targets::Option<CodeGenOptLevel>
// This is an ugly privacy hack so that PassManagerSubType can stay private
// to this module and so that super traits using this trait will be not be
// implementable outside this library
pub trait PassManagerSubType {
    type Input;

    unsafe fn create<I: Borrow<Self::Input>>(input: I) -> LLVMPassManagerRef;
    unsafe fn run_in_pass_manager(&self, pass_manager: &PassManager<Self>) -> bool
    where
        Self: Sized;
}

impl PassManagerSubType for Module<'_> {
    type Input = ();

    unsafe fn create<I: Borrow<Self::Input>>(_: I) -> LLVMPassManagerRef {
        LLVMCreatePassManager()
    }

    unsafe fn run_in_pass_manager(&self, pass_manager: &PassManager<Self>) -> bool {
        LLVMRunPassManager(pass_manager.pass_manager, self.module.get()) == 1
    }
}

// With GATs https://github.com/rust-lang/rust/issues/44265 this could be
// type Input<'a> = &'a Module;
impl<'ctx> PassManagerSubType for FunctionValue<'ctx> {
    type Input = Module<'ctx>;

    unsafe fn create<I: Borrow<Self::Input>>(input: I) -> LLVMPassManagerRef {
        LLVMCreateFunctionPassManagerForModule(input.borrow().module.get())
    }

    unsafe fn run_in_pass_manager(&self, pass_manager: &PassManager<Self>) -> bool {
        LLVMRunFunctionPassManager(pass_manager.pass_manager, self.as_value_ref()) == 1
    }
}

// SubTypes: PassManager<Module>, PassManager<FunctionValue>
/// A manager for running optimization and simplification passes. Much of the
/// documentation for specific passes is directly from the [LLVM
/// documentation](https://llvm.org/docs/Passes.html).
#[derive(Debug)]
pub struct PassManager<T> {
    pub(crate) pass_manager: LLVMPassManagerRef,
    sub_type: PhantomData<T>,
}

impl PassManager<FunctionValue<'_>> {
    /// Acquires the underlying raw pointer belonging to this `PassManager<T>` type.
    pub fn as_mut_ptr(&self) -> LLVMPassManagerRef {
        self.pass_manager
    }

    // return true means some pass modified the module, not an error occurred
    pub fn initialize(&self) -> bool {
        unsafe { LLVMInitializeFunctionPassManager(self.pass_manager) == 1 }
    }

    pub fn finalize(&self) -> bool {
        unsafe { LLVMFinalizeFunctionPassManager(self.pass_manager) == 1 }
    }
}

impl<T: PassManagerSubType> PassManager<T> {
    pub unsafe fn new(pass_manager: LLVMPassManagerRef) -> Self {
        assert!(!pass_manager.is_null());

        PassManager {
            pass_manager,
            sub_type: PhantomData,
        }
    }

    pub fn create<I: Borrow<T::Input>>(input: I) -> PassManager<T> {
        let pass_manager = unsafe { T::create(input) };

        unsafe { PassManager::new(pass_manager) }
    }

    /// This method returns true if any of the passes modified the function or module
    /// and false otherwise.
    pub fn run_on(&self, input: &T) -> bool {
        unsafe { input.run_in_pass_manager(self) }
    }

}

impl<T> Drop for PassManager<T> {
    fn drop(&mut self) {
        unsafe { LLVMDisposePassManager(self.pass_manager) }
    }
}

#[derive(Debug)]
pub struct PassBuilderOptions {
    pub(crate) options_ref: LLVMPassBuilderOptionsRef,
}

impl PassBuilderOptions {
    /// Create a new set of options for a PassBuilder
    pub fn create() -> Self {
        unsafe {
            PassBuilderOptions {
                options_ref: LLVMCreatePassBuilderOptions(),
            }
        }
    }

    /// Acquires the underlying raw pointer belonging to this `PassBuilderOptions` type.
    pub fn as_mut_ptr(&self) -> LLVMPassBuilderOptionsRef {
        self.options_ref
    }

    ///Toggle adding the VerifierPass for the PassBuilder, ensuring all functions inside the module is valid.
    pub fn set_verify_each(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetVerifyEach(self.options_ref, value as i32);
        }
    }

    ///Toggle debug logging when running the PassBuilder.
    pub fn set_debug_logging(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetDebugLogging(self.options_ref, value as i32);
        }
    }

    pub fn set_loop_interleaving(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetLoopInterleaving(self.options_ref, value as i32);
        }
    }

    pub fn set_loop_vectorization(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetLoopVectorization(self.options_ref, value as i32);
        }
    }

    pub fn set_loop_slp_vectorization(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetSLPVectorization(self.options_ref, value as i32);
        }
    }

    pub fn set_loop_unrolling(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetLoopUnrolling(self.options_ref, value as i32);
        }
    }

    pub fn set_forget_all_scev_in_loop_unroll(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(self.options_ref, value as i32);
        }
    }

    pub fn set_licm_mssa_opt_cap(&self, value: u32) {
        unsafe {
            LLVMPassBuilderOptionsSetLicmMssaOptCap(self.options_ref, value);
        }
    }

    pub fn set_licm_mssa_no_acc_for_promotion_cap(&self, value: u32) {
        unsafe {
            LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(self.options_ref, value);
        }
    }

    pub fn set_call_graph_profile(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetCallGraphProfile(self.options_ref, value as i32);
        }
    }

    pub fn set_merge_functions(&self, value: bool) {
        unsafe {
            LLVMPassBuilderOptionsSetMergeFunctions(self.options_ref, value as i32);
        }
    }
}


impl Drop for PassBuilderOptions {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassBuilderOptions(self.options_ref);
        }
    }
}
