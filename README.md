# ⚡ ZW-CLI 

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Version](https://img.shields.io/badge/version-1.0.0-blue?style=for-the-badge)
![Maintained](https://img.shields.io/badge/Maintained%3F-yes-green.svg?style=for-the-badge)

```text
 _______       __      ________    ____
/__  / |     / /     / ____/ /   /  _/
  / /| | /| / /_____/ /   / /    / /  
 / /_| |/ |/ /_____/ /___/ /____/ /   
/____/|__/|__/      \____/_____/___/  

```

**ZW-CLI** adalah sebuah alat *Command Line Interface* (CLI) pantas yang ditulis sepenuhnya di dalam **Rust**. Pada masa ini, ia bertindak sebagai pengurus pakej berkuasa untuk persekitaran Linux/WSL (membungkus arahan `apt` dengan antaramuka berwarna), dan akan dikembangkan dengan pelbagai fungsi utiliti pembangun pada masa hadapan.

Dibangunkan oleh **[ZWDevs](https://www.google.com/search?q=https://github.com/Wishprogram01)**.

---

## ✨ Ciri-ciri Utama

* 🚀 **Sangat Pantas:** Dibina menggunakan Rust (dikompilasi ke *native binary*).
* 🎨 **Antaramuka Menarik:** Dilengkapi dengan ASCII art dan output berwarna menggunakan modul `colored`.
* 📦 **Pengurus Pakej Mudah:** Pasang dan buang pakej sistem (seperti `neofetch`, `htop`) dengan arahan yang ringkas.
* 🛠️ **Struktur Boleh Dikembangkan:** Menggunakan `clap` *crate* untuk pengurusan *subcommand* yang mudah diselenggara.

## 🚧 Roadmap (Akan Datang)

* [ ] Integrasi `pulldown-cmark` untuk menukar fail Markdown (`.md`) kepada HTML secara terus dari terminal.
* [ ] Automasi muat turun fail dan skrip terus dari GitHub.
* [ ] Pengurusan fail tempatan secara pukal.

---

## ⚙️ Pemasangan (Bina dari Sumber)

Oleh kerana ZW-CLI dibangunkan dengan Rust, pastikan anda telah memasang `cargo` dan `rustc` di dalam sistem anda.

1. **Klon repositori ini:**
```bash
git clone [https://github.com/Wishprogram01/zw-cli-v2.git](https://github.com/Wishprogram01/zw-cli-v2.git)
cd zw-cli-v2

```


2. **Kompilasi kod ke mod Release:**
```bash
cargo build --release

```


3. **Pasang secara global (Linux/WSL):**
```bash
sudo mv target/release/zwcli /usr/local/bin/zw-cli

```



---

## 💻 Cara Penggunaan

Gunakan `zw-cli` dari mana-mana lokasi di dalam terminal anda.

**Papar Senarai Arahan:**

```bash
zw-cli list

```

**Pasang Pakej (Contoh: htop):**

```bash
zw-cli install htop

```

**Buang Pakej (Contoh: sl):**

```bash
zw-cli remove sl

```

**Papar Menu Bantuan Penuh:**

```bash
zw-cli --help

```

---

Untuk memuat naik fail ini ke GitHub, jalankan arahan berikut di terminal WSL anda:

```bash
touch README.md
nano README.md
# Paste kod di atas, kemudian tekan Ctrl+O, Enter, dan Ctrl+X untuk simpan
git add README.md
git commit -m "docs: add awesome README file"
git push -u origin main

*Dicipta dengan 💻 & ☕ oleh Darwisy Amsyar (ZWDevs).*

```

```
