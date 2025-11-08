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

#[test]
fn the_name() {
    let first_name = String::from("Naufal");
    let last_name = String::from("Abdul");
    let full_name = nama_variable(first_name, last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);

}