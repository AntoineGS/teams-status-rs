use winres::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();
    res.set_icon("microsoft-teams.ico");
    res.compile().unwrap();
}
