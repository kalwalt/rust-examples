use imageproc::utils::gray_bench_image;
use imageproc::filter::gaussian_blur_f32;
use imageproc::noise::salt_and_pepper_noise;


fn main() {

    let image = gray_bench_image(100, 100);
    let noised_image = salt_and_pepper_noise(&image, 0.3, 14);

    let sigma = 0.8;

    let filtered_image = gaussian_blur_f32(&noised_image, sigma);

    // Write the contents of this image to the Writer in PNG format.
    filtered_image.save("filtered_image.png").unwrap();
}
