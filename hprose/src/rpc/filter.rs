/**********************************************************\
|                                                          |
|                          hprose                          |
|                                                          |
| Official WebSite: http://www.hprose.com/                 |
|                   http://www.hprose.org/                 |
|                                                          |
\**********************************************************/
/**********************************************************\
 *                                                        *
 * rpc/filter.rs                                          *
 *                                                        *
 * hprose filter manager for Rust.                        *
 *                                                        *
 * LastModified: Oct 9, 2016                              *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

pub trait Filter: Send + Sync + 'static {
    fn input(&self, data: &[u8]) -> &[u8];
    fn output(&self, data: &[u8]) -> &[u8];
}

pub struct FilterManager {
    filters: Vec<Box<Filter>>
}

impl FilterManager {
    #[inline]
    pub fn new() -> FilterManager {
        FilterManager { filters: vec![] }
    }

    pub fn add_filter<F>(&mut self, filter: F) where F: Filter {
        self.filters.push(Box::new(filter));
    }
}

impl Filter for FilterManager {
    fn input(&self, data: &[u8]) -> &[u8] {
        unimplemented!()
    }

    fn output(&self, data: &[u8]) -> &[u8] {
        unimplemented!()
    }
}
