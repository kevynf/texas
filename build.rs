#[cfg(windows)]
fn main() {
    use std::{env, fs, path::PathBuf};

    println!("cargo:rerun-if-changed=icons/texas/texas.svg");

    let svg = include_bytes!("icons/texas/texas.svg");
    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(svg, &options)
        .expect("failed to parse the Texas application icon");

    let mut frames = Vec::new();
    for size in [16, 32, 48, 256] {
        let mut pixmap = tiny_skia::Pixmap::new(size, size)
            .expect("failed to allocate the Texas application icon");
        let scale = size as f32 / tree.size().width();
        resvg::render(
            &tree,
            tiny_skia::Transform::from_scale(scale, scale),
            &mut pixmap.as_mut(),
        );

        frames.push(
            image::codecs::ico::IcoFrame::as_png(
                pixmap.data(),
                size,
                size,
                image::ExtendedColorType::Rgba8,
            )
            .expect("failed to encode the Texas application icon"),
        );
    }

    let icon_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("texas.ico");
    let mut icon =
        fs::File::create(&icon_path).expect("failed to create the Texas icon");
    image::codecs::ico::IcoEncoder::new(&mut icon)
        .encode_images(&frames)
        .expect("failed to write the Texas application icon");

    let mut resource = winres::WindowsResource::new();
    resource.set_icon(icon_path.to_str().expect("invalid icon path"));
    resource
        .compile()
        .expect("failed to embed the Texas application icon");
}

#[cfg(not(windows))]
fn main() {}
