
use neutrondb::Store;
use rand::Rng;

fn main() {

    println!("NeutronDB Bench Started ...");

    let mut test_store: Store<u64, String> = Store::new("./test").unwrap();

    println!("Test Store Created ...");

    let items: u64 = 100_000;

    let first_key = 0;
    let last_key = items - 1;

    for i in 0..items {
        let s = format!("lorem ipsum #{}", i);
        test_store.put(&i, &s).unwrap();
    }

    let first_value = test_store.get(&first_key);
    println!("R_{} -> {:?}", first_key, first_value);

    let last_value = test_store.get(&last_key);
    println!("R_{} -> {:?}", last_key, last_value);

    let mut rng = rand::thread_rng();
    let r: u64 = rng.gen_range(0u64..items);
    let r_value = test_store.get(&r);
    println!("R_{} -> {:?}", r, r_value);

}
