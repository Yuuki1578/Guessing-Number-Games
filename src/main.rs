use std::io::stdin;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let angka_target = rand::thread_rng().gen_range(1..=100);
    println!("Game menebak angka!\nTebaklah angka dari 1 - 100\n");
    println!("\t-Jika player benar, program akan berhenti");
    println!("\t-Jika player salah, program akan terus berjalan\n");

    loop {
        let mut angka_input = String::new();
        println!("Tebakan anda : ");

        stdin().read_line(&mut angka_input).expect("Gagal mengakses io::stdin");

        let angka_input: u32 = match angka_input.trim().parse() {
            Ok(nums) => nums,
            Err(_) => {
                println!("Method trim() dan parse() gagal");
                continue;
            }
        };

        match angka_input.cmp(&angka_target) {
            Ordering::Greater => println!("Angka terlalu besar!\n"),
            Ordering::Less => println!("Angka terlalu kecil!\n"),
            Ordering::Equal => {
                println!("Angka cocok! {angka_target} == {angka_input}\n");
                break;
            }
        }
    }
}
