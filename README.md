# Guessing-Number-Games
**=======================================**

1. ## Apaan emang?
  - Program ini saya buat sebagai latihan dalam belajar
  bahasa pemrograman **[Rust](https://www.rust-lang.org/)**

2. ## Gimana cara pakenya???

  - Kamu bisa mendownload nya dulu melalui git
  dengan cara : 
  ```bash
  git clone https://github.com/Yuuki1578/Guessing-Number-Games.git
  ```

  
  - Untuk cara menjalankannya, kamu harus punya 
  **[Rust](https://www.rust-lang.org/)** Terinstall
  di Terminal, Karena saya menggunakan *[Termux](https://termux.dev/en/)* Saya mendownloadnya
  dengan cara
  ```bash
  pkg install rust -y
  ```


  - Jika sudah terinstall, masuk ke direktori
  *repository*, lalu ketikkan command: 
  ```bash
  cargo run
  ```


3. ## Gimana cara kerjanya???
  - Simple nya, core dari program ini adalah *crate*
  ***rand*** yang berisi fungsi untuk memghasilkan
  angka random yang nantinya akan ditebak oleh player

  - Di file *Cargo.toml*, saya menambahkan *dependencies*
  berupa ***rand*** dengan versi "0.8.5"
  untuk lebih jelas, lihat kode dibawah: 
  ```toml
  [dependencies]
  rand = "0.8.5"
  ```

  - Lalu di file *main.rs*, saya menambahkan crate
  tersebut sebelum menulis *main function*:
  ```rust
  use rand::Rng;
  ```

4. ## Kesimpulan
  - Udah segitu aja sih, susah ngedit markdown sampe 
  panjang lebar kayak repository-repository
  terkenal 😓😓😓

**=======================================**
# TERIMA KASIH
