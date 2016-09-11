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
 * main.rs                                                *
 *                                                        *
 * hprose main for Rust.                                  *
 *                                                        *
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

#![feature(test)]

mod io;

use io::writer::Writer;

fn main() {
    let mut writer = Writer::new();
    writer
        .serialize(true)
        .serialize(false)
        .serialize(8)
        .serialize(std::f32::consts::PI)
        .serialize(std::f64::consts::PI);
    println!("{}", writer.string());
}