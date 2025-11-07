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
const MINIMUM :i32 = 20;

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

