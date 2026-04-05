use log::info;
use webp_animation::Decoder;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let buffer = std::fs::read("./data/animated.webp").unwrap();
    let decoder = Decoder::new(&buffer).unwrap();

    for frame in decoder.into_iter() {
        let dimensions = frame.dimensions();
        let data_len = frame.data().len();

        assert_eq!(dimensions, (400, 400));
        assert_eq!(data_len, 400 * 400 * 4); // w * h * rgba

        #[cfg(feature = "image")]
        assert_eq!(frame.into_image().unwrap().dimensions(), (400, 400));

        info!("Frame, dimensions={:?}, data_len={}", dimensions, data_len);
    }
}
