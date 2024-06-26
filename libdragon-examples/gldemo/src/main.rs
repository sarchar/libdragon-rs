#![no_std]
#![no_main]
#![feature(offset_of)]

use libdragon::*;

use libdragon::display::{Resolution, BitDepth, Gamma, FilterOptions};
use libdragon::sprite::Sprite;
use libdragon::surface::{TexFormat, Surface};
use libdragon::rdpq::{TexParms, TexParmsST};

mod camera;
use camera::Camera;

mod cube;
use cube::Cube;

mod plane;
use plane::Plane;

mod decal;
use decal::Decal;

mod sphere;
use sphere::Sphere;

mod skinned;
use skinned::Skinned;

mod prim_test;
use prim_test::PrimTest;

use core_maths::*;

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    texcoord: [f32; 2],
    normal  : [f32; 3],
    color   : u32,
}


static ENVIRONMENT_COLOR: [f32; 4] = [0.1, 0.03, 0.2, 1.0];

static LIGHT_POS: [[f32; 4]; 8] = [
    [  1.0, 0.0,  0.0, 0.0 ],
    [ -1.0, 0.0,  0.0, 0.0 ],
    [  0.0, 0.0,  1.0, 0.0 ],
    [  0.0, 0.0, -1.0, 0.0 ],
    [  8.0, 3.0,  0.0, 1.0 ],
    [ -8.0, 3.0,  0.0, 1.0 ],
    [  0.0, 3.0,  8.0, 1.0 ],
    [  0.0, 3.0, -8.0, 1.0 ],
];

static LIGHT_DIFFUSE: [[f32; 4]; 8] = [
    [ 1.0, 0.0, 0.0, 1.0 ],
    [ 0.0, 1.0, 0.0, 1.0 ],
    [ 0.0, 0.0, 1.0, 1.0 ],
    [ 1.0, 1.0, 0.0, 1.0 ],
    [ 1.0, 0.0, 1.0, 1.0 ],
    [ 0.0, 1.0, 1.0, 1.0 ],
    [ 1.0, 1.0, 1.0, 1.0 ],
    [ 1.0, 1.0, 1.0, 1.0 ],
];


