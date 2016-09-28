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
 * lib.rs                                                 *
 *                                                        *
 * hprose lib for Rust.                                   *
 *                                                        *
 * LastModified: Sep 28, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

#![feature(specialization)]
#![feature(test)]

extern crate test;

extern crate num;
extern crate time;
extern crate uuid;

pub mod io;
pub mod rpc;
