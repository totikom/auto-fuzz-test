use syn::token::{Async, Unsafe};
use syn::{visit::Visit, FnDecl, Ident, ImplItem, ItemFn, ItemImpl, Type};

pub struct FnVisitor<T> {
    pub callback:
        Box<dyn FnMut(Option<&Type>, &Ident, &FnDecl, &Option<Unsafe>, &Option<Async>, &T) -> ()>,
    pub context: T,
}

impl<'ast, T> Visit<'ast> for FnVisitor<T> {
    // based on syn visitor example by David Tolnay:
    // https://github.com/dtolnay/syn/issues/549
    fn visit_item_fn(&mut self, f: &'ast ItemFn) {
        (self.callback)(
            None,
            &f.ident,
            &*f.decl,
            &f.unsafety,
            &f.asyncness,
            &self.context,
        );
        syn::visit::visit_item_fn(self, f);
    }
    fn visit_item_impl(&mut self, f: &'ast ItemImpl) {
        let self_type = &f.self_ty;
        for item in &f.items {
            if let ImplItem::Method(f) = item {
                (self.callback)(
                    Some(self_type),
                    &f.sig.ident,
                    &f.sig.decl,
                    &f.sig.unsafety,
                    &f.sig.asyncness,
                    &self.context,
                );
            }
        }
        syn::visit::visit_item_impl(self, f);
    }
}
