use std::collections::HashMap;

use inkwell::{AddressSpace, OptimizationLevel};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{FloatValue, PointerValue};

use crate::ast::{BinOp, Expr};

type FuncSig = unsafe extern "C" fn(*const f64) -> f64;

pub struct JitSystem<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> JitSystem<'ctx> {
    /// # Panics
    ///
    /// Possible that LLVM may go wrong and be unable to produce an execution engine
    #[must_use]
    pub fn new(ctx: &Context, level: OptimizationLevel) -> JitSystem {
        let module = ctx.create_module("main_module");
        let execution_engine = module
            .create_jit_execution_engine(level)
            .expect("Should have an execution engine");
        JitSystem {
            context: ctx,
            module,
            execution_engine,
        }
    }

    /// # Panics
    ///
    /// Possible that LLVM may go wrong and be unable to produce the function we are inserting into the module
    pub fn build_function(&self, function_name: &str, expr: &Expr) -> FunctionResult {
        let slot_positions = self.prepare_function(function_name, expr);

        let func: JitFunction<FuncSig> = unsafe {
            self.execution_engine
                .get_function(function_name)
                .expect("Function should exist")
        };

        FunctionResult {
            slot_positions,
            func,
        }
    }

    fn prepare_function(&self, function_name: &str, expr: &Expr) -> Vec<String> {
        let builder = FunctionBuilder::new(&self, function_name);
        let mut slot_data = SlotData::new();
        let res = builder.build_expression(expr, &mut slot_data);
        builder.builder.build_return(Some(&res)).ok();
        slot_data.to_positions()
    }
}


struct SlotData {
    slot_position: HashMap<String, u64>,
}

impl SlotData {
    fn new() -> Self {
        Self {
            slot_position: HashMap::new(),
        }
    }

    fn get(&mut self, name: &String) -> u64 {
        if let Some(slot) = self.slot_position.get(name) {
            *slot
        } else {
            let slot = self.slot_position.len() as u64;
            self.slot_position.insert(name.clone(), slot);
            slot
        }
    }

    pub fn to_positions(&self) -> Vec<String> {
        let mut slot_positions = self.slot_position.keys().cloned().collect::<Vec<_>>();
        slot_positions.sort_by_key(|s| self.slot_position.get(s));
        slot_positions
    }
}

struct FunctionBuilder<'ctx> {
    context: &'ctx Context,
    slice_param: PointerValue<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> FunctionBuilder<'ctx> {
    fn new(function_generator: &JitSystem<'ctx>, function_name: &str) -> FunctionBuilder<'ctx> {
        let builder = function_generator.context.create_builder();

        // Define the function signature
        let f64_type = function_generator.context.f64_type();

        // slice of f64 VALUES
        let slice_ptr_type = f64_type.ptr_type(AddressSpace::default());

        // fn(f64*) -> f64
        let fn_type = f64_type.fn_type(&[slice_ptr_type.into()], false);
        let function = function_generator
            .module
            .add_function(function_name, fn_type, None);
        // Extract function parameters
        let slice_param = function.get_nth_param(0).unwrap().into_pointer_value();

        let entry_block = function_generator
            .context
            .append_basic_block(function, "entry");
        builder.position_at_end(entry_block);

        FunctionBuilder {
            context: function_generator.context,
            slice_param,
            builder,
        }
    }

    fn build_expression(&self, expr: &Expr, slot_data: &mut SlotData) -> FloatValue {
        match expr {
            Expr::Binary(left, op, right) => self.build_binary_op(left, op, right, slot_data),
            Expr::Number(num) => self.build_number(*num),
            Expr::Identifier(name) => self.build_identifier(name, slot_data),
        }
    }

    fn build_number(&self, num: f64) -> FloatValue {
        self.context.f64_type().const_float(num)
    }

    fn build_identifier(&self, name: &String, slot_data: &mut SlotData) -> FloatValue {
        let index_param = self.context.i64_type().const_int(slot_data.get(name), false);
        let nth_element_ptr = unsafe {
            self.builder
                .build_gep(
                    self.context.f64_type(),
                    self.slice_param,
                    &[index_param],
                    "nth_element_ptr",
                )
                .unwrap()
        };
        FloatValue::try_from(
            self.builder
                .build_load(self.context.f64_type(), nth_element_ptr, "nth_element")
                .unwrap(),
        )
            .unwrap()
    }
    fn build_binary_op(
        &self,
        left: &Expr,
        op: &BinOp,
        right: &Expr,
        slot_data: &mut SlotData,
    ) -> FloatValue {
        let left = self.build_expression(left, slot_data);
        let right = self.build_expression(right, slot_data);

        match op {
            BinOp::Add => self.builder.build_float_add(left, right, "add"),
            BinOp::Subtract => self.builder.build_float_sub(left, right, "sub"),
            BinOp::Multiply => self.builder.build_float_mul(left, right, "mul"),
            BinOp::Divide => self.builder.build_float_div(left, right, "div"),
        }
            .expect("IR builder should be able to build basic floating operations")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    MissingIdentifier,
}

pub struct FunctionResult<'ctx> {
    slot_positions: Vec<String>,
    func: JitFunction<'ctx, FuncSig>,
}

impl<'ctx> FunctionResult<'ctx> {
    pub fn call(&self, values: &HashMap<String, f64>) -> Result<f64, Error> {
        let slots = self
            .slot_positions
            .iter()
            .map(|x| {
                values
                    .get(x)
                    .ok_or(Error::MissingIdentifier)
                    .map(|value| *value)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(unsafe { self.func.call(slots.as_ptr()) })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ast = (
            Expr::ident("a") + Expr::num(2.0)
        ) * (
            Expr::ident("b") / Expr::ident("c")
        );

        let mut values = HashMap::new();
        values.insert("a".into(), 1.0);
        values.insert("b".into(), 2.0);
        values.insert("c".into(), 3.0);

        let context = Context::create();
        let jitter = JitSystem::new(&context, OptimizationLevel::None);
        let function = jitter.build_function("demo_func", &ast);
        assert_eq!(function.call(&values), Ok(2.));
    }
}
