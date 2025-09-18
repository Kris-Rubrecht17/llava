





#[cfg(test)]
mod tests {
    use inkwell::{AddressSpace, targets::{InitializationConfig, Target}};

    #[test]
    fn create_context() {
        let ctx = inkwell::context::Context::create();
        let module = ctx.create_module("main");
        let print_ty = ctx.void_type().fn_type(&[ctx.ptr_type(AddressSpace::default()).into()],false);
        let main_ty = ctx.i32_type().fn_type(&[
            ctx.i32_type().into(),
            ctx.ptr_type(inkwell::AddressSpace::default()).into()
        ],false);
        let m = module.add_function("main",main_ty,None);
        let p = module.add_function("print",print_ty,None);
        let builder = ctx.create_builder();
        let entry = ctx.append_basic_block(m,"entry");
        builder.position_at_end(entry);
        let argv = m.get_nth_param(1).unwrap().into_pointer_value();
        
        let ptr = unsafe {builder.build_gep(ctx.ptr_type(AddressSpace::default()),argv , &[ctx.i32_type().const_int(0,false)], "tmp")};
        let argv0 = builder.build_load(ctx.ptr_type(AddressSpace::default()),ptr.unwrap(),"argv0").unwrap();
        let _ = builder.build_call(p,&[argv0.into()],"call_print");
        let _ = builder.build_return(Some(&ctx.i32_type().const_int(0,false)));
        
        let _ = Target::initialize_all(&InitializationConfig::default());
        let trip = inkwell::targets::TargetMachine::get_default_triple();
        let target = Target::from_triple(&trip).unwrap();
        let target_machine = target
            .create_target_machine(
                &trip,
                "generic",
                "",
                inkwell::OptimizationLevel::Default,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default
            ).unwrap();
        let _ = target_machine.write_to_file(
            &module,
            inkwell::targets::FileType::Object,
            &std::path::Path::new("t.o")
        );
            
    }
}