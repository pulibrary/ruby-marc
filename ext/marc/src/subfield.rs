use std::cell::RefCell;

use magnus::{ scan_args::scan_args, Value };

#[derive(Default, Debug)]
pub struct Subfield {
  code: String,
  value: String,
}

#[derive(Default, Debug)]
#[magnus::wrap(class = "MARC::Subfield")]
pub struct MutSubfield {cell: RefCell<Subfield>}

impl MutSubfield {
  pub fn new(args: &[Value]) -> Self {
    let args = scan_args::<(), (Option<String>, Option<String>,), (), (), (), ()>(args).unwrap();
    let (code, value,): (Option<String>, Option<String>,) = args.optional;
    MutSubfield {cell: RefCell::new(Subfield {
      code: code.unwrap_or_default(),
      value: value.unwrap_or_default(),
    })}
  }

  pub fn get_code(&self) -> String {
    self.cell.borrow().code.clone()
  }

  pub fn set_code(&self, code: String) -> String {
    let mut sf = self.cell.borrow_mut();
    sf.code = code;
    sf.code.clone()
  }

  pub fn get_value(&self) -> String {
    self.cell.borrow().value.clone()
  }

  pub fn set_value(&self, value: String) -> String {
    let mut sf = self.cell.borrow_mut();
    sf.value = value;
    sf.value.clone()
  }

  pub fn to_s(&self) -> String {
    let sf = self.cell.borrow();
    format!("${} {} ", sf.code, sf.value)
  }
}

impl PartialEq for MutSubfield {
    fn eq(&self, other: &Self) -> bool {
      self.cell.borrow().code == other.cell.borrow().code && self.cell.borrow().value == other.cell.borrow().value
    }
}

#[cfg(test)]
mod tests {
  use magnus::IntoValue;
  use rb_sys_test_helpers::ruby_test;

use super::*;

  #[ruby_test]
  fn it_can_be_created_with_values() {
    let subfield = MutSubfield::new(&["a".to_string().into_value(), "foo".to_string().into_value()]);
    assert_eq!(subfield.get_code(), "a");
    assert_eq!(subfield.get_value(), "foo");
  }

  #[ruby_test]
  fn it_can_determine_equality() {
    let s1 =  MutSubfield::new(&["a".to_string().into_value(), "foo".to_string().into_value()]);
    let s2 =  MutSubfield::new(&["a".to_string().into_value(), "foo".to_string().into_value()]);
    let s3 =  MutSubfield::new(&["b".to_string().into_value(), "bar".to_string().into_value()]);

    assert_eq!(s1, s2);
    assert_ne!(s1, s3);
  }

  #[ruby_test]
  fn it_can_return_the_code_and_value() {
    let s1 =  MutSubfield::new(&["a".to_string().into_value(), "foo".to_string().into_value()]);
    assert_eq!(s1.get_code(), "a");
    assert_eq!(s1.get_value(), "foo");
  }

  #[ruby_test]
  fn it_can_update_fields() {
    let s1 =  MutSubfield::new(&["a".to_string().into_value(), "foo".to_string().into_value()]);
    assert_eq!(s1.get_code(), "a");
    assert_eq!(s1.get_value(), "foo");
    s1.set_code("b".to_string());
    s1.set_value("bar".to_string());
    assert_eq!(s1.get_code(), "b");
    assert_eq!(s1.get_value(), "bar");
  }

  #[ruby_test]
  fn it_can_convert_to_string() {
    let s1 =  MutSubfield::new(&["a".to_string().into_value(), "foo".to_string().into_value()]);
    assert_eq!(s1.to_s(), "$a foo ");
  }
}
