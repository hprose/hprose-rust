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
 * io/util.rs                                             *
 *                                                        *
 * io util for Rust.                                      *
 *                                                        *
 * LastModified: Sep 28, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use std::{i64, str};

use time::now;

const DIGITS: &'static [u8] = b"0123456789";

const DIGIT2: &'static [u8] = b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

const DIGIT3: &'static [u8] = b"\
    000001002003004005006007008009010011012013014015016017018019\
    020021022023024025026027028029030031032033034035036037038039\
    040041042043044045046047048049050051052053054055056057058059\
    060061062063064065066067068069070071072073074075076077078079\
    080081082083084085086087088089090091092093094095096097098099\
    100101102103104105106107108109110111112113114115116117118119\
    120121122123124125126127128129130131132133134135136137138139\
    140141142143144145146147148149150151152153154155156157158159\
    160161162163164165166167168169170171172173174175176177178179\
    180181182183184185186187188189190191192193194195196197198199\
    200201202203204205206207208209210211212213214215216217218219\
    220221222223224225226227228229230231232233234235236237238239\
    240241242243244245246247248249250251252253254255256257258259\
    260261262263264265266267268269270271272273274275276277278279\
    280281282283284285286287288289290291292293294295296297298299\
    300301302303304305306307308309310311312313314315316317318319\
    320321322323324325326327328329330331332333334335336337338339\
    340341342343344345346347348349350351352353354355356357358359\
    360361362363364365366367368369370371372373374375376377378379\
    380381382383384385386387388389390391392393394395396397398399\
    400401402403404405406407408409410411412413414415416417418419\
    420421422423424425426427428429430431432433434435436437438439\
    440441442443444445446447448449450451452453454455456457458459\
    460461462463464465466467468469470471472473474475476477478479\
    480481482483484485486487488489490491492493494495496497498499\
    500501502503504505506507508509510511512513514515516517518519\
    520521522523524525526527528529530531532533534535536537538539\
    540541542543544545546547548549550551552553554555556557558559\
    560561562563564565566567568569570571572573574575576577578579\
    580581582583584585586587588589590591592593594595596597598599\
    600601602603604605606607608609610611612613614615616617618619\
    620621622623624625626627628629630631632633634635636637638639\
    640641642643644645646647648649650651652653654655656657658659\
    660661662663664665666667668669670671672673674675676677678679\
    680681682683684685686687688689690691692693694695696697698699\
    700701702703704705706707708709710711712713714715716717718719\
    720721722723724725726727728729730731732733734735736737738739\
    740741742743744745746747748749750751752753754755756757758759\
    760761762763764765766767768769770771772773774775776777778779\
    780781782783784785786787788789790791792793794795796797798799\
    800801802803804805806807808809810811812813814815816817818819\
    820821822823824825826827828829830831832833834835836837838839\
    840841842843844845846847848849850851852853854855856857858859\
    860861862863864865866867868869870871872873874875876877878879\
    880881882883884885886887888889890891892893894895896897898899\
    900901902903904905906907908909910911912913914915916917918919\
    920921922923924925926927928929930931932933934935936937938939\
    940941942943944945946947948949950951952953954955956957958959\
    960961962963964965966967968969970971972973974975976977978979\
    980981982983984985986987988989990991992993994995996997998999";

const MIN_I64_BUF: &'static [u8] = b"-9223372036854775808";

