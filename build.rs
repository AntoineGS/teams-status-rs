use winresource::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();
    res.set_icon_with_id("microsoft-teams.ico", "default-icon");
    res.compile().unwrap();
}
