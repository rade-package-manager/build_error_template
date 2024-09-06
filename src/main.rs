#[cfg(target_os = "windows")]
fn hi() {
    println!("Hello! How did you get here?");
    println!("Oh yeah, if you're running comrade on windows, it would be nice to know if you're getting any errors or anything.\nI can see you're a windows user.");
}
#[cfg(target_os = "macos")]
fn hi() {
    println!("Hello! How did you get here?");
    println!("Have you had any problems running comrade on a mac?");
    println!("If you're getting errors or anything like that, I'd appreciate it if you'd join the discord and report it to me.");
}
#[cfg(target_os = "linux")]
fn hi() {
    println!("Hello! How did you get here?");
    println!("You are a linux user, aren't you!");
    println!("Linux is so cool and developer friendly.");
    println!("I'm an Arch Linux user myself!");
}

fn main() {
    compile_error!("Intended compilation error");
    hi();
}