pub fn get_int_bytes(buf: &mut [u8], mut i: i64) -> &[u8] {
    if i == 0 {
        buf[0] = '0' as u8;
        return &buf[..1]
    }

    if i == i64::MIN {
        return MIN_I64_BUF
    }

    let mut sign = '+';
    if i < 0 {
        sign = '-';
        i = -i;
    }

    let mut off = buf.len();
    let mut q: i64;
    let mut p: i64;
    while i >= 100 {
        q = i / 1000;
        p = (i - (q * 1000)) * 3;
        i = q;
        off -= 3;
        buf[off] = DIGIT3[p as usize];
        buf[off + 1] = DIGIT3[p as usize + 1];
        buf[off + 2] = DIGIT3[p as usize + 2];
    }
    if i >= 10 {
        q = i / 100;
        p = (i - (q * 100)) * 2;
        i = q;
        off -= 2;
        buf[off] = DIGIT2[p as usize];
        buf[off + 1] = DIGIT2[p as usize + 1];
    }
    if i > 0 {
        off -= 1;
        buf[off] = DIGITS[i as usize];
    }
    if sign == '-' {
        off -= 1;
        buf[off] = sign as u8;
    }
    return &buf[off..]
}

pub fn get_uint_bytes(buf: &mut [u8], mut i: u64) -> &[u8] {
    if i == 0 {
        buf[0] = '0' as u8;
        return &buf[..1]
    }

    let mut off = buf.len();
    let mut q: u64;
    let mut p: u64;
    while i >= 100 {
        q = i / 1000;
        p = (i - (q * 1000)) * 3;
        i = q;
        off -= 3;
        buf[off] = DIGIT3[p as usize];
        buf[off + 1] = DIGIT3[p as usize + 1];
        buf[off + 2] = DIGIT3[p as usize + 2];
    }
    if i >= 10 {
        q = i / 100;
        p = (i - (q * 100)) * 2;
        i = q;
        off -= 2;
        buf[off] = DIGIT2[p as usize];
        buf[off + 1] = DIGIT2[p as usize + 1];
    }
    if i > 0 {
        off -= 1;
        buf[off] = DIGITS[i as usize];
    }
    return &buf[off..]
}

pub fn utf16_len(s: &str) -> usize {
    let bytes = s.as_bytes();
    let len = s.len();
    let mut n = len;
    let mut p = 0;
    while p < len {
        let a = bytes[p];
        match a >> 4 {
            0 ... 7 => p += 1,
            12 | 13 => {
                p += 2;
                n -= 1;
            },
            14 => {
                p += 3;
                n -= 2;
            },
            15 => {
                if a & 8 == 8 {
                    unreachable!()
                }
                p += 4;
                n -= 2;
            }
            _ => unreachable!()
        }
    }
    n
}

#[inline]
pub fn utf8_slice_to_str(v: &[u8]) -> &str {
    unsafe { str::from_utf8_unchecked(v) }
}

// GetDateBytes returns the []byte representation of year, month and day.
// The format of []byte returned is 20060102
// buf length must be greater than or equal to 8
pub fn get_date_bytes(buf: &mut [u8], year: i32, month: i32, day: i32) -> &[u8] {
    let q = year / 100;
    let mut p = q << 1;
    buf[0] = DIGIT2[p as usize];
    buf[1] = DIGIT2[p as usize + 1];
    p = (year - q * 100) << 1;
    buf[2] = DIGIT2[p as usize];
    buf[3] = DIGIT2[p as usize + 1];
    p = month << 1;
    buf[4] = DIGIT2[p as usize];
    buf[5] = DIGIT2[p as usize + 1];
    p = day << 1;
    buf[6] = DIGIT2[p as usize];
    buf[7] = DIGIT2[p as usize + 1];
    &buf[..8]
}

// GetTimeBytes returns the []byte representation of hour, min and sec.
// The format of []byte returned is 150405
// buf length must be greater than or equal to 6
pub fn get_time_bytes(buf: &mut [u8], hour: i32, min: i32, sec: i32) -> &[u8] {
    let mut p = hour << 1;
    buf[0] = DIGIT2[p as usize];
    buf[1] = DIGIT2[p as usize + 1];
    p = min << 1;
    buf[2] = DIGIT2[p as usize];
    buf[3] = DIGIT2[p as usize + 1];
    p = sec << 1;
    buf[4] = DIGIT2[p as usize];
    buf[5] = DIGIT2[p as usize + 1];
    &buf[..6]
}

