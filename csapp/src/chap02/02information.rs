#!/bin/sh
//usr/bin/env rustc $0 -o a.out && ./a.out && rm ./a.out ; exit

fn main() {
    // println!("overflow: {}", 200_i32 * 300 * 400 * 500); // compiler error
    println!("Practice Problem 2.2.D hex: {:X}", 0b1100100010110110010110);

    println!();
    print_raw(12345i32);
    print_raw(12345.0f32);
    print_raw(&12345i32);
    print_raw(123455678i32);
    print_raw(2607352i32);
    print_raw(2607352.0f32);
    print_raw(b"12345");
    print_raw(b"mnopqr");

    println!();
    for (ori, res) in [
        ("~0x41", !(0x41i8)),
        ("~0x00", !(0x00i8)),
        ("0x69 & 0x55", 0x69i8 & 0x55i8),
        ("0x69 | 0x55", 0x69i8 | 0x55i8),
    ] {
        println!("{ori: >20} {res: >8b} {res:x}");
    }

    println!();
    let val = 0x87654321u32;
    println!("0x{: >8x}", val);
    println!("0x{: >8x}", val & 0xff);
    println!("0x{: >8x}", (!val) & (! 0xff) | (val & 0xff)  );
    println!("0x{: >8x}", (val & (! 0xff)) | (0xff));

    println!();
    println!("{:0>8b}", 0x55i8);
    println!("{:0>8b}", 0x46i8);
    println!("{:x}", 0b01010101);

    println!();
    for v in [0x2e0, -0x58i32, 0x28i32, -0x30, 0x78, 0x88, 0x1f8, 0x8, 0xc0, -0x48] {
        println!("\"0x{:0>8x}\" = {:0>32b} = {}", v, v, v);
    }

    println!();
    println!("{:0>32b}", unsafe { std::mem::transmute::<_, i32>(std::f32::NAN) });
    println!("{:0>32b}", unsafe { std::mem::transmute::<_, i32>(std::f32::INFINITY) });
    println!("{:0>32b}", unsafe { std::mem::transmute::<_, i32>(std::f32::NEG_INFINITY) });
    println!("{:0>32b}", 0x00359141);
    println!("{}", 0x00359141);
    println!("{:b}", 148);
    println!("{:x}", 0b01001010010101100100010100000100);
    // println!("{}", 1.0f64 / std::f64::INFINITY);
}

fn print_raw<T: std::fmt::Debug>(number: T) {
    let len = std::mem::size_of_val(&number);
    print!("value={:?} address={:p} len={}>>", number, &number, len);
    for ch in unsafe { std::slice::from_raw_parts(&number as *const _ as *const i8, len) } {
        print!("{:02x} ", ch);
        // print!("{:02x}({:0b}) ", ch, ch);
    }
    println!();
}
