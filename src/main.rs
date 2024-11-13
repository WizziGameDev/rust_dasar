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

#[test]
fn tuple_mutable () {
    let mut data = (true, 12, '1');
    println!("Data lama: {:?}", data);

    data.0 = false;
    println!("Data baru: {:?}", data);
}

#[test]
fn array () {
    let data: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Data Array {:?}", data);

    // Akses data nya
    println!("Data ke-1 {}", data[0]);
}

#[test]
fn array_mutable() {
    let mut data: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Data Array = {:?}", data);

    data[0] = 10;
    data[1] = 20;
    println!("Data Array Baru = {:?}", data);

    let long_array = data.len();
    println!("Panjang Array = {}", long_array)
}

#[test]
fn array_2d_immutable() {
    let data: [[i32;4];2] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
    ];

    println!("Data Array = {:?}", data); // ambil semua data
    println!("{}", data[1][2]);
}

#[test]
fn array_2d_mutable() {
    let mut data: [[i32;4];2] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
    ];

    data[1][1] = 10;

    println!("Data Array = {:?}", data); // ambil semua data
    println!("{}", data[1][1]);
}

#[test]
fn constant() {
    const MINIMUM: i8 = 10;

    println!("{}", MINIMUM);
}

#[test]
fn variable_scope() {
    let a = 10; // outer variable
    {
        let b = 10; // inner variable
        println!("{}", b);
        println!("{}", a+b);
    }
    println!("{}", a);
    // println!("{}", b); // error!
}

#[test]
fn str_type() {
    let lama = "   Broo   ";
    let baru = lama.trim();

    println!("Lama = {}", lama);
    println!("Baru = {}", baru);
}

#[test]
fn string_type() {
    let mut nama: String = String::from("Adam");
    println!("nama awal = {}", nama); // Adam

    nama = String::from("Didi");
    println!("nama diubah = {}", nama); // Didi

    // Data akan ditambahkan di belakangnya, namun di heap tetap sama 1 variable
    nama.push_str(" Budi");
    println!("{}", nama);

    let data_baru = nama.replace("Didi", "adam");
    println!("data baru = {}", data_baru);
}