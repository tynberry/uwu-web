use macroquad::prelude::*;

const HOLY_TEXTS: [&str; 2] = [
    "UwU",
    "You got UwUwified!",
];

struct MiniUwU {
    pos: Vec2,
    vel: Vec2
}

#[macroquad::main("~~ UwU ~~")]
async fn main() {
    //načti epický text
    let font = load_ttf_font("font.ttf").await.unwrap();

    //vytvoř miniuwuvátka
    let mut miniuwus = Vec::with_capacity(100);
    for _ in 0..100 {
        let pos = vec2(
            fastrand::f32()*500.0 - 250.0,
            fastrand::f32()*500.0 - 250.0,
        );
        let vel = vec2(
            fastrand::f32()*500.0 - 250.0,
            fastrand::f32()*500.0 - 250.0,
        );
        miniuwus.push(
            MiniUwU { pos, vel }
        );
    }
    
    loop {
        clear_background(BLACK);
        
        //get canvas size
        let canvas_sx = screen_width();
        let canvas_sy = screen_height();

        //text velikost
        let dimensions = [
            measure_text(HOLY_TEXTS[0], Some(font), 64, 1.0),
            measure_text(HOLY_TEXTS[1], Some(font), 32, 1.0),
        ];

        //vykresli miniuwu
        let dt = get_frame_time();
        for muwu in &mut miniuwus {
            //posun
            muwu.pos += muwu.vel * dt;
            //pružinová síla
            muwu.vel += -muwu.pos * 0.3 * dt;
            //vykresli
            draw_text_ex(HOLY_TEXTS[0],
                canvas_sx / 2.0 + muwu.pos.x, 
                canvas_sy / 2.0 + muwu.pos.y, TextParams { 
                    font, 
                    font_size: 16, 
                    color: color_u8!(0, 255, 255, 64), 
                    ..Default::default()
                }
            );

        }

        //vykresli hlavní text
        let celkova_vyska = dimensions[0].height + dimensions[1].height;

        draw_text_ex(HOLY_TEXTS[0], 
            canvas_sx / 2.0 - dimensions[0].width / 2.0, 
            canvas_sy / 2.0 - celkova_vyska / 2.0, TextParams { 
                font, 
                font_size: 64, 
                color: color_u8!(0, 255, 255, 255), 
                ..Default::default()
            }
        );

        draw_text_ex(HOLY_TEXTS[1], 
            canvas_sx / 2.0 - dimensions[1].width / 2.0, 
            canvas_sy / 2.0 - celkova_vyska / 2.0 + dimensions[0].height, TextParams { 
                font, 
                font_size: 32, 
                color: color_u8!(0, 255, 255, 255), 
                ..Default::default()
            }
        );

        next_frame().await;
    }
}
