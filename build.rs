fn main() {
    println!("cargo:rerun-if-changed=src/query_engine/get_metrics.graphql");
}

