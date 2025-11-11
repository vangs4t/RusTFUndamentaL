use core::num;
use std::{ops::Index, path::StripPrefixError};

struct User {
    nama: String,
    umur: u32,
}

fn main() {
    let user = User {
        nama: String::from("Naufal"),
        umur: 23,
    };

    println!("{} berumur {}", user.nama, user.umur);
}

#[test]
fn numbers() {
    let n :i64 = 1_000_000_000;

    let s :i8 = n as i8;

    println!("{s}");
}

#[test]
fn tuplesd() {
    /* Cara membuat tuple tidak seperti array yang menggunakan kurung kotak
    tuple menggunakan kurung buka biasa, panjang tuple bersifat immutable
    namun index nya bisa di ganti, tuple bisa memuat berbagai macam tipe data tak seperti array
    yang hanya menginzinkan satu tipedata saja 
    cara pemanggilan index tuple bisa dengan menulis namavariable.nomorIndex*/
    let user :(&str, i32, f64) = ("Naufal", 2332, 155.5); 

    let nama = user.0;
    let id = user.1;
    let tinggi = user.2;

    println!("{},{},{}", nama, id, tinggi);

}
#[test]
fn array_feature() {
    let arr: [i32; 5] = [10,20,30,40,50];

    let result = arr[0]; // panggil dengan namavariable[nomor index]
    let panjang = arr.len(); // gunakan method len() untuk mencari panjang array
    let slicing = &arr[2..]; // slicing digunakan untuk memotong atau mencari suatu index

    println!("{:?}", arr);
    println!("{result}");
    println!("{panjang}");
    println!("{:?}",slicing);
}
// const MINIMUM :i32 = 20;

#[test]
fn constanta() {
    /*untuk membuat constant diperlukan tipe data explisit dan data nya langsung di isi
    kita juga diharuskan menulis nama variable nya dengan uppercase semua
    constant secara default bisa digunakan diluar function */
    const MAXIMUM :i32 = 100; 
    println!("{MAXIMUM}");

}

/*Aku baru tau kalau kita mengubah 
&str tidak mengubah data yang sudah ditentukan dan hanya mengganti 
nya dengan data baru penyimpanan nya juga bukan di heap tapi di stack 
amat sangat manuk akal
sedangkan String bisa melebar data nya dan tidak mengganti datanya sama sekali wow
amat sangat mantap */

#[test]
fn ifelse_statement() {
    let nilai = 75;
    let info = if nilai >= 75 {
        "Cukup"
    } else if nilai >= 80{
        "Baik"
    } else if nilai >= 90{
        "Bagus"
    } else {
        "Sangat Bagus"
    };

    println!("{info}");
}

#[test]
fn looping() {
    let mut n1 = 0;
    /*Ini merupakan contoh loop sederhana  */
    loop {
        n1 += 1;

        if n1 > 10 {
            break;
        } else if n1 %2 == 0 {
            continue;
        }

        println!("Counter : {n1}");
    }
}

#[test]
fn looping_in_let() {
    let mut  counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter *2;
        }
    };

    println!("{result}");
}

#[test]
fn while_loop() {
    let mut counter = 10;

    while counter > 0 {
        counter -= 1;

        println!("{counter}");
    };
}

fn recursive_function(n: i32) -> i32{
    if n == 1 {
        return 1;
    }

    n * recursive_function(n -1)
}
#[test]
fn faktorial(){
    let result = recursive_function(5);

    println!("{result}");
}

fn say_number(n :i32) -> i32{
    n *2
}

fn say_hai(say :&String){
    print!("Halo {}", say);
}

#[test]
fn ownership_function() {
    println!("halo nomor {}", say_number(5));

    let nama = String::from("Naufal");

    say_hai(&nama);
    /*Alangkah baiknya jika kita membuat parameter 
    yang datanya disimpan di heap menggunakan borrowing atau tanda & */
    println!("Halo {}", nama);
}

fn nama_variable(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

// #[test]
// fn the_name() {
//     let first_name = String::from("Naufal");
//     let last_name = String::from("Abdul");
//     let full_name = nama_variable(first_name, last_name);

//     println!("{}", full_name);
//     println!("{}", first_name);
//     println!("{}", last_name);

// }

/*Kalo mutable boorowing itu lifecycle nya hanya boleh ada satu */

fn the_borrow(secun : &mut String) {
    secun.push_str(" Jhonny");
}

#[test]
fn mutable_borrowing() {
    let mut variable = String::from("Naufal");

    /*
    Kita bisa mengubah data variable beberapakali 
    asal logikanya diluar scope variable induk */
    the_borrow(&mut variable); 
    the_borrow(&mut variable);
    the_borrow(&mut variable);

    println!("The name is :{}", variable);

}

#[test]
fn mutable_variables() {
    let mut variable = String::from("Naufal");

    let result= &mut variable;
    // let result1 = &mut variable;

    the_borrow(result);
    

    println!("The name is :{}", variable);

}

struct Coworkers{
    name: String,
    username :String,
    id : u32,
}
/*
Struct juga bisa memiliki turunan yaitu method */

impl Coworkers {
    // &self digunakan untuk memanggil field kalo di java itu this
    fn user(&self, nama : &str){
        println!("Halo {}, nama saya adalah {}", self.name, nama);
    }
    // ini adalah associated function atau di java static function
    fn new(input : u32) -> u32{
        input
    }
}

#[test]
fn calling_by_name() {
    
    let result = Coworkers{
        name : String::from("Naufal"),
        username : String::from("Jhonny").to_lowercase(),
        id : 2345,

    };

    result.user("Agartha");
    let id = Coworkers::new(23);

    println!("id anda : {}", id);
}

#[test]
fn user_data() {

    let name= String::from("Naufal b");
    let username = String::from("jhonnnycage");
    /*Untuk mengambil data di field struct kita bisa membuat variable
    dari struct nya terlebih dahulu
    lalu jika data di field struct nya belum di implementasi kan maka
    kita hanya perlu membuatnya di dalam variable seperti dibawah */
    let result = Coworkers{
        name : String::from("Naufal"),
        username : String::from("Jhonny").to_lowercase(),
        id : 2345,

    };
    // untuk pemanggilan field tiggal panggil variable.namafield

    println!("Halo ,{}", result.name);

    /*Kita juga bisa mengcopy semua isi field ke dalam variable baru 
    jika ingin 
    namun perlu di ingat kalau reference nya tidak di implementasikan otomatis 
    ownernya berpindah ke variable yang baru
    dan kita tak bisa mengakses field yang lama*/
    let result1 = Coworkers{..result};

    println!("Hao {}", result1.name);
    
    /*Sebenarnya kita juga bisa mengisi otomatis field dengan data jika
    kita membuat nama variable yang sama dengan nama field */
    let result2 = Coworkers{
        name, // kita bisa menggunakan variable yang sebelumnya dibuat 
        username, // untuk mengisi field asal nama nya sama
        ..result
    };

    println!("{}", result2.name);
    println!("{}", result2.username);
    println!("{}", result2.id);



}


struct Client{
    name : String,
    id : i32,
}

#[test]
fn simple() {
    let costumer = Client{
        name : String::from("Monalisa"),
        id : 719300,
    };

    println!("Selamat datang {}, dengan id {}", costumer.name, costumer.id);
}

struct Laptop {
    merek : String,
    harga : u32,
}

impl Laptop {
    fn info(&self) {
        println!("Laptop {} seharga {}", self.merek, self.harga);
    }
}

#[test]
fn mean() {
    let l = Laptop {
        merek: String::from("Lenovo"),
        harga: 8_000_000,
    };

    l.info();
}


struct Kordinat(f64, f64);

#[test]
fn position() {
    let posisi = Kordinat(-6.38244, 10.38539);

    let lat = posisi.0;
    let long = posisi.1;

    println!("{}", lat);
    println!("{}", long);
}

struct Server;

impl Server {
    fn start(){
        print!("Server terhubung...");
    }
}

#[test]
fn server_s() {
    Server::start();
}

struct Alamat<'a> {
    kota : &'a str,
    negara: &'a str,
}

struct Person<'a>{
    nama:  String,
    alamat :Alamat<'a>,
}

#[test]
fn manusa() {
    let alamat = Alamat{
        kota : "Kuningan",
        negara : "Jenapa",
    };

    let person = Person{
        nama : String::from("Naufal"),
        alamat,
    };

    println!("Nama :{}, Kota : {}, Negara : {}", person.nama, person.alamat.kota, person.alamat.negara); 
}

struct Sapa<'a>{
    pesan: &'a str,
}

#[test]
fn say_halo() {
    let halo = Sapa{
        pesan : "Halo rust",
    };

    println!("{}", halo.pesan);
}

/*Enum java sama enum rust punya banyak kesamaaan sih */
enum Gender {
    Pria,
    Wanita,
}

impl Gender {
    fn client(&self, toilet: String) {
        println!("Selamat menikmati");
    }
}

#[test]
fn test_gender() {
    let sex = Gender::Pria;
    sex.client(String::from("Jhonny"));

}

enum Cuaca{
    Cerah,
    Hujan(u32),
    Mendung(String),
}
impl Cuaca{
    fn tampilkan(&self){
        /* Saat melakukan pattern maching pada enum yang memiliki data
        kita hanya perlu memasukan nama variable nya aja pada datanya
        karena nanti datanya kita bisa ubah saat pembuatan variable
        di kondisi yang akan dieksekusi */
        match self {
            Cuaca::Cerah => println!("Matahari bersinar terang"),
            Cuaca::Hujan(number) => println!("Banyak air jatuh"),
            Cuaca::Mendung(kondisi) => println!("Matahari Gakeliatan"),
        }
    }
}

#[test]
fn kondisi_cuaca() {
    let result = Cuaca::Mendung(String::from("Berkabut"));
    result.tampilkan();
    let result1 = Cuaca::Cerah;
    result1.tampilkan();
    let result2 = Cuaca::Hujan(17);
    result2.tampilkan()
}

#[test]
fn pattern_fundamental() {

    let angka = 3;
    /*Dalam pattern matching kita bisa menggunakan beberapa 
    cara  */
    let hasil = match angka {
        1 => "Satu", // cara paling dasar
        2 | 3 => "Dua atau Tiga", // menggunakan usecase or 
        4..=6 => "Prima", // menggunakan range tapi hanya range ini saja
        _ => "Invalid" // strip bawah sebagai default atau else 
    };

    println!("{}", hasil);
}

/*Kita bisa destructuring data di tuple dan struct */

struct Data{
    id: u32,
    address: String,
    date: String,
}

#[test]
fn pattern_struct() {
    let hasil = Data{
        id: 3241,
        address : String::from("Kuningan"),
        date : String::from("Juli"),
    };
    
    match hasil {
        /*Kita bisa mengambil beberapa field di dalam struct menggunakan
        pattern matching */
        Data{id,..} if id > 1000 => println!("Kita mendapatkannya"),
        Data{..} => println!("Kemana dia?"),
    }


}

#[test]
fn rust_guard_pattern() {
    let angka = Some(8);

    match angka {
        Some(x) if x < 10 => println!("tailo"),
        Some(x) if x > 10 => println!("Bravo"),

        None => println!("Nothing"),
        Some(x) => println!("Kanjut? {}",x),
    }
}