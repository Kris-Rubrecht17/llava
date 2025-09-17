#![allow(non_snake_case)]

use crate::error::LLVMErrorRef;
use crate::prelude::*;
use crate::target_machine::LLVMTargetMachineRef;

#[derive(Debug)]
pub enum LLVMOpaquePassBuilderOptions {}
pub type LLVMPassBuilderOptionsRef = *mut LLVMOpaquePassBuilderOptions;

unsafe extern "C" {
    pub fn LLVMRunPasses(
        M: LLVMModuleRef,
        Passes: *const c_char,
        TM: LLVMTargetMachineRef,
        Options: LLVMPassBuilderOptionsRef,
    ) -> LLVMErrorRef;
    /// Construct and run a set of passes over a function.
    ///
    /// This function behaves the same as LLVMRunPasses, but operates on a single
    /// function instead of an entire module.
    pub fn LLVMRunPassesOnFunction(
        F: LLVMValueRef,
        Passes: *const c_char,
        TM: LLVMTargetMachineRef,
        Options: LLVMPassBuilderOptionsRef,
    ) -> LLVMErrorRef;
    pub fn LLVMCreatePassBuilderOptions() -> LLVMPassBuilderOptionsRef;
    pub fn LLVMPassBuilderOptionsSetVerifyEach(
        Options: LLVMPassBuilderOptionsRef,
        VerifyEach: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetDebugLogging(
        Options: LLVMPassBuilderOptionsRef,
        DebugLogging: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetAAPipeline(
        Options: LLVMPassBuilderOptionsRef,
        AAPipeline: *const c_char,
    );
    pub fn LLVMPassBuilderOptionsSetLoopInterleaving(
        Options: LLVMPassBuilderOptionsRef,
        LoopInterleaving: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopVectorization(
        Options: LLVMPassBuilderOptionsRef,
        LoopVectorization: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetSLPVectorization(
        Options: LLVMPassBuilderOptionsRef,
        SLPVectorization: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopUnrolling(
        Options: LLVMPassBuilderOptionsRef,
        LoopUnrolling: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(
        Options: LLVMPassBuilderOptionsRef,
        ForgetAllSCEVInLoopUnroll: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLicmMssaOptCap(
        Options: LLVMPassBuilderOptionsRef,
        LicmMssaOptCap: c_uint,
    );
    pub fn LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(
        Options: LLVMPassBuilderOptionsRef,
        LicmMssaNoAccForPromotionCap: c_uint,
    );
    pub fn LLVMPassBuilderOptionsSetCallGraphProfile(
        Options: LLVMPassBuilderOptionsRef,
        CallGraphProfile: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetMergeFunctions(
        Options: LLVMPassBuilderOptionsRef,
        MergeFunctions: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetInlinerThreshold(
        Options: LLVMPassBuilderOptionsRef,
        Threshold: c_int,
    );
    pub fn LLVMDisposePassBuilderOptions(Options: LLVMPassBuilderOptionsRef);
}
