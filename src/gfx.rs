
const RES_X: usize = 64;
const RES_Y: usize = 32;
use std::fmt;
use std::sync::{Arc, RwLock};
pub struct Gfx{
    back_buffer: Arc<RwLock<[[bool; RES_X]; RES_Y]>>,
}

impl Gfx{
    pub fn new() -> Gfx{
        Gfx{
            back_buffer: Arc::new(RwLock::new([[false; RES_X]; RES_Y])),
        }
    }
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool{ // return strue when a collision is detected
        let mut the_buffer = self.back_buffer.write().unwrap();
        
        let mut collision = false;
        for (sprite_index, tmp_y) in (y .. y + sprite.len()  ).enumerate(){

            let fitted_y = tmp_y % RES_Y;
            let pix1 = sprite[sprite_index] & 0x1;
            if Gfx::draw_pixel(x % RES_X, fitted_y, pix1, &mut *the_buffer){
                collision = true
            }

            let pix2 = (sprite[sprite_index] >> 1) & 0x1;
            if Gfx::draw_pixel((x+1) % RES_X, fitted_y, pix2, &mut *the_buffer){
                collision = true
            }

            let pix3= (sprite[sprite_index] >> 2) & 0x1;
            if Gfx::draw_pixel( (x+2) % RES_X, fitted_y, pix3, &mut *the_buffer){
                collision = true
            }

            let pix4 = (sprite[sprite_index] >> 3) & 0x1;
            if Gfx::draw_pixel((x+3) % RES_X, fitted_y, pix4, &mut *the_buffer){
                collision = true
            }

            let pix5 = (sprite[sprite_index] >> 4) & 0x1;
            if Gfx::draw_pixel((x+4) % RES_X, fitted_y, pix5, &mut *the_buffer){
                collision = true
            }

            let pix6 = (sprite[sprite_index] >> 5) & 0x1;
            if Gfx::draw_pixel((x+5) % RES_X, fitted_y, pix6, &mut *the_buffer){
                collision = true
            }

            let pix7 = (sprite[sprite_index] >> 6) & 0x1;
            if Gfx::draw_pixel((x+6) % RES_X, fitted_y, pix7, &mut *the_buffer){
                collision = true
            }

            let pix8 = (sprite[sprite_index] >> 7) & 0x1;
            if Gfx::draw_pixel((x+7) % RES_X, fitted_y, pix8, &mut *the_buffer){
                collision = true
            }
        }
        collision
    }

    fn draw_pixel( x: usize, y: usize, pixel: u8, buffer: &mut [[bool; RES_X];RES_Y]) -> bool{
        if pixel == 1 && buffer[y][x] == true{
                buffer[y][x] = false;
                return true
            } else if pixel == 1 || buffer[y][x] == true{
                buffer[y][x] = true
            }
            false
    }

    pub fn get_frame(&mut self) -> [[bool;RES_X]; RES_Y]{
        *self.back_buffer.read().unwrap()
    }
}
    impl fmt::Debug for Gfx {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           let handle = self.back_buffer.read().unwrap();
            for (i, row) in handle.iter().enumerate(){
                print!("{:02}|",i);
                for entry in row.iter(){
                    if *entry{
                        print!("■");
                    } else{
                        print!(" ");
                    }
                }
                print!("|{:02}\n", i);
            }
            Ok(())
        }
    }

