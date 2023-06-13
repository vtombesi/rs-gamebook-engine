pub fn log_damage<T: std::fmt::Display>(message: T) {
    println!("\x1b[31m{}\x1b[0m", message); // Stampa in rosso
}

pub fn log_recovery<T: std::fmt::Display>(message: T) {
    println!("\x1b[32m{}\x1b[0m", message); // Stampa in verde
}

pub fn log_monster_name<T: std::fmt::Display>(message: T) {
    println!("\x1b[41;1;37m{}\x1b[0m", message); // Sfondo rosso, scritta bianca
}

pub fn log_location<T: std::fmt::Display>(message: T) {
    println!("\x1b[34m{}\x1b[0m", message); // Stampa in blu
}

pub fn log_narration<T: std::fmt::Display>(message: T) {
    println!("\x1b[90m{}\x1b[0m", message); // Stampa in grigio
}

pub fn log_choice<T: std::fmt::Display>(message: T) {
    println!("\x1b[37m{}\x1b[0m", message); // Stampa in grigio
}

pub fn log_loot_item<T: std::fmt::Display>(message: T) {
    println!("\x1b[32m+ {}\x1b[0m", message); // Stampa in verde con prefisso "+ "
}
