fn main() {
    // Creates a CpuPool with workers equal to the cores on the machine.
    let pool = Builder::new().create();

    let returns_string = pool.spawn_fn(move || -> Result<_, ()> {
        Ok("First")
    });
    let returns_integer = pool.spawn_fn(move || -> Result<_, ()> {
        Ok(2)
    });

    let resulting_string = returns_string.wait().unwrap();
    let resulting_integer = returns_integer.wait().unwrap();
    
    println!("{}, {}", resulting_string, resulting_integer);
    // Returns `First, 2`
}

extern crate futures;
extern crate futures_cpupool;
use futures::future::Future;
use futures_cpupool::Builder;