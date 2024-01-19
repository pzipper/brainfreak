use cranelift::{
    codegen::ir::{types, MemFlags},
    frontend::{FunctionBuilder, Variable},
    prelude::InstBuilder,
};
use cranelift_module::{DataId, FuncId};

use crate::inst::Inst;

pub mod object;

/// A map of intrinsic functions for the [Backend] to use.
#[derive(Debug)]
pub struct Intrinsics {
    /// The LibC `putchar` or equivalent.
    pub putchar: FuncId,

    /// The LibC `getchar` or equivalent.
    pub getchar: FuncId,

    /// The heap for the Brainfreak program to use.
    pub heap: DataId,
}

/// The backend for the Brainfreak compiler.
pub struct Backend<'a, Module: cranelift_module::Module> {
    intrinsics: &'a Intrinsics,
    builder: &'a mut FunctionBuilder<'a>,
    module: &'a mut Module,
    data_ptr: Variable,
}

impl<'a, Module: cranelift_module::Module> Backend<'a, Module> {
    /// Creates a new [Backend] for the provided module.
    pub fn new(
        intrinsics: &'a Intrinsics,
        builder: &'a mut FunctionBuilder<'a>,
        module: &'a mut Module,
    ) -> Self {
        let block = builder.create_block();
        builder.switch_to_block(block);

        let data_ptr = Variable::from_u32(0);
        builder.declare_var(data_ptr, module.target_config().pointer_type());
        let data_ptr_value = {
            let global_value = module.declare_data_in_func(intrinsics.heap, builder.func);
            builder
                .ins()
                .global_value(module.target_config().pointer_type(), global_value)
        };
        builder.def_var(data_ptr, data_ptr_value);

        Self {
            intrinsics,
            builder,
            module,
            data_ptr,
        }
    }

    /// Lowers a single instruction into Cranelift IR.
    fn lower_inst(&mut self, inst: &Inst) {
        match inst {
            Inst::DataInc => {
                let data_ptr = self.builder.use_var(self.data_ptr);
                let old_data = self
                    .builder
                    .ins()
                    .load(types::I8, MemFlags::new(), data_ptr, 0);
                let new_data = self.builder.ins().iadd_imm(old_data, 1);
                self.builder
                    .ins()
                    .store(MemFlags::new(), new_data, data_ptr, 0);
            }
            Inst::DataDec => {
                let data_ptr = self.builder.use_var(self.data_ptr);
                let old_data = self
                    .builder
                    .ins()
                    .load(types::I8, MemFlags::new(), data_ptr, 0);
                let new_data = self.builder.ins().iadd_imm(old_data, -1);
                self.builder
                    .ins()
                    .store(MemFlags::new(), new_data, data_ptr, 0);
            }
            Inst::PtrInc => {
                let old_ptr = self.builder.use_var(self.data_ptr);
                let new_ptr = self.builder.ins().iadd_imm(old_ptr, 1);
                self.builder.def_var(self.data_ptr, new_ptr);
            }
            Inst::PtrDec => {
                let old_ptr = self.builder.use_var(self.data_ptr);
                let new_ptr = self.builder.ins().iadd_imm(old_ptr, -1);
                self.builder.def_var(self.data_ptr, new_ptr);
            }
            Inst::Output => {
                let data_ptr = self.builder.use_var(self.data_ptr);
                let data = self
                    .builder
                    .ins()
                    .load(types::I8, MemFlags::new(), data_ptr, 0);
                let putchar = self
                    .module
                    .declare_func_in_func(self.intrinsics.putchar, self.builder.func);
                self.builder.ins().call(putchar, &[data]);
            }
            Inst::Input => {
                let getchar = self
                    .module
                    .declare_func_in_func(self.intrinsics.getchar, self.builder.func);
                let res = {
                    let res = self.builder.ins().call(getchar, &[]);
                    let value = self.builder.inst_results(res);
                    value[0].clone()
                };
                let data_ptr = self.builder.use_var(self.data_ptr);
                self.builder.ins().store(MemFlags::new(), res, data_ptr, 0);
            }
            Inst::Loop(insts) => {
                let header = self.builder.create_block();
                let body = self.builder.create_block();
                let exit = self.builder.create_block();

                self.builder.ins().jump(header, &[]);

                self.builder.switch_to_block(header);

                let data_ptr = self.builder.use_var(self.data_ptr);
                let data = self
                    .builder
                    .ins()
                    .load(types::I8, MemFlags::new(), data_ptr, 0);
                self.builder.ins().brif(data, body, &[], exit, &[]);

                self.builder.switch_to_block(body);

                for inst in insts {
                    self.lower_inst(inst);
                }

                self.builder.ins().jump(header, &[]);

                self.builder.switch_to_block(exit);

                self.builder.seal_block(header);
                self.builder.seal_block(body);
                self.builder.seal_block(exit);
            }
        }
    }

    /// Lowers the provided [Inst]s into Cranelift IR.
    pub fn lower(&mut self, insts: &[Inst]) {
        for inst in insts {
            self.lower_inst(inst);
        }
    }

    /// Finishes with the backend.
    #[inline]
    pub fn finish(self) {
        let result = self.builder.ins().iconst(types::I32, 0);
        self.builder.ins().return_(&[result]);
    }
}