struct App<'a> {
    camera: Camera,
    sprites: [Sprite<'a>; 4],
    textures: [u32; 4],
    texture_index: usize,
    frames: u64,
    animation: u32,

    sphere: Sphere,
    cube: Cube,
    plane: Plane,
    decal: Decal,
    skinned: Skinned,
    prim_test: PrimTest,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let sprites = [
            Sprite::load(dfs::PathBuf::from("rom:/circle0.sprite")).unwrap(),
            Sprite::load(dfs::PathBuf::from("rom:/diamond0.sprite")).unwrap(),
            Sprite::load(dfs::PathBuf::from("rom:/pentagon0.sprite")).unwrap(),
            Sprite::load(dfs::PathBuf::from("rom:/triangle0.sprite")).unwrap(),
        ];

        let mut sphere = Sphere::new();
        sphere.make_sphere_mesh();

        let cube = Cube::new();
        let mut plane = Plane::new();
        plane.make_plane_mesh();

        let aspect_ratio = (display::get_width() as f64) / (display::get_height() as f64);
        let near_plane = 1.0;
        let far_plane = 50.0;

        gl::MatrixMode(gl::PROJECTION);
        gl::LoadIdentity();
        gl::Frustum(-near_plane*aspect_ratio, near_plane*aspect_ratio, -near_plane, near_plane, near_plane, far_plane);

        gl::MatrixMode(gl::MODELVIEW);
        gl::LoadIdentity();

        gl::LightModelfv(gl::LIGHT_MODEL_AMBIENT, &ENVIRONMENT_COLOR);
        gl::LightModeli(gl::LIGHT_MODEL_LOCAL_VIEWER, gl::TRUE);

        let light_radius = 10.0;
        for i in 0..8 as i32 {
            gl::Enable(gl::LIGHT0 + i);
            gl::Lightfv(gl::LIGHT0 + i, gl::DIFFUSE, &LIGHT_DIFFUSE[i as usize]);
            gl::Lightf(gl::LIGHT0 + i, gl::LINEAR_ATTENUATION, 2.0 / light_radius);
            gl::Lightf(gl::LIGHT0 + i, gl::QUADRATIC_ATTENUATION, 1.0 / (light_radius * light_radius));
        }

        let mat_diffuse = [1.0, 1.0, 1.0, 1.0];
        gl::Materialfv(gl::FRONT_AND_BACK, gl::AMBIENT_AND_DIFFUSE, &mat_diffuse);

        gl::Fogf(gl::FOG_START, 5.0);
        gl::Fogf(gl::FOG_END, 20.0);
        gl::Fogfv(gl::FOG_COLOR, &ENVIRONMENT_COLOR);

        gl::Enable(gl::MULTISAMPLE_ARB);

        let mut textures = [0u32; 4];
        gl::GenTextures(&mut textures);

        let min_filter = gl::LINEAR; // gl::LINEAR_MIPMAP_LINEAR
        for i in 0..4 {
            gl::BindTexture(gl::TEXTURE_2D, textures[i]);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter);

            gl::SpriteTextureN64(gl::TEXTURE_2D, &sprites[i], TexParms {
                s: TexParmsST {
                    repeats: rdpq::REPEAT_INFINITE,
                    ..Default::default()
                },
                t: TexParmsST {
                    repeats: rdpq::REPEAT_INFINITE,
                    ..Default::default()
                },
                ..Default::default()
            });
        }

        rspq::profile_start();

        Self {
            camera: Camera {
                distance: -10.0,
                rotation: 0.0,
            },
            sprites: sprites, // can't let sprites memory be dropped, as they contain the texture data
            textures: textures,
            texture_index: 0,
            frames: 0,
            animation: 3283,

            sphere: sphere,
            cube: cube,
            plane: plane,
            decal: Decal::new(),
            skinned: Skinned::new(),
            prim_test: PrimTest::new(),
        }
    }

    fn set_light_positions(&self, rotation: f32) {
        gl::PushMatrix();
        gl::Rotatef(rotation*5.43, 0.0, 1.0, 0.0);
        for i in 0..8 as i32 {
            gl::Lightfv(gl::LIGHT0 + i, gl::POSITION, &LIGHT_POS[i as usize]);
        }
        gl::PopMatrix();
    }

    fn render(&mut self) {
        let disp = display::get();
        let zbuf = display::get_zbuf();
        rdpq::attach(&disp, Some(&zbuf));

        gl::context_begin();

        gl::ClearColor(ENVIRONMENT_COLOR[0], ENVIRONMENT_COLOR[1], ENVIRONMENT_COLOR[2], ENVIRONMENT_COLOR[3]);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        gl::MatrixMode(gl::MODELVIEW);
        self.camera.transform();

        let rotation = (self.animation as f32) * 0.5;
        self.set_light_positions(rotation);

        gl::Enable(gl::LIGHTING);
        gl::Enable(gl::NORMALIZE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);

        gl::Enable(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, self.textures[self.texture_index]);

        self.plane.render();
        self.decal.render();
        self.cube.render();
        self.skinned.render(&self.camera, self.animation as f32);

        gl::BindTexture(gl::TEXTURE_2D, self.textures[(self.texture_index + 1) % self.textures.len()]);
        self.sphere.render(rotation);

        gl::Disable(gl::TEXTURE_2D);
        gl::Disable(gl::LIGHTING);
        self.prim_test.render(rotation);

        // Draw a primitive with GL_RDPQ_TEXTURING_N64
        gl::Enable(gl::RDPQ_TEXTURING_N64);
        gl::Enable(gl::RDPQ_MATERIAL_N64);

        // When rendering with GL_RDPQ_TEXTURING_N64 we need to manualy specify the
        // tile size and if a 0.5 offset should be used since the ucode itself cannot
        // determine these. Here we set the tile size to be 32x32 and we apply an offset
        // since we are using bilinear texture filtering
        gl::TexSizeN64(32, 32);
        rdpq::sprite_upload(rdpq::Tile(0), &self.sprites[0], rdpq::TexParms {
            s: rdpq::TexParmsST { repeats: rdpq::REPEAT_INFINITE, ..Default::default() },
            t: rdpq::TexParmsST { repeats: rdpq::REPEAT_INFINITE, ..Default::default() },
            ..Default::default()
        });
        rdpq::set_mode_standard();
        rdpq::mode_filter(rdpq::Filter::Bilinear);

        gl::Begin(gl::TRIANGLE_FAN);
            gl::TexCoord2f(0.0, 0.0);
            gl::Vertex3f(-5.5, 1.0, -1.0);

            gl::TexCoord2f(0.0, 1.0);
            gl::Vertex3f(-5.5, 1.0, 1.0);

            gl::TexCoord2f(1.0, 1.0);
            gl::Vertex3f(-3.5, 1.0, 1.0);

            gl::TexCoord2f(1.0, 0.0);
            gl::Vertex3f(-3.5, 1.0, -1.0);
        gl::End();

        gl::Disable(gl::RDPQ_TEXTURING_N64);
        gl::Disable(gl::RDPQ_MATERIAL_N64);

        gl::context_end();

        rdpq::detach_show();

        rspq::profile_next_frame();

        self.frames += 1;
        if (self.frames % 60) == 0 {
            rspq::profile_dump();
            rspq::profile_reset();
            eprintln!("frame {}", self.frames);
        }
    }
}

#[no_mangle]
extern "C" fn main() -> ! {
    debug::init(debug::FEATURE_LOG_ISVIEWER | debug::FEATURE_LOG_USB);

    dfs::init(None).unwrap_or_else(|e| panic!("Could not initialize filesystem: {:?}", e));

    display::init(Resolution::_320x240, BitDepth::Bpp16, 3, Gamma::None, FilterOptions::ResampleAntialiasDedither);

    rdpq::init();
    gl::init();

    rdpq::debug_start();

    let mut app = App::new();

    joypad::init();

    let mut shade_model = gl::SMOOTH;
    let mut fog_enabled = false;

    loop {
        joypad::poll();
        let port = joypad::Port::get_port_1();
        let pressed = port.get_buttons_pressed();
        let held    = port.get_buttons_held();
        let inputs  = port.get_inputs();

        if held.a {
            app.animation += 1;
        }

        if held.b {
            app.animation -= 1;
        }

        if pressed.start {
            eprintln!("{}", app.animation);
        }

        if pressed.r {
            shade_model = if shade_model == gl::SMOOTH { gl::FLAT } else { gl::SMOOTH };
            gl::ShadeModel(shade_model);
        }

        if pressed.l {
            fog_enabled = !fog_enabled;
            if fog_enabled { gl::Enable(gl::FOG) } else { gl::Disable(gl::FOG) }
        }

        if pressed.c_up {
            app.sphere.increment_rings();
            app.sphere.increment_segments();
            app.sphere.make_sphere_mesh();
        }

        if pressed.c_down {
            app.sphere.decrement_rings();
            app.sphere.decrement_segments();
            app.sphere.make_sphere_mesh();
        }

        if pressed.c_right {
            app.texture_index = (app.texture_index + 1) % app.textures.len();
        }

        let y = (inputs.stick_y as f32) / 128.0;
        let x = (inputs.stick_x as f32) / 128.0;
        let mag = x*x + y*y;

        if mag.abs() > 0.01 {
            app.camera.distance += y * 0.2;
            app.camera.rotation = app.camera.rotation - x * 1.2;
        }

        app.render();
    }
}

