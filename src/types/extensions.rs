use crate::IndexCell;
use crate::str_err;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

pub trait GetResultExt {
    fn get_res(&self, key: &str) -> Result<&usize, String>;
}
impl GetResultExt for HashMap<String, usize> {
    fn get_res(&self, key: &str) -> Result<&usize, String> {
        self.get(key)
            .ok_or(str_err!(format!("Key '{}' not found", key)))
    }
}

pub trait GetCellResultExt {
    fn get_res(&self, index: usize) -> Result<&Rc<RefCell<IndexCell>>, String>;
}
impl GetCellResultExt for Vec<Rc<RefCell<IndexCell>>> {
    fn get_res(&self, index: usize) -> Result<&Rc<RefCell<IndexCell>>, String> {
        self.get(index).ok_or(str_err!(format!(
            "Index {} out of bounds (len: {})",
            index,
            self.len()
        )))
    }
}

pub trait UpgradeResultExt<T: ?Sized> {
    fn upgrade_res(&self) -> Result<Rc<T>, String>;
}
impl<T: ?Sized> UpgradeResultExt<T> for Weak<T> {
    fn upgrade_res(&self) -> Result<Rc<T>, String> {
        self.upgrade()
            .ok_or(str_err!("Weak pointer is expired or no longer valid"))
    }
}
