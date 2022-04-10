pub mod asciigen {
    use image::{io::Reader as ImageReader, DynamicImage, GrayImage};

    fn lookup(val: u8) -> char {
        let luminosity = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft()1{}[]?-_+~<>i!lI;:,";
        let bval = (val / 255) * (luminosity.len() as u8 - 1);
        luminosity.chars().nth((bval).into()).unwrap()
    }

    pub fn convert(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let img = ImageReader::open(&filepath)?.decode()?;
        let mut luma: GrayImage = DynamicImage::into_luma8(img);
        luma = image::imageops::resize(&luma, 40, 20, image::imageops::FilterType::Nearest);
        for (_n, i) in luma.enumerate_rows() {
            // Print to console.
            i.into_iter().for_each(|x| print!("{} ", lookup(x.2 .0[0])));
            println!("");
        }
        Ok(())
        //eprintln!("Printing complete");
    }
}
