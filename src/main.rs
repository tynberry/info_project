use macroquad::prelude::*;

#[macroquad::main("Warping Warp")]
async fn main() {
    //init world 
    let mut world = hecs::World::default();

    loop {
        //UPDATE WORLD 


        //RENDERING PHASE
        clear_background(BLACK);

        next_frame().await;
    }
}
