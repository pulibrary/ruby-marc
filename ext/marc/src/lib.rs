use magnus::{Error, function, method, Ruby, Module, Object};
use subfield::MutSubfield;

mod subfield;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("MarcRs")?;
    let subfield = module.define_class("Subfield", ruby.class_object())?;
    subfield.define_singleton_method("new", function!(MutSubfield::new, -1))?;
    subfield.define_method("code", method!(MutSubfield::get_code, 0))?;
    subfield.define_method("code=", method!(MutSubfield::set_code, 1))?;
    subfield.define_method("value", method!(MutSubfield::get_value, 0))?;
    subfield.define_method("value=", method!(MutSubfield::set_value, 1))?;
    Ok(())
}