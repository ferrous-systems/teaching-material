mod workload;

fn main() {
    workload::work();
    
    workload::thing::do_stuff();
}