use windres::Build;

fn main() {
    Build::new().compile("teams-status.rc").unwrap();
}
