pub fn boot_os() {
    println!("Volition OS is booting!");

    // TODO: Add the real initialization here
    init_memory();
    init_interrupts();
    init_drivers();
}

fn init_mamory() {}
fn init_interrupts() {}
fn init_drivers() {}