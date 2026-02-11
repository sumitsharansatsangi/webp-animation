use log::info;
use webp_animation::Decoder;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let buffer = std::fs::read("./data/animated.webp").unwrap();
    let decoder = Decoder::new(&buffer).unwrap();

    for frame in decoder.into_iter() {
        #[cfg(feature = "image")]
        let (dimensions, data_len) = {
            let dims = frame.dimensions();
            let len = frame.data().len();
            let image = frame.into_image().unwrap();
            assert_eq!(image.dimensions(), (400, 400));
            (dims, len)
        };

        #[cfg(not(feature = "image"))]
        let (dimensions, data_len) = { (frame.dimensions(), frame.data().len()) };

        info!("Frame, dimensions={:?}, data_len={}", dimensions, data_len);
    }
}
