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
 * LastModified: Sep 25, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

#![feature(question_mark)]
#![feature(specialization)]
#![feature(test)]

extern crate time;
extern crate uuid;

pub mod io;
pub mod rpc;
