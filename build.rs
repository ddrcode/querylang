fn main() {
    println!("cargo:rerun-if-changed=src/repository/schema.graphql");
    println!("cargo:rerun-if-changed=src/repository/get_metrics.graphql");
}

