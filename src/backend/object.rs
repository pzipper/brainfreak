//! A backend for Brainfreak which outputs an object file.

use cranelift::{
    codegen::{
        ir::{types, AbiParam, UserExternalName, UserFuncName},
        settings,
    },
    frontend::{FunctionBuilder, FunctionBuilderContext},
    prelude::Configurable,
};
use cranelift_module::{default_libcall_names, DataDescription, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::{
    backend::{Backend, Intrinsics},
    inst::Inst,
};

/// Compiles the provided instructions into an object file.
pub fn compile(insts: &[Inst]) -> Vec<u8> {
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "false").unwrap();
    flag_builder.set("is_pic", "false").unwrap();

    let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
        panic!("host machine not supported: {}", msg);
    });
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .unwrap();

    let mut module =
        ObjectModule::new(ObjectBuilder::new(isa, "", default_libcall_names()).unwrap());

    let mut ctx = module.make_context();
    let mut builder_ctx = FunctionBuilderContext::new();

    let mut main_sig = module.make_signature();
    main_sig.returns.push(AbiParam::new(types::I32));

    let main_func = module
        .declare_function("main", Linkage::Export, &main_sig)
        .unwrap();

    ctx.func.signature = main_sig;
    ctx.func.name = UserFuncName::User(UserExternalName::new(0, main_func.as_u32()));

    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

    let intrinsics = Intrinsics {
        putchar: {
            let mut putchar_sig = module.make_signature();
            putchar_sig.params.push(AbiParam::new(types::I8));
            putchar_sig.returns.push(AbiParam::new(types::I32));

            module
                .declare_function("putchar", Linkage::Import, &putchar_sig)
                .unwrap()
        },
        getchar: {
            let mut getchar_sig = module.make_signature();
            getchar_sig.returns.push(AbiParam::new(types::I8));

            module
                .declare_function("getchar", Linkage::Import, &getchar_sig)
                .unwrap()
        },
        heap: {
            let data = module.declare_anonymous_data(true, false).unwrap();
            let mut data_desc = DataDescription::new();
            data_desc.define_zeroinit(32000);
            module.define_data(data, &data_desc).unwrap();
            data
        },
    };

    let mut backend = Backend::new(&intrinsics, &mut builder, &mut module);
    backend.lower(insts);
    backend.finish();

    module.define_function(main_func, &mut ctx).unwrap();

    module.finish().emit().unwrap()
}
