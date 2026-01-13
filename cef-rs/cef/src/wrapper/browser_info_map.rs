use std::{collections::BTreeMap, ops::ControlFlow};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BrowserInfoMapVisitorResult {
    /// Remove the entry from the map.
    RemoveEntry,
    /// Keep the entry in the map.
    KeepEntry,
}

/// Implement this interface to visit and optionally delete objects in the map.
pub trait BrowserInfoMapVisitor<K: Copy + Ord, V: Clone> {
    fn on_next_info(
        &self,
        browser_id: i32,
        key: K,
        value: &V,
    ) -> ControlFlow<BrowserInfoMapVisitorResult, BrowserInfoMapVisitorResult>;
}

#[derive(Default)]
pub struct BrowserInfoMap<K: Clone + Ord, V: Clone> {
    map: BTreeMap<i32, BTreeMap<K, V>>,
}

impl<K: Copy + Ord, V: Clone> BrowserInfoMap<K, V> {
    /// Add an object associated with the specified ID values.
    pub fn insert(&mut self, browser_id: i32, key: K, value: V) {
        self.map.entry(browser_id).or_default().insert(key, value);
    }

    /// Find the object with the specified ID values. |visitor| can optionally be
    /// used to evaluate or remove the object at the same time. If the object is
    /// removed using the Visitor the caller is responsible for destroying it.
    pub fn find(
        &mut self,
        browser_id: i32,
        key: K,
        visitor: Option<&dyn BrowserInfoMapVisitor<K, V>>,
    ) -> Option<V> {
        let info_map = self.map.get_mut(&browser_id)?;
        let entry = info_map.get(&key)?;

        if let Some(visitor) = visitor {
            let result = match visitor.on_next_info(browser_id, key, entry) {
                ControlFlow::Break(result) => result,
                ControlFlow::Continue(result) => result,
            };

            if result == BrowserInfoMapVisitorResult::RemoveEntry {
                let entry = info_map.remove(&key);
                if info_map.is_empty() {
                    self.map.remove(&browser_id);
                }
                return entry;
            }
        }

        Some(entry.clone())
    }

    /// Find all objects. If any objects are removed using the Visitor the caller
    /// is responsible for destroying them.
    pub fn find_all(&mut self, visitor: &dyn BrowserInfoMapVisitor<K, V>) {
        let browser_ids: Vec<_> = self.map.keys().copied().collect();
        for browser_id in browser_ids {
            let info_map = self
                .map
                .get_mut(&browser_id)
                .expect("missing browser info map");

            let mut keep_going = true;
            let mut removed = vec![];
            let keys: Vec<_> = info_map.keys().copied().collect();
            for key in keys {
                let value = info_map.get(&key).expect("missing value");
                let result = visitor.on_next_info(browser_id, key, value);
                let (stop, result) = match result {
                    ControlFlow::Break(result) => (true, result),
                    ControlFlow::Continue(result) => (false, result),
                };

                if result == BrowserInfoMapVisitorResult::RemoveEntry {
                    removed.push(key);
                }

                if stop {
                    keep_going = false;
                    break;
                }
            }

            for key in removed {
                info_map.remove(&key);
            }

            if info_map.is_empty() {
                self.map.remove(&browser_id);
            }

            if !keep_going {
                break;
            }
        }
    }

    /// Find all objects associated with the specified browser. If any objects are
    /// removed using the Visitor the caller is responsible for destroying them.
    pub fn find_browser_all(&mut self, browser_id: i32, visitor: &dyn BrowserInfoMapVisitor<K, V>) {
        let info_map = self
            .map
            .get_mut(&browser_id)
            .expect("missing browser info map");

        let mut removed = vec![];
        let keys: Vec<_> = info_map.keys().copied().collect();
        for key in keys {
            let value = info_map.get(&key).expect("missing value");
            let result = visitor.on_next_info(browser_id, key, value);
            let (stop, result) = match result {
                ControlFlow::Break(result) => (true, result),
                ControlFlow::Continue(result) => (false, result),
            };

            if result == BrowserInfoMapVisitorResult::RemoveEntry {
                removed.push(key);
            }

            if stop {
                break;
            }
        }

        for key in removed {
            info_map.remove(&key);
        }

        if info_map.is_empty() {
            self.map.remove(&browser_id);
        }
    }

    /// Returns true if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of objects in the map.
    pub fn len(&self) -> usize {
        self.map.values().map(|info_map| info_map.len()).sum()
    }

    /// Returns the number of objects in the map that are associated with the
    /// specified browser.
    pub fn browser_len(&self, browser_id: i32) -> usize {
        self.map
            .get(&browser_id)
            .map_or(0, |info_map| info_map.len())
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}
