fn main() {
    println!("Hello, world!");
    print!("Bro");
}

#[test]
fn test_hello_test() {
    println!("Hello, Test");
}

// variable let bersifat imutable
#[test]
fn test_variable() {
    let name = "Ricky Adam Saputra";

    // name = "budi"; imutable tidak bisa diubah
    println!("Hello {}", name)
}

// Cara agar variable mutable
#[test]
fn test_var_mut() {
    let mut name = "Ricky Adam Saputra";
    println!("Hello {}", name);

    name = "Didi Nugraha";
    println!("Hello {}, value mutable", name);
}

#[test]
fn shadowing() {
    let name = "budi";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn explicit_int() {
    let age: i64 = 122;
    println!("Bro my age is {}", age);

    let panjang: f32 = 20.4;
    println!("Panjang sisi nya {}", panjang)
}

#[test]
fn range_integer() {
    let a: i8 = 10;
    println!("Hello {}", a);

    let b: i16 = a as i16;
    print!("Hello {}", b);

    let c: i32 = b as i32;
    println!("Hello {}", c);
}

#[test]
fn numeric() {
    let a = 10;
    let b = 10;
    println!("Nilai {} + {} = {}", a, b, a+b);
    println!("Nilai {} - {} = {}", a, b, a-b);
    println!("Nilai {} * {} = {}", a, b, a*b);
    println!("Nilai {} / {} = {}", a, b, a/b);
    println!("Nilai {} % {} = {}", a, b, a%b);
}


#[test]
fn augmented_assigment() {
    let mut a = 10;
    println!("Awal {}", a);

    a+=10;
    println!("Augment (+) = {}", a);

    a-=10;
    println!("Augment (-) = {}", a);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 10;

    let check = a > b;
    println!("check = {}", check);
}

#[test]
fn boolean_operator() {
    let nilai = 80;
    let absen = 80;

    let nilai_lulus = nilai > 75;
    let absen_lulus = absen > 75;

    let lulus = nilai_lulus && absen_lulus;
    println!("Anda dinyatakan {}", lulus);
}

#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';
    print!("{} and {}", char1, char2);
}

#[test]
fn tuple_type() {
    let data: (bool, i8, char) = (true, 12, '1');

    println!("Datanya: {:?}", data);

    let a = data.0;
    let b = data.1;

    println!("data Boolean = {} and data Integer = {}", a, b);
}

#[test]
fn destruction_tuple() {
    let data = (true, 12, '1');
    println!("Datanya: {:?}", data);

    let (a, b, _c) = data; // melakukan pengecualian dengan memakai (_var)
    println!("Data tersedia = {} {}", a, b);
}










