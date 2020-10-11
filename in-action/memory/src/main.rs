use std::io::Read;

fn pointers() {
    // We can create raw pointers by coercing references
    let a = 42;
    let a_ptr = &a as *const i32;
    // It is unsafe to access a pointer value
    // But we need it if we want to print the address of a's last byte
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 3);
}

fn stack_and_heap() {
    // This lives on the stack
    let a = 40;

    // This lives on the heap, which means that the integer 50 is allocated on the heap
    // But the pointer to it lives on the stack.
    let b = Box::new(50);

    // The latter, b, is accessed through a pointer
    let sum = a + *b;

    println!("{} + {} = {}", a, b, sum);
}

#[derive(Debug)]
struct Process {
    pid: nix::unistd::Pid,
    name: String,
}

fn inspection() {
    // Can get this process ID
    let pid = nix::unistd::getpid();
    let mut process = Process {
        pid,
        name: String::from(""),
    };

    // Read /proc/<pid>/status to retrieve information about this process
    let mut buffer = String::new();
    std::fs::File::open(format!("/proc/{}/status", pid))
        .expect("Failed to open process status")
        .read_to_string(&mut buffer)
        .expect("Failed to read process status");

    // Look for line Name: <process-name>
    for line in buffer.lines() {
        let elements: Vec<&str> = line.split_whitespace().collect();
        if elements[0] == "Name:" {
            process.name = String::from(elements[1]);
        }
    }

    println!("{:?}", process);
}

fn main() {
    pointers();
    stack_and_heap();
    inspection();
}
