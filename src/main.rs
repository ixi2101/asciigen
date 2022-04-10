use image::{io::Reader as ImageReader, GrayImage, DynamicImage};


fn lookup(val: u8) -> char{
    let luminosity = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft()1{}[]?-_+~<>i!lI;:,";
    //println!("LEN{}", luminosity.len());
    let bval = (val/255) * (luminosity.len() as u8 - 1);
    //println!("{}", bval);
    luminosity.chars().nth((bval).into()).unwrap()
    
}
fn main() {
    println!("Hello, world!");
    let filename = String::from("img.png");
  
    let img = ImageReader::open(&filename).unwrap().decode().unwrap();
    let mut luma: GrayImage = DynamicImage::into_luma8(img);
    luma = image::imageops::resize(&luma, 40, 20, image::imageops::FilterType::Nearest);
    for (n,i) in luma.enumerate_rows(){
        
        i.into_iter().for_each(|x| print!("{} ", lookup(x.2.0[0])));
        println!("");
    }
    //println!("{:#?}", luma[(10,10)]);
    luma.save("filename.png").unwrap();
}
