
const RES_X: usize = 64;
const RES_Y: usize = 32;
use std::fmt;

pub struct Gfx{
    back_buffer: [[bool; RES_X]; RES_Y],
}

impl Gfx{
    pub fn new() -> Gfx{
        Gfx{
            back_buffer: [[false; RES_X]; RES_Y]
        }
    }
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool{ // return strue when a collision is detected
        let mut collision = false;

        for (sprite_index, tmp_y) in (y .. y + sprite.len()).enumerate(){

            let pix1 = sprite[sprite_index] & 0x1;
            if self.draw_pixel(x, tmp_y, pix1){
                collision = true
            }

            let pix2 = sprite[sprite_index] & 0x2;
            if self.draw_pixel(x+1, tmp_y, pix2){
                collision = true
            }

            let pix3= sprite[sprite_index] & 0x4;
            if self.draw_pixel(x+2, tmp_y, pix3){
                collision = true
            }

            let pix4 = sprite[sprite_index] & 0x8;
            if self.draw_pixel(x+3, tmp_y, pix4){
                collision = true
            }

            let pix5 = sprite[sprite_index] & 0x10;
            if self. draw_pixel(x+4, tmp_y, pix5){
                collision = true
            }

            let pix6 = sprite[sprite_index] & 0x20;
            if self.draw_pixel(x+5, tmp_y, pix6){
                collision = true
            }

            let pix7 = sprite[sprite_index] & 0x40;
            if self.draw_pixel(x+6, tmp_y, pix7){
                collision = true
            }

            let pix8 = sprite[sprite_index] & 0x80;
            if self.draw_pixel(x+7, tmp_y, pix8){
                collision = true
            }
        }
        collision
    }

    fn draw_pixel(&mut self, x: usize, y: usize, pixel: u8) -> bool{
         if pixel == 1 && self.back_buffer[x][y] == true{
                self.back_buffer[x][y] = false;
                return true
            } else if pixel == 1 || self.back_buffer[x][y] == true{
                self.back_buffer[x][y] = true
            }
            false
    }
}
    impl fmt::Debug for Gfx {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for row in self.back_buffer.iter(){
                for entry in row.iter(){
                    if *entry{
                        print!("X");
                    } else{
                        print!(" ");
                    }
                }
                print!("\n");
            }
            Ok(())
        }
    }

