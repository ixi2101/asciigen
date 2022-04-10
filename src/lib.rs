pub mod asciigen {

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
        let fval = (val as f32 / 255.0) as f32;
        let xval = fval * (luminosity.len() as f32 - 1.0);

        
        luminosity.chars().nth(xval as usize).unwrap()
    }

    fn resize(args: Args, img: &GrayImage) -> Result<GrayImage, Box<dyn std::error::Error>> {
        let scale_factor: u32;
        match args.height {
            None => match args.width {
                None => scale_factor = img.width() / 80,
                Some(w) => scale_factor = img.width() / w,
            },
            Some(h) => scale_factor = img.height() / h,
        }
        let nwidth = img.width() / scale_factor;
        let nheight = img.height() / scale_factor;
        eprintln!("printing to {}w x {}h", nwidth, nheight);
        let resized: GrayImage =
            image::imageops::resize(img, nwidth, nheight, image::imageops::FilterType::Nearest);
        let resized_ctr = image::imageops::contrast(&resized, 100.0);
        resized_ctr.save("debug.png")?;
        Ok(resized_ctr)
    }

    pub fn convert(args: Args) -> Result<(), Box<dyn std::error::Error>> {
        let img = ImageReader::open(&args.path)?.decode()?;
        let luma: GrayImage = DynamicImage::into_luma8(img);
        let luma_resized = resize(args, &luma)?;
        for (_n, i) in luma_resized.enumerate_rows() {
            // Print to console.
            i.into_iter().for_each(|x| print!("{} ", lookup(x.2 .0[0])));
            println!();
        }
        Ok(())
    }
}
