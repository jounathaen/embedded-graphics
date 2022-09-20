use criterion::*;
use embedded_graphics::{
    framebuffer::{buffer_size, Framebuffer},
    geometry::Point,
    pixelcolor::{raw::LittleEndian, BinaryColor},
};

fn framebuffer_set_1bpp(c: &mut Criterion) {
    c.bench_function("framebuffer set pixel 1bpp", |b| {
        let mut fb = Framebuffer::<
            BinaryColor,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<BinaryColor>(320, 240) },
        >::new();

        b.iter(|| {
            fb.set_pixel(Point::new(1, 1), BinaryColor::On);
            fb.set_pixel(Point::new(300, 200), BinaryColor::On);
        })
    });
}

fn framebuffer_get_1bpp(c: &mut Criterion) {
    c.bench_function("framebuffer get pixel 1bpp", |b| {
        let fb = Framebuffer::<
            BinaryColor,
            _,
            LittleEndian,
            320,
            240,
            { buffer_size::<BinaryColor>(320, 240) },
        >::new();

        b.iter(|| {
            fb.pixel(Point::new(1, 1));
            fb.pixel(Point::new(300, 200));
        })
    });
}

criterion_group!(framebuffer, framebuffer_set_1bpp, framebuffer_get_1bpp);
criterion_main!(framebuffer);
