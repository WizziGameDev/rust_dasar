# Belajar Rust Dasar

Seputar rust, rust merupakan bahasa pemrograman dengan garbage collection yang stabil, jadi tidak ada jeda time berapa miliseconds. Rust memiliki Package Manager bawaan dengan nama cargo. Di rust kamu akan dipaksa untuk membuat code secara optimal, tidak seperti bahasa pemrograman lainnya.

Untuk start awal project yang akan di jalankan dari file yang ada fn main() {}. Jika di satu folder ada 2 main agan terjadi error

Beberapa fitur cargo
- cargo build												| Untuk melakukan run project nya
- cargo build --release							| Final binary kita

Untuk pembelajaran agar bisa menjalankan code programnya kita bisa memakai Unit Test dengan 

```rust
#[test]
```

## Variable (immutable)
Variable di rust cukup simple dan bersifat imutable dimana ketika melakukan inisiasi value nya tidak bisa diubah (let)
contoh:

```rust
#[test]
fn test_variable() {
    let name = "Ricky Adam Saputra";

    // name = "budi"; imutable tidak bisa diubah
    println!("Hello {}", name)
}
```

## Variable (mutable)
Dengan menambahkan mut setelah let kita dapat membuat variable bersifat mutable, perlu diingat walaupun bisa diubah value nya kita tidak bisa mengubah tipe datanya seperti di java perlu melakukan konversi terlebih dahulu. Kita akan mempelajar konversi di materi" selanjutnya 

Implementasi:
```rust
#[test]
fn test_var_mut() {
    let mut name = "Ricky Adam Saputra";
    println!("Hello {}", name);

    name = "Didi Nugraha";
    println!("Hello {}, value mutable", name);
}
```

## Shadowing
Di rust kita diperbolehkan melakukan shadowing walaupun praktiknya akan membingungkan saat membaca code nya. Di bahasa pemrograman lain tidak bisa. Setiap variable yang diinisiasi apabila sama yang variable sebelumnya akan berlaku shadow, untuk peletakan setiap variable itu seperti membuat data yang berbeda namun nama yang sama.

Implementasi:
```rust
#[test]
fn shadowing() {
    let name = "budi";
    println!("Hello {}", name);
    
    let name = 10;
    println!("Hello {}", name);
}
```

Perlu diperhatikan jika ada variable shadowing, data variable yang akan di ambil yang sebelumnya atau ketika melakukan print atau operasi apapun. Tidak diambil variable yang awal jika melakukan print variable setelah yang ke-2

## Commant
Seperti commant di java menggunakan (//) dan multi command (/* */)

## Data Type
Terdapat 2 type data, Scalar dan Compound
Scalar:
- Integer
- Float
- Boolean
- Char

Compound
- Tuple        | Berisi data dengan type Scalar bisa berbeda"
- Array        | Berisi data dengan type Scalar yang sama

## Explisit Type
Kita dapat secara explisit mendefinisikan berapa panjang Integer maupun Float yang kita pakai. 
Seperti bisa untuk rentang type Integer sendiri :
SIGNED                UNSIGNED
- i8                  u8
- i16                 u16
- i32                 u32
- i64                 u64
- i128                u128
Secara default untuk type Integer memiliki panjang i32

Untuk Float:
- f32
- f64
Default untuk Float sendiri f64

Implementasi
```rust
#[test]
fn explicit_int() {
    let mut age: i64 = 122;
    println!("Bro my age is {}", age);

    let mut panjang: f32 = 20.4;
    println!("Panjang sisi nya {}", panjang)
}
```

## Konversi Ke Jangkauan Tertentu
Untuk melakukan pengubahan jangkauan dari bit seperti dari i32 ke i16 misalnya kita harus memastikan bahwa value nya tidak melebihi kapasitas yang dimiliki oleh i16. Jika melebihi akan terjadi Integer overflow.

Cara penggunaannya dengan let name_variable = value_var as type_range

Implementasi
```rust
#[test]
fn range_integer() {
    let a: i8 = 10;
    println!("Hello {}", a);

    let b: i16 = a as i16;
    print!("Hello {}", b);

    let c: i32 = b as i32;
    println!("Hello {}", c);
}
```

## Numeric Operation
Seperti halnya bahasa pemrograman lainnya. Terdapat +, -, *, /, %.

Implementasi
```rust
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
```

## Augmented Assigment
Untuk penggunaan penjumlahan maupun operasi lainnya dengan dirinnya sendiri, harus melakukan inisiasi variable dengan sifat mutable (let mut).

Implementasi
```rust
#[test]
fn augmented_assigment() {
    let mut a = 10;
    println!("Awal {}", a);
    
    a+=10;
    println!("Augment (+) = {}", a);
    
    a-=10;
    println!("Augment (-) = {}", a);
}
```

## Boolean
Cukup simple karena hany abernilai true dan false

Implementasi
```rust
#[test]
fn comparison() {
    let a = 10;
    let b = 10;

    let check = a > b;
    println!("check = {}", check);
}
```

## Operasi Boolean
Terdapat 3 operasi boolean, seperti biasa sama halnya dengan gerbang logika:

&& (and)
|| (or)
! (not)

Implementasi
```rust
#[test]
fn boolean_operator() {
    let nilai = 80;
    let absen = 80;

    let nilai_lulus = nilai > 75;
    let absen_lulus = absen > 75;

    let lulus = nilai_lulus && absen_lulus;
    println!("Anda dinyatakan {}", lulus);
}
```

## Char
Untuk type data ini akan memakai petik satu untuk value nya ('...')

Implementasi
```rust
#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';
    print!("{} and {}", char1, char2);
}
```

## Tuple
Untuk type ini datanya bisa bermacam macamn namun datanya ketika sudah diinisiasi di variable nya tidak bisa ditambah, dikurangi ataupun di ubah, hanya bisa melakukan akses dengan menggunakan index nya (dimulai dari 0). Untuk penggunaannya memakai (..., ..., ...)

Data Tuple Bersifat Imutable namun kita bisa melakukan agar data Tuple dapat bersifat Mutable, penjelasan di materi selanjutnya

Implementasi
```rust
#[test]
fn tuple_type() {
    let data: (bool, i8, char) = (true, 12, '1');

    println!("Datanya: {:?}", data);

    let a = data.0;
    let b = data.1;
    
    println!("data Boolean = {} and data Integer = {}", a, b);
}
```

## Destruction Tuple
Seperti melakukan extract tuple ke dalam sebuah variable agar lebih mudah dalam akses dan bisa melakukan pengecualian data yang ingin di extract dengan cara menambahkan underscore (_) sebelum nama variable nya.

Implementasi
```rust
#[test]
fn destruction_tuple() {
    let data = (true, 12, '1');
    println!("Datanya: {:?}", data);

    let (a, b, _c) = data; // melakukan pengecualian dengan memakai (_var)
    println!("Data tersedia = {} {}", a, b);
}
```

## Tuple Mutable
Dengan melakukan inisiasi pembuatan variable dengan menambahkan mut setelah let akan membuat Compound Tuple bersifat mutable 

Implementasi
```
#[test]
fn tuple_mutable () {
    let mut data = (true, 12, '1');
    println!("Data lama: {:?}", data);

    data.0 = false;
    println!("Data baru: {:?}", data); 
}
```

## Array
Di array datanya harus sejenis tidak bisa bercampus type datanya. Untuk inisiasi menggunakan [..., ..., ...]. Untuk array sendiri tidak bisa memanjang dan mengecil, ketika sudah ditentukan panjangnya tidak bisa dikurang atau ditambah.

Implementasi
```rust
#[test]
fn array () {
    let data: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Data Array {:?}", data);
    
    // Akses data nya
    println!("Data ke-1 {}", data[0]);
}
```

Untuk i32 itu besarnya setiap data, dan untuk 5 itu panjang yang ada di array nya. Untuk aksesnya seperti umumnya memakai var[index_ke]. Tetep memakai index yang diawali dari 0


## Imutable Array
Dengan array yang bersifat imutable kita dapat mengubah value dari setiap index nya.

Implementasi
```rust
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
```
Untuk melihat panjang array nya kita bisa memakai .len()

## Array 2D
Seperti umumnya cara menggambil data dari array 2D. Kita bisa menambahkan format secara explisit [[i32;4];2] namun juga langsung melakukan inisiasi value nya bisa tanpa harus mendefinisikan secara explisit.

[i32;4] -> untuk detail arraynya
[2] -> untuk jumlah array 2d nya

Implementasi Immutable
```rust
#[test]
fn array_2d_immutable() {
    let data: [[i32;4];2] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
    ];

    println!("Data Array = {:?}", data); // ambil semua data
    println!("{}", data[1][2]);
}
```

Implementasi Mutable
```rust
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
```

## Constant
Untuk membuat variable dari const wajib secara explisit mendefinisikan tipe datanya. Untuk value nya wajib diisi langsung tidak boleh value dari yang lain/proses dari variable lain. Untuk nama variable diwajibkan UPPERCASE semua;

Perlu diperhatikan contant tidak ada mutable, jadi sifat nya immutable

Implementasi
```rust
#[test]
fn constant() {
    const MINIMUM: i8 = 10;

    println!("{}", MINIMUM);
}
```

## Variable Scope
Untuk variable scope outer variable bisa diakses di inner variable/didalam, namun inner variable tidak dapat diakses di outer variable/diluar

Implementasi
```
#[test]
fn variable_scope() {
    let a = 10; // outer variable
    {
        let b = 10; // inner variable
        println!("{}", b);
        println!("{}", a+b);
    }
    println!("{}", a);
    println!("{}", b); // error!
}
```

## MANAGEMENT MEMORY
Di rust untuk management memori tidak memakai cara kerja garbage collection maupun manual management. Di rust memakai konsep stack dan heap.

Untuk cara kerjannya setiap variabel maupun function akan di tumpak dengan konsep stack FILO (First In Last Out). Heap untuk apa? jadi untuk data yang tidak berkembang/fix seperti Scalar dan Compound akan dimasukkan ke Stack dan untuk yang berkembang seperti String akan dimasukkan ke Heap. Namun konsep penghubung dari stack ke heap menggunakan pointer. Walaupun disimpan di Heap untuk data mengembang sebenarnya pointernya tetap di stack yang dihubungkan ke heap.

Setiap data yang selesai di satu function/variable yang keluar di scope nya akan di hapus otomatis, jadi setiap proses yang selesai akan dihapus yang dimulai dari variable maupun function terakhir.

## &str
slice string bersifat imutable, jika diubah ke mut (mutable) dan dijalankan akan terjadi error saat proses compile walaupun di code nya tidak error.  Untuk &str terletak di stack untuk memori management nya.

Implementasi
```rust
#[test]
fn str_type() {
    let lama = "   Broo   ";
    let baru = lama.trim();

    println!("Lama = {}", lama);
    println!("Baru = {}", baru);
}
```
	
## String
Untuk String sendiri bisa dilakukan secara imutable maupun mutable. Untuk mutable nya bisa mengubah value dari variable nya. Dan untuk funtion replace cara kerjanya akan membuat data baru di heap nya. Karena String bisa memanjang value nya akan di taruh di heap walaupun di buat bersifat imutable akan tetap ditaruh di heap

Implementasi
```
#[test]
fn string_type() {
    let mut nama: String = String::from("Adam");
    println!("nama awal = {}", nama); // Adam

    nama = String::from("Didi");
    println!("nama diubah = {}", nama); // Didi

    // Data akan ditambahkan di belakangnya, namun di heap tetap sama 1 variable
    nama.push_str(" Budi");
    println!("{}", nama);
    
    // Ketika melakukan replace, di heap nya akan membuat data baru dengan nama variable 
    // yang menyimpan dari data yang akan di replace
    let data_baru = nama.replace("Didi", "adam");
    println!("data baru = {}", data_baru);
}
```

## Ownership Rules
Untuk ownership seperti pembahasan sebelumnya ketika variable atau apapun itu di dalam scope akan dihapus apabila sudah keluar. Jadi ketika proses selesai/keluar dari scope tersebut ada dihapus otomatis dimomory nya

Implementasi
```rust
#[test]
fn ownership() {
    // variable a tidak bisa digunakan di sebelumnya, belum dideklarasi
    let a: i32 = 10; // variable a bisa diakses sekarang

    {   // variable b tidak bisa digunakan sebelumnya, belum dideklarasi
        let b: i32 = 20; // sekarang bis digunakan
        println!("{}", b);
    }   // b/scope ini selesai dan dihapus dari memory, tidak bisa diakses lagi

    println!("{}", a);
    // scope a selesai, a/scope ini akan dihapus dari memory
}
```





