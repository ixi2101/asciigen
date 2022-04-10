pub mod asciigen {

    use std::ptr::NonNull;

    use clap::{ArgGroup, Parser};
    use image::{io::Reader as ImageReader, DynamicImage, GrayImage};

    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    #[clap(group(
        ArgGroup::new("resize")
        .required(false)
        .args(&["width", "height"])
    ))]
    pub struct Args {
        // Filepath
        path: String,

        /// Scale image width
        #[clap(long)]
        width: Option<u32>,
        /// Scale image height
        #[clap(long)]
        height: Option<u32>,
    }

    fn lookup(val: u8) -> char {
        let luminosity = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft()1{}[]?-_+~<>i!lI;:,";
        let bval = (val / 255) * (luminosity.len() as u8 - 1);
        luminosity.chars().nth((bval).into()).unwrap()
    }

    fn resize(args: Args, img: &GrayImage) -> Result<GrayImage, Box<dyn std::error::Error>> {
        // Scale to 80W
        let scale_factor: u32;
        match args.height {
            None => match args.width {
                None => scale_factor = img.width() / 80,
                Some(w) => scale_factor = img.width() / w,
            },
            Some(h) => scale_factor = img.height() / h,
        }
        let nwidth = img.width() * 1 / scale_factor;
        let nheight = img.height() * 1 / scale_factor;
        eprintln!("printing to {} x {}", nwidth, nheight);
        let resized: GrayImage =
            image::imageops::resize(img, nwidth, nheight, image::imageops::FilterType::Nearest);

        Ok(resized)
    }

    pub fn convert(args: Args) -> Result<(), Box<dyn std::error::Error>> {
        let img = ImageReader::open(&args.path)?.decode()?;
        let luma: GrayImage = DynamicImage::into_luma8(img);
        // luma = image::imageops::resize(&luma, 40, 20, image::imageops::FilterType::Nearest);
        let luma_resized = resize(args, &luma)?;
        for (_n, i) in luma_resized.enumerate_rows() {
            // Print to console.
            i.into_iter().for_each(|x| print!("{} ", lookup(x.2 .0[0])));
            println!("");
        }
        Ok(())
        //eprintln!("Printing complete");
    }
}
