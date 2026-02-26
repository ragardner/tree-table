use crate::IndexCell;
use crate::str_err;
use std::collections::HashMap;
use std::rc::Rc;

pub trait GetResultExt {
    fn get_res(&self, key: &str) -> Result<&usize, String>;
}
impl GetResultExt for HashMap<Rc<str>, usize> {
    fn get_res(&self, key: &str) -> Result<&usize, String> {
        self.get(key).ok_or(str_err!("Key '{}' not found", key))
    }
}

pub trait GetCellResultExt {
    fn get_res(&self, index: usize) -> Result<&IndexCell, String>;
}
impl GetCellResultExt for Vec<IndexCell> {
    fn get_res(&self, index: usize) -> Result<&IndexCell, String> {
        let len = self.len();
        self.get(index)
            .ok_or(str_err!("Index {} out of bounds (len: {})", index, len))
    }
}

pub trait GetCellMutResultExt {
    fn get_res_mut(&mut self, index: usize) -> Result<&mut IndexCell, String>;
}
impl GetCellMutResultExt for Vec<IndexCell> {
    fn get_res_mut(&mut self, index: usize) -> Result<&mut IndexCell, String> {
        let len = self.len();
        self.get_mut(index)
            .ok_or(str_err!("Index {} out of bounds (len: {})", index, len))
    }
}

pub trait TrySetExt<T> {
    fn try_set(&mut self, index: usize, value: T) -> Result<(), String>;
}

impl<T> TrySetExt<T> for Vec<T> {
    fn try_set(&mut self, index: usize, value: T) -> Result<(), String> {
        if index < self.len() {
            self[index] = value;
            Ok(())
        } else {
            Err(format!(
                "index {} out of bounds for length {}",
                index,
                self.len()
            ))
        }
    }
}
