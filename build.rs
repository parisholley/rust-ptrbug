fn main() {
    cc::Build::new()
        .file("lib.c")
        .compile("ptrbug");
}