use clap::{Parser, Subcommand};
use colored::*;
use std::process::Command;

const ASCII_ART: &str = r#"
 _______       __      ________    ____
/__  / |     / /     / ____/ /   /  _/
  / /| | /| / /_____/ /   / /    / /  
 / /_| |/ |/ /_____/ /___/ /____/ /   
/____/|__/|__/      \____/_____/___/  
"#;

#[derive(Parser)]
#[command(name = "zw-cli")]
#[command(author = "ZWDevs")]
#[command(version = "1.0")]
#[command(about = "Pengurus Pakej ZW-CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pasang pakej baru ke dalam sistem
    Install {
        /// Nama pakej yang ingin dipasang (contoh: htop, neofetch)
        package_name: String,
    },
    /// Buang pakej dari sistem
    Remove {
        /// Nama pakej yang ingin dibuang
        package_name: String,
    },
    /// Senaraikan semua arahan yang ada berserta contoh
    List,
}

fn main() {
    // Cetak ASCII art dengan warna cyan yang terang dan bold
    println!("{}", ASCII_ART.cyan().bold());

    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { package_name } => {
            println!("{}", format!("[+] Memulakan pemasangan untuk pakej: {}", package_name).green());
            
            // Memanggil command apt install di dalam Linux
            let status = Command::new("sudo")
                .arg("apt")
                .arg("install")
                .arg("-y")
                .arg(package_name)
                .status()
                .expect("Gagal menjalankan arahan apt");

            if status.success() {
                println!("{}", format!("[-] Pakej {} berjaya dipasang!", package_name).blue().bold());
            } else {
                println!("{}", format!("[!] Gagal memasang pakej {}.", package_name).red().bold());
            }
        }
        Commands::Remove { package_name } => {
            println!("{}", format!("[+] Membuang pakej: {}", package_name).yellow());
            
            // Memanggil command apt remove di dalam Linux
            let status = Command::new("sudo")
                .arg("apt")
                .arg("remove")
                .arg("-y")
                .arg(package_name)
                .status()
                .expect("Gagal menjalankan arahan apt");

            if status.success() {
                println!("{}", "[-] Pakej berjaya dibuang.".green());
            } else {
                println!("{}", format!("[!] Gagal membuang pakej {}.", package_name).red().bold());
            }
        }
        Commands::List => {
            println!("{}", "Fungsi yang tersedia:".cyan().bold());
            
            println!("  {:<10} {}", "install".green(), "- Pasang pakej baru ke dalam sistem");
            println!("  {:<10} {}", "remove".red(), "- Buang pakej dari sistem");
            println!("  {:<10} {}", "list".blue(), "- Senaraikan semua arahan");

            println!("\n{}", "Contoh Penggunaan:".yellow().bold());
            println!("  $ zw-cli install neofetch  {}", "(Memasang pakej neofetch)".bright_black());
            println!("  $ zw-cli remove sl         {}", "(Membuang pakej sl)".bright_black());
            println!("  $ zw-cli --help            {}", "(Melihat menu bantuan lengkap clap)".bright_black());
        }
    }
}
