use clap::Parser;
use sysinfo::{Disks, Networks, System};

#[derive(Parser)]
#[command(name = "sysinfo")]
#[command(about = "Zeigt Systeminformationen an", long_about = None)]
struct Args {
    /// Zeigt CPU-Informationen
    #[arg(short, long)]
    cpu: bool,

    /// Zeigt Speicher-Informationen
    #[arg(short, long)]
    memory: bool,

    /// Zeigt Festplatten-Informationen
    #[arg(short, long)]
    disks: bool,

    /// Zeigt Netzwerk-Informationen
    #[arg(short, long)]
    network: bool,

    /// Zeigt alle Informationen
    #[arg(short, long)]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Wenn keine Option gewählt, zeige Hilfe
    if !args.cpu && !args.memory && !args.disks && !args.network && !args.all {
        println!("Bitte wählen Sie mindestens eine Option (--help für Hilfe)");
        return Ok(());
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    if args.all || args.cpu {
        show_cpu_info(&sys)?;
    }

    if args.all || args.memory {
        show_memory_info(&sys)?;
    }

    if args.all || args.disks {
        show_disk_info()?;
    }

    if args.all || args.network {
        show_network_info()?;
    }

    Ok(())
}

fn show_cpu_info(sys: &System) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== CPU Informationen ===");
    println!("Anzahl CPUs: {}", sys.cpus().len());
    println!(
        "Systemname: {}",
        System::name().unwrap_or_else(|| "Unbekannt".to_string())
    );
    println!(
        "Kernel Version: {}",
        System::kernel_version().unwrap_or_else(|| "Unbekannt".to_string())
    );
    println!(
        "OS Version: {}",
        System::os_version().unwrap_or_else(|| "Unbekannt".to_string())
    );
    println!(
        "Host Name: {}",
        System::host_name().unwrap_or_else(|| "Unbekannt".to_string())
    );
    Ok(())
}

fn show_memory_info(sys: &System) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Speicher Informationen ===");
    println!(
        "Gesamter RAM: {} GB",
        sys.total_memory() / 1024 / 1024 / 1024
    );
    println!(
        "Genutzter RAM: {} GB",
        sys.used_memory() / 1024 / 1024 / 1024
    );
    println!(
        "Freier RAM: {} GB",
        sys.available_memory() / 1024 / 1024 / 1024
    );
    println!(
        "Gesamter Swap: {} GB",
        sys.total_swap() / 1024 / 1024 / 1024
    );
    println!(
        "Genutzter Swap: {} GB",
        sys.used_swap() / 1024 / 1024 / 1024
    );
    Ok(())
}

fn show_disk_info() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Festplatten Informationen ===");
    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        println!("\nFestplatte: {}", disk.name().to_string_lossy());
        println!("  Mountpoint: {}", disk.mount_point().display());
        println!("  Dateisystem: {:?}", disk.file_system());
        println!("  Gesamt: {} GB", disk.total_space() / 1024 / 1024 / 1024);
        println!(
            "  Verfügbar: {} GB",
            disk.available_space() / 1024 / 1024 / 1024
        );
    }
    Ok(())
}

fn show_network_info() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== Netzwerk Informationen ===");
    let networks = Networks::new_with_refreshed_list();

    for (interface_name, network) in &networks {
        println!("\nInterface: {}", interface_name);
        println!("  Empfangen: {} KB", network.total_received() / 1024);
        println!("  Gesendet: {} KB", network.total_transmitted() / 1024);
    }
    Ok(())
}
