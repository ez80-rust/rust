//! Full disclaimer: Claude was used to create this, because I don't understand how this works.
// see https://github.com/CE-Programming/llvm-project/blob/z80/llvm/lib/Target/Z80/Z80CallingConv.td
use crate::callconv::{FnAbi, ArgAbi, TyAbiInterface, HasDataLayout, HasTargetSpec};
// use rustc_abi::FieldsShape;

/// eZ80 (24-bit) return value convention
fn classify_ret_eez80<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.is_ignore() {
        return;
    }

    match ret.layout.size.bits() {
        // Promote i1 to i8
        // 1 => ret.extend_integer_width_to(8),
        // // i8 values
        // 8 => ret.extend_integer_width_to(24),
        // // i16 values
        // 16 => ret.extend_integer_width_to(24),
        ..24 => ret.extend_all_width_to(24),
        // i24 values stay as-is
        24 => {}
        // i64, f64, and aggregates: pass indirect
        _ => ret.make_indirect(),
    }
}

/// eZ80 (24-bit) argument convention
fn classify_arg_eez80<Ty>(arg: &mut ArgAbi<'_, Ty>) {
    if arg.is_ignore() {
        return;
    }

    // CCIfByVal<CCPassByVal<3, 1>>
    if arg.layout.is_aggregate() {
        arg.pass_by_stack_offset(Some(crate::callconv::Align::from_bytes(1).unwrap()));
        return;
    }

    // CCIfType<[ i1, i8, i16 ], CCPromoteToType<i24>>
    match arg.layout.size.bits() {
        // 1 | 8 | 16 => arg.extend_integer_width_to(24),
        ..24 => arg.extend_all_width_to(24),
        // CCIfType<[ i24 ], CCAssignToStack<3, 1>>
        24 => {}
        _ => arg.make_indirect(),
    }
}

pub(crate) fn compute_abi_info<Ty>(fn_abi: &mut FnAbi<'_, Ty>) {
    if !fn_abi.ret.is_ignore() {
        classify_ret_eez80(&mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg_eez80(arg);
    }
}

// end AI generated code

fn classify_one_rust<'a, Ty, C>(_cx: &C, arg: &mut ArgAbi<'a, Ty>) where Ty: TyAbiInterface<'a, C> + Copy,
        C: HasDataLayout + HasTargetSpec {
    arg.extend_all_width_to(24);
}

pub(crate) fn compute_rust_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>) where Ty: TyAbiInterface<'a, C> + Copy,
        C: HasDataLayout + HasTargetSpec {
    if !fn_abi.ret.is_ignore() {
        classify_one_rust(cx, &mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_one_rust(cx, arg);
    }
}