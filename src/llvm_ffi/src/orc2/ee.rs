use super::*;
use crate::execution_engine::*;

pub type LLVMMemoryManagerCreateContextCallback = unsafe extern "C" fn(CtxCtx: *mut c_void);
pub type LLVMMemoryManagerNotifyTerminatingCallback = unsafe extern "C" fn(CtxCtx: *mut c_void);

unsafe extern "C" {
    pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager(
        ES: LLVMOrcExecutionSessionRef,
    ) -> LLVMOrcObjectLayerRef;
    pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithMCJITMemoryManagerLikeCallbacks(
        ES: LLVMOrcExecutionSessionRef,
        CreateContext: LLVMMemoryManagerCreateContextCallback,
        NotifyTerminating: LLVMMemoryManagerNotifyTerminatingCallback,
        AllocateCodeSection: LLVMMemoryManagerAllocateCodeSectionCallback,
        AllocateDataSection: LLVMMemoryManagerAllocateDataSectionCallback,
        FinalizeMemory: LLVMMemoryManagerFinalizeMemoryCallback,
        Destroy: LLVMMemoryManagerDestroyCallback,
    ) -> LLVMOrcObjectLayerRef;
    pub fn LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener(
        RTDyldObjLinkingLayer: LLVMOrcObjectLayerRef,
        Listener: LLVMJITEventListenerRef,
    );
}
