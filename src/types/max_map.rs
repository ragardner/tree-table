use crate::CellAlign;
use crate::CellBorders;
use crate::CellProps;
use crate::Checkbox;
use crate::DataFormat;
use crate::Dropdown;
use crate::Highlight;
use crate::Note;
use crate::ProgressBar;
use crate::TableCell;
use crate::Val;
use alloc::vec::Vec;
use hashbrown::HashMap;
use hashbrown::hash_map::Entry;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MaxMap<T> {
    pub hm: HashMap<usize, T>,
    pub maxk: Option<usize>,
}

pub struct MaxMapKeys<'a, T> {
    map: &'a MaxMap<T>,
    current: usize,
    end: usize,
}

impl<'a, T> Iterator for MaxMapKeys<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current <= self.end {
            let key = self.current;
            self.current += 1;
            if self.map.hm.contains_key(&key) {
                return Some(key);
            }
        }
        None
    }
}

pub struct MaxMapIter<'a, T> {
    map: &'a MaxMap<T>,
    current: usize,
    end: usize,
}

impl<'a, T> Iterator for MaxMapIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current <= self.end {
            let key = self.current;
            self.current += 1;
            if let Some(value) = self.map.hm.get(&key) {
                return Some((key, value));
            }
        }
        None
    }
}

pub struct MaxMapValues<'a, T> {
    iter: MaxMapIter<'a, T>,
}

impl<'a, T> Iterator for MaxMapValues<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub struct MaxMapIntoIter<T> {
    map: MaxMap<T>,
    keys: Vec<usize>,
    index: usize,
}

impl<T> Iterator for MaxMapIntoIter<T> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index];
            self.index += 1;
            if let Some(value) = self.map.hm.remove(&key) {
                return Some((key, value));
            }
        }
        None
    }
}

impl<T> MaxMap<T> {
    pub fn new() -> Self {
        MaxMap {
            hm: HashMap::new(),
            maxk: None,
        }
    }

    pub fn insert(&mut self, k: usize, v: T) {
        self.hm.insert(k, v);
        self.maxk = Some(self.maxk.map_or(k, |max| max.max(k)));
    }

    pub fn max_removed(&mut self) {
        if !self.hm.is_empty() {
            for k in (0..=self.maxk.unwrap_or(0)).rev() {
                if self.hm.contains_key(&k) {
                    self.maxk = Some(k);
                    break;
                }
            }
        } else {
            self.maxk = None;
        }
    }

    pub fn get(&self, key: &usize) -> Option<&T> {
        self.hm.get(key)
    }

    pub fn contains_key(&self, key: &usize) -> bool {
        self.hm.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get_maxk(&self) -> Option<usize> {
        self.maxk
    }

    pub fn iter(&self) -> MaxMapIter<'_, T> {
        let end = self.maxk.unwrap_or(0);
        MaxMapIter {
            map: self,
            current: 0,
            end,
        }
    }

    pub fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut T),
    {
        if let Some(maxk) = self.maxk {
            for k in 0..=maxk {
                if let Some(value) = self.hm.get_mut(&k) {
                    f(k, value);
                }
            }
        }
    }

    pub fn keys(&self) -> MaxMapKeys<'_, T> {
        let end = self.maxk.unwrap_or(0);
        MaxMapKeys {
            map: self,
            current: 0,
            end,
        }
    }

    pub fn values(&self) -> MaxMapValues<'_, T> {
        MaxMapValues { iter: self.iter() }
    }

    pub fn entry(&mut self, key: usize) -> Entry<'_, usize, T, hashbrown::DefaultHashBuilder> {
        self.hm.entry(key)
    }

    pub fn get_mut(&mut self, key: &usize) -> Option<&mut T> {
        self.hm.get_mut(key)
    }

    pub fn remove(&mut self, key: &usize) -> Option<T> {
        let result = self.hm.remove(key);
        if result.is_some() && self.maxk == Some(*key) {
            self.max_removed();
        }
        result
    }

    pub fn clear(&mut self) {
        self.hm.clear();
        self.maxk = None;
    }

    pub fn drain(&mut self) -> hashbrown::hash_map::Drain<'_, usize, T> {
        self.maxk = None;
        self.hm.drain()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&usize, &mut T) -> bool,
    {
        self.hm.retain(f);
        if !self.hm.is_empty() && self.maxk.is_some() {
            self.max_removed();
        } else {
            self.maxk = None;
        }
    }
}

impl<T> IntoIterator for MaxMap<T> {
    type Item = (usize, T);
    type IntoIter = MaxMapIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let keys: Vec<usize> = (0..=self.maxk.unwrap_or(0))
            .filter(|k| self.hm.contains_key(k))
            .collect();
        MaxMapIntoIter {
            map: self,
            keys,
            index: 0,
        }
    }
}

impl<'a, T> IntoIterator for &'a MaxMap<T> {
    type Item = (usize, &'a T);
    type IntoIter = MaxMapIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

macro_rules! impl_set_prop_maxmap {
    ($prop:ident, $ty:ty) => {
        #[inline]
        pub fn $prop(&mut self, pos: usize, prop: Option<$ty>, empty_val: &Val) {
            self.hm
                .entry(pos)
                .or_insert(TableCell {
                    val: empty_val.clone(),
                    props: None,
                })
                .props
                .get_or_insert_with(CellProps::new)
                .$prop = prop;
            self.maxk = Some(self.maxk.map_or(pos, |max| max.max(pos)));
        }
    };
}

impl MaxMap<TableCell> {
    impl_set_prop_maxmap!(align, CellAlign);
    impl_set_prop_maxmap!(borders, CellBorders);
    impl_set_prop_maxmap!(checkbox, Checkbox);
    impl_set_prop_maxmap!(data_format, DataFormat);
    impl_set_prop_maxmap!(dropdown, Dropdown);
    impl_set_prop_maxmap!(highlight, Highlight);
    impl_set_prop_maxmap!(note, Note);
    impl_set_prop_maxmap!(progress_bar, ProgressBar);

    #[inline]
    pub fn readonly(&mut self, pos: usize, prop: bool, empty_val: &Val) {
        self.hm
            .entry(pos)
            .or_insert(TableCell {
                val: empty_val.clone(),
                props: None,
            })
            .props
            .get_or_insert_with(CellProps::new)
            .readonly = prop;
        self.maxk = Some(self.maxk.map_or(pos, |max| max.max(pos)));
    }

    #[inline]
    pub fn set_cell_val(&mut self, pos: usize, val: Val) {
        if let Some(cell) = self.hm.get_mut(&pos) {
            cell.val = val;
        } else {
            self.hm.insert(pos, TableCell { val, props: None });
        }
        self.maxk = Some(self.maxk.map_or(pos, |max| max.max(pos)));
    }

    #[inline]
    pub fn set_cell(&mut self, pos: usize, cell: TableCell) {
        self.hm.insert(pos, cell);
        self.maxk = Some(self.maxk.map_or(pos, |max| max.max(pos)));
    }
}
