#![allow(unused)]

// use rand::Rng;
use std::{fmt::Result, io};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut jari = String::new();
    println!("Masukkan Nama Anda :");
    io::stdin().read_line(&mut input)?;
    println!("Masukkan panjang jari jari lingkaran: ");
    io::stdin().read_line(&mut jari)?;
    let jejari: f32 = jari.trim().parse().unwrap();
    let mut luas: f32 = luas_lingkaran(jejari);

    luas = luas * 1000.0;
    println!("Saudara {input} kita akan menghitung luas lingkaran dengn {jari}");
    println!("Luas lingkaran : {}", luas);
    Ok(())
}

fn luas_lingkaran(jari_jari: f32) -> f32 {
    3.14 * (jari_jari * jari_jari)
}
