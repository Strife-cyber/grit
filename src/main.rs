
mod algorithms;
mod systems;
mod structure;

fn main() {
    let old_version = "Line 1\nHello World\nLine 3";
    let new_version = "Line 1\nHello Git\nLine 3\nLine 4";

    let changes = algorithms::vcompare::compv::compare(old_version, new_version);

    for change in changes {
        println!("{:?}", change);
    }
}
