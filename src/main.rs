extern crate taskworks;

use taskworks::id::cpuinfo::Cpuinfo;

fn main() {
    Cpuinfo::read_cpuinfo();
}
