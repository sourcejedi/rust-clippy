use syntax::ptr::P;
use syntax::ast::*;
use rustc::lint::{Context, LintPass, LintArray, Lint};
use rustc::middle::ty::{expr_ty, sty, ty_ptr, ty_rptr, mt};

declare_lint!(pub MUT_MUT, Warn,
              "Warn on usage of double-mut refs, e.g. '&mut &mut ...'");

#[derive(Copy,Clone)]
pub struct MutMut;

impl LintPass for MutMut {
	fn get_lints(&self) -> LintArray {
        lint_array!(MUT_MUT)
	}
	
	fn check_expr(&mut self, cx: &Context, expr: &Expr) {
		
		fn unwrap_addr(expr : &Expr) -> Option<&Expr> {
			match expr.node {
				ExprAddrOf(MutMutable, ref e) => Option::Some(e),
				_ => Option::None
			}
		}
		
		unwrap_addr(expr).map_or((), |e| {
			unwrap_addr(e).map(|_| {
				cx.span_lint(MUT_MUT, expr.span, 
					"Generally you want to avoid &mut &mut _ if possible.")
			}).unwrap_or_else(|| {
				if let ty_rptr(_, mt{ty: _, mutbl: MutMutable}) = 
						expr_ty(cx.tcx, e).sty {
					cx.span_lint(MUT_MUT, expr.span,
						"This expression mutably borrows a mutable reference. \
						Consider reborrowing")
				}
			})
		})
	}
	
	fn check_ty(&mut self, cx: &Context, ty: &Ty) {
		unwrap_mut(ty).and_then(unwrap_mut).map_or((), |_| cx.span_lint(MUT_MUT, 
			ty.span, "Generally you want to avoid &mut &mut _ if possible."))
	}
}

fn unwrap_mut(ty : &Ty) -> Option<&Ty> {
	match ty.node {
		TyPtr(MutTy{ ty: ref pty, mutbl: MutMutable }) => Option::Some(pty),
		TyRptr(_, MutTy{ ty: ref pty, mutbl: MutMutable }) => Option::Some(pty),
		_ => Option::None
	}
}
