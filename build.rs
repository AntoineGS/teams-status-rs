use windres::Build;

fn main() {
    Build::new().compile(".\\teams_status.rc").unwrap();
}
