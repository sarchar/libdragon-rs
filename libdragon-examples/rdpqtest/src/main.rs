#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use libdragon::*;

use libdragon::dfs::DfsPathBuf;
use libdragon::display::{Resolution, BitDepth, Gamma, FilterOptions};
use libdragon::sprite::Sprite;
use libdragon::surface::TexFormat;
use libdragon::timer::Timer;

use core_maths::*;
use rand_mt::Mt64;

struct Object {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    scale_factor: f32,
}

const NUM_OBJECTS: usize = 64;

struct App<'a> {
    brew_sprite: Sprite<'a>,
    _tiles_sprite: Sprite<'a>,
    tiles_block: rspq::Block,
    objects: Vec<Object>,
    num_objects: usize,
    cur_tick: u64,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let mut rng = Mt64::new(0);

        let display_width = display::get_width();
        let display_height = display::get_height();

        let brew_sprite = Sprite::load(DfsPathBuf::from("rom:/n64brew.sprite")).unwrap();
        let obj_max_x = display_width - brew_sprite.width() as u32;
        let obj_max_y = display_height - brew_sprite.height() as u32;

        let mut objects = Vec::new();
        for _ in 0..NUM_OBJECTS {
            objects.push(Object {
                x: (rng.next_u32() % obj_max_x) as i32,
                y: (rng.next_u32() % obj_max_y) as i32,
                dx: -3 + (rng.next_u32() % 7) as i32,
                dy: -3 + (rng.next_u32() % 7) as i32,
                scale_factor: 1.0,
            });
        }

        let tiles_sprite = Sprite::load(DfsPathBuf::from("rom:/tiles.sprite")).unwrap();

        // Create a block for the background, so that we can replay it later.
        rspq::Block::begin();

        // Check if the sprite was compiled with a paletted format. Normally
        // we should know this beforehand, but for this demo we pretend we don't
        // know. This also shows how rdpq can transparently work in both modes.
        let mut tlut = false;
        let format = tiles_sprite.get_format();
        if format == TexFormat::Ci4 || format == TexFormat::Ci8 {
            // If the sprite is paletted, turn on palette mode and load the
            // palette in TMEM. We use the mode stack for demonstration,
            // so that we show how a block can temporarily change the current
            // render mode, and then restore it at the end.
            rdpq::mode_push();
            rdpq::mode_tlut(rdpq::Tlut::Rgba16);
            rdpq::tex_upload_tlut(tiles_sprite.get_palette(), 0, 16);
            tlut = true;
        }

        let tile_width = tiles_sprite.width() / (tiles_sprite.hslices() as u16);
        let tile_height = tiles_sprite.height() / (tiles_sprite.vslices() as u16);

        // block off tiles_surf so that it's dropped before the move of tiles_sprite above
        {
            let tiles_surf = tiles_sprite.get_pixels();
            for ty in (0..display_height).step_by(tile_height as usize) {
                for tx in (0..display_width).step_by(tile_width as usize) {
                    let s = (rng.next_u32() % 2) * 32;
                    let t = (rng.next_u32() % 2) * 32;
                    rdpq::tex_upload_sub(rdpq::Tile(0), &tiles_surf, None, s as i32, t as i32, (s+32) as i32, (t+32) as i32);
                    rdpq::texture_rectangle(rdpq::Tile(0), tx as i32, ty as i32, (tx+32) as i32, (ty+32) as i32, s as i32, t as i32);
                }
            }
        }

        if tlut { rdpq::mode_pop(); }
        let tiles_block = rspq::Block::end();

        Self {
            brew_sprite: brew_sprite,
            _tiles_sprite: tiles_sprite, // can't drop this memory
            tiles_block: tiles_block,
            objects: objects,
            num_objects: 1,
            cur_tick: 0,
        }
    }

    fn update(&mut self) {
        let obj_max_x = (display::get_width() - self.brew_sprite.width() as u32) as i32;
        let obj_max_y = (display::get_height() - self.brew_sprite.height() as u32) as i32;

        for i in 0..self.objects.len() {
            let obj = &mut self.objects[i];
            let mut x = obj.x + obj.dx;
            let mut y = obj.y + obj.dy;

            if x >= obj_max_x { x -= obj_max_x; }
            if x < 0 { x += obj_max_x; }
            if y >= obj_max_y { y -= obj_max_y; }
            if y < 0 { y += obj_max_y; }

            obj.x = x;
            obj.y = y;
            obj.scale_factor = ((self.cur_tick as f32) * 0.1 + (i as f32)).sin() * 0.5 + 1.5;
        }

        self.cur_tick += 1;
    }

    fn render(&self) {
        let disp = display::get();
        rdpq::attach_clear(&disp, None);

        // Draw the tile background, by playing back the compiled block.
        // This is using copy mode by default, but notice how it can switch
        // to standard mode (aka "1 cycle" in RDP terminology) in a completely
        // transparent way. Even if the block is compiled, the RSP commands within it
        // will adapt its commands to the current render mode, Try uncommenting
        // the line below to see.
        rdpq::debug_log_msg("tiles");
        rdpq::set_mode_copy(false);
        //rdqp::set_mode_standard();
        self.tiles_block.run();

        // Draw the brew sprites. Use standard mode because copy mode cannot handle
        // scaled sprites.
        rdpq::debug_log_msg("sprites");
        rdpq::set_mode_standard();
        rdpq::mode_filter(rdpq::Filter::Bilinear);
        rdpq::mode_alphacompare(1); // colorkey (draw pixel with alpha >= 1)

        for i in 0..self.num_objects {
            let obj = &self.objects[i];
            rdpq::sprite_blit(&self.brew_sprite, obj.x as f32, obj.y as f32, rdpq::BlitParms {
                scale_x: obj.scale_factor,
                scale_y: obj.scale_factor,
                ..Default::default()
            });
        }

        rdpq::detach_show();
    }
}

#[no_mangle]
extern "C" fn main() -> ! {
    debug::init(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    display::init(Resolution::_320x240, BitDepth::Bpp16, 3, Gamma::None, FilterOptions::Resample);

    joypad::init();
    timer::init();

    dfs::init(None).unwrap_or_else(|e| panic!("Could not initialize filesystem: {:?}", e));

    rdpq::init();
    rdpq::debug_start();

    let mut app = App::new();
    app.update();

    let do_update = Arc::new(AtomicBool::new(false));
    let do_update_clone = do_update.clone();
    let _timer = Timer::new(timer::make_ticks(1000000 / 60), timer::Mode::Continuous, Box::new(move |_| {
        do_update_clone.store(true, Ordering::SeqCst);
    }));

    loop {
        if let Ok(_) = do_update.compare_exchange(true, false, Ordering::SeqCst, Ordering::Acquire) {
            app.update();
        }
        app.render();
        joypad::poll();

        let ckeys = joypad::Port::get_port_1().get_buttons_pressed();
        if ckeys.c_up && app.num_objects < NUM_OBJECTS {
            app.num_objects += 1;
        }

        if ckeys.c_down && app.num_objects > 1 {
            app.num_objects -= 1;
        }
    }
}