// GetNsecBytes returns the []byte representation of nsec.
// The format of []byte returned is 123, 123456 or 123456789
// buf length must be greater than or equal to 9
pub fn get_nsec_bytes(buf: &mut [u8], mut nsec: i32) -> &[u8] {
    let mut q = nsec / 1000000;
    let mut p = q * 3;
    nsec = nsec - q * 1000000;
    buf[0] = DIGIT3[p as usize];
    buf[1] = DIGIT3[p as usize + 1];
    buf[2] = DIGIT3[p as usize + 2];
    if nsec == 0 {
        return &buf[..3]
    }
    q = nsec / 1000;
    p = q * 3;
    nsec = nsec - q * 1000;
    buf[3] = DIGIT3[p as usize];
    buf[4] = DIGIT3[p as usize + 1];
    buf[5] = DIGIT3[p as usize + 2];
    if nsec == 0 {
        return &buf[..6];
    }
    p = nsec * 3;
    buf[6] = DIGIT3[p as usize];
    buf[7] = DIGIT3[p as usize + 1];
    buf[8] = DIGIT3[p as usize + 2];
    &buf[..9]
}

#[inline]
pub fn bytes_to_diget2(bytes: &[u8]) -> i32 {
    (bytes[0] - b'0') as i32 * 10 + (bytes[1]  - b'0') as i32
}

#[inline]
pub fn bytes_to_diget3(bytes: &[u8]) -> i32 {
    (bytes[0]  - b'0') as i32 * 100 + (bytes[1] - b'0') as i32 * 10 + (bytes[2] - b'0') as i32
}

#[inline]
pub fn bytes_to_diget4(bytes: &[u8]) -> i32 {
    (bytes[0] - b'0') as i32 * 1000 + (bytes[1] - b'0') as i32 * 100 + (bytes[2] - b'0') as i32 * 10 + (bytes[3] - b'0') as i32
}

use std::cell::RefCell;

thread_local! {
    static UTCOFF: RefCell<Option<i32>> = RefCell::new(None);
}

pub fn get_utcoff() -> i32 {
    UTCOFF.with(|f| {
        if (*f.borrow()).is_none() { *f.borrow_mut() = Some(now().tm_utcoff); }
            (*f.borrow()).unwrap()
    })
}

#[cfg(test)]
mod tests {
    use std::*;

    use super::*;

    #[test]
    fn test_get_int_bytes() {
        let data = [
            0, 9, 10, 99, 100, 999, 1000, -1000, 10000, -10000, 123456789, -123456789,
            i32::MAX as i64, i32::MIN as i64, i64::MAX, i64::MIN
        ];
        let mut buf: [u8; 20] = [0; 20];
        for i in &data {
            assert!(get_int_bytes(&mut buf, *i) == i.to_string().as_bytes(), r#"b must be []byte("{:?}")"#, i.to_string().as_bytes());
        }
    }

    #[test]
    fn test_get_uint_bytes() {
        let data = [
            0, 9, 10, 99, 100, 999, 1000, 10000, 123456789,
            i32::MAX as u64, u32::MAX as u64, i64::MAX as u64, u64::MAX
        ];
        let mut buf: [u8; 20] = [0; 20];
        for i in &data {
            assert!(get_uint_bytes(&mut buf, *i) == i.to_string().as_bytes(), r#"b must be []byte("{:?}")"#, i.to_string().as_bytes());
        }
    }

    #[test]
    fn test_utf16_len() {
        let test_cases = [
            ("", 0),
            ("π", 1),
            ("你", 1),
            ("你好", 2),
            ("你好啊,hello!", 10),
            ("🇨🇳", 4)
        ];
        for test_case in &test_cases {
            assert!(utf16_len(test_case.0) == test_case.1, r#"The utf16 len of "{}" must be {}"#, test_case.0, test_case.1);
        }
    }
}
