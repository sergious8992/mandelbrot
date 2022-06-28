use num::Complex;
use std::{fs::File, io::Write};

const WIDTH: usize = 2_000;
const HEIGHT: usize = 2_000;

type Pixels = Vec<u8>;
const F_HEIGHT: f64 = HEIGHT as f64;
const  F_WIDTH: f64 = WIDTH as f64;


fn write_image(pixels: &mut Pixels, path: &str){
    let header: &str = &format!("P3\n{} {}\n{}\n", WIDTH, HEIGHT, 255);
    let mut image = File::create(path).unwrap();
    let mut aux: String = "".to_owned();
    image.write_all(header.as_bytes()).unwrap();

    for p in 0..WIDTH*HEIGHT{
        let c = pixels[p];
        aux = aux + &format!("{} {} {}\n", c, c, c);
        if p % 1000 == 0 {
            image.write_all(aux.as_bytes()).unwrap();
            aux = "".to_owned();
            println!("{:.5}%", ((p as f64/ (F_HEIGHT*F_HEIGHT)) * 100.0));
        }
    }
    
}

fn write_pixel(x: usize, y: usize, color: u8, pixels: &mut Pixels){

    let index;

    if y>HEIGHT || x>WIDTH {
        panic!("Write pixel out of range");
    }
    index = (y*WIDTH) + x;
    (*pixels)[index] = color;
}


fn clear_image(pixels: &mut Pixels){
    if pixels.len() == 0{
        for _ in 0..WIDTH*HEIGHT{
            pixels.push(255);
        }
    } else {
        for pixel in (*pixels).iter_mut(){
            *pixel = 255 as u8;
        }
    }
}


fn mandelbrot(pixels: &mut Pixels){    
    for y in 0..HEIGHT{
        
        let im: f64 = (y as f64  - (F_HEIGHT/2.0)) / F_HEIGHT * 2.5;
        
        for x in 0..WIDTH{
            let re = ((x as f64 - (F_WIDTH/2.0)) / F_WIDTH*2.5) - 0.5;
            let mut z = re + im * Complex::i();
            let c = z;

            for i in 0..255u8{
                z = (z*z) + c;
                if z.norm() > 2.5 {
                    write_pixel(x, y, i, pixels);
                }
            }
        }
        println!("{:.5}%", ((y as f64/ F_WIDTH) * 100.0));
    }
}

fn main() {
    let file_name = "MaldelbrotR.ppm".to_owned();
    let mut pixels: Vec<u8> = Vec::with_capacity(WIDTH*HEIGHT);
    //let mut pixels = Box::new([0 as u8; WIDTH*HEIGHT]);

    clear_image(&mut pixels);
    mandelbrot(&mut pixels);
    write_image(&mut pixels, &file_name);

    println!("Hello, world!");
}
