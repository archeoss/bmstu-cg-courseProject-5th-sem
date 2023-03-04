use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use bmstu_cg_courseProject_5th_sem::{
    app_factory::{canvas_factory::create_canvas, drawer::create_frame_drawer},
    managers::camera_manager::CameraManager,
    models::frame_model::Point,
    objects::revolution::{BodiesOfRevolution, RevolutionBuilder},
};
use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
fn constSubdiv_bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut group = c.benchmark_group("Spheres, CONST=Subdivision");
    group.plot_config(plot_config);
    group.sample_size(25);
    let mut models_0 = Vec::new();
    let mut models_1 = Vec::new();
    let mut models_2 = Vec::new();
    let mut man = CameraManager::new((PI / 30.0).to_radians(), 1.0, 1000.0, (3200.0, 2600.0));
    man.spawn_camera();
    let camera = man.active_camera().unwrap();
    let canvas = Rc::new(RefCell::new(
        create_canvas("skia", 3200, 2600 /*self.width, self.height*/).unwrap(),
    ));
    let drawer0 = Rc::new(RefCell::new(
        create_frame_drawer("std", canvas.clone()).unwrap(),
    ));
    drawer0.borrow_mut().set_camera(camera.clone());
    let drawer1 = Rc::new(RefCell::new(
        create_frame_drawer("std", canvas.clone()).unwrap(),
    ));
    drawer1.borrow_mut().set_camera(camera.clone());
    let drawer2 = Rc::new(RefCell::new(
        create_frame_drawer("std", canvas.clone()).unwrap(),
    ));
    drawer2.borrow_mut().set_camera(camera.clone());
    for size in (100..=1000).step_by(100) {
        let mut builder = RevolutionBuilder::new();
        for i in 0..100 {
            let model = builder
                .build(
                    String::from("Sphere"),
                    BodiesOfRevolution::Sphere(20.0, 0),
                    [255; 4],
                )
                .unwrap();
            models_0.push(Rc::new(RefCell::new(model)));
            let model = builder
                .build(
                    String::from("Sphere"),
                    BodiesOfRevolution::Sphere(20.0, 1),
                    [255; 4],
                )
                .unwrap();
            models_1.push(Rc::new(RefCell::new(model)));
            let model = builder
                .build(
                    String::from("Sphere"),
                    BodiesOfRevolution::Sphere(20.0, 2),
                    [255; 4],
                )
                .unwrap();
            models_2.push(Rc::new(RefCell::new(model)));
        }
        group.bench_with_input(BenchmarkId::new("SUBDIV=0", size), &size, |b, _size| {
            b.iter(|| {
                black_box(
                    drawer0
                        .borrow_mut()
                        .draw_in_3d(&models_0, Point::new(100.0, 100.0, 100.0)),
                );
            });
        });
        group.bench_with_input(BenchmarkId::new("SUBDIV=1", size), &size, |b, _size| {
            b.iter(|| {
                black_box(
                    drawer1
                        .borrow_mut()
                        .draw_in_3d(&models_1, Point::new(100.0, 100.0, 100.0)),
                );
            });
        });
        group.bench_with_input(BenchmarkId::new("SUBDIV=2", size), &size, |b, _size| {
            b.iter(|| {
                black_box(
                    drawer2
                        .borrow_mut()
                        .draw_in_3d(&models_2, Point::new(100.0, 100.0, 100.0)),
                );
            });
        });
    }

    group.finish();
}

fn figures_bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut group = c.benchmark_group("Figures");
    group.plot_config(plot_config);
    group.sample_size(25);

    let mut man = CameraManager::new((PI / 30.0).to_radians(), 1.0, 1000.0, (3200.0, 2600.0));
    man.spawn_camera();
    let camera = man.active_camera().unwrap();
    let canvas = Rc::new(RefCell::new(
        create_canvas("skia", 3200, 2600 /*self.width, self.height*/).unwrap(),
    ));
    let drawer0 = Rc::new(RefCell::new(
        create_frame_drawer("std", canvas.clone()).unwrap(),
    ));
    drawer0.borrow_mut().set_camera(camera.clone());
    let drawer1 = Rc::new(RefCell::new(
        create_frame_drawer("std", canvas.clone()).unwrap(),
    ));
    drawer1.borrow_mut().set_camera(camera.clone());
    let drawer2 = Rc::new(RefCell::new(
        create_frame_drawer("std", canvas.clone()).unwrap(),
    ));
    drawer2.borrow_mut().set_camera(camera.clone());
    for size in (0..=4) {
        let mut builder = RevolutionBuilder::new();
        let mut models_0 = Vec::new();
        let mut models_1 = Vec::new();
        let mut models_2 = Vec::new();
        for i in 0..100 {
            let model = builder
                .build(
                    String::from("Sphere"),
                    BodiesOfRevolution::Sphere(60.0, size),
                    [255; 4],
                )
                .unwrap();
            if i % 2 == 0 {
                models_0.push(Rc::new(RefCell::new(model)));
            }
            let model = builder
                .build(
                    String::from("Sphere"),
                    BodiesOfRevolution::Cone(60.0, 60.0, size),
                    [255; 4],
                )
                .unwrap();
            models_1.push(Rc::new(RefCell::new(model)));
            let model = builder
                .build(
                    String::from("Sphere"),
                    BodiesOfRevolution::Cylinder(60.0, 60.0, 60.0, size),
                    [255; 4],
                )
                .unwrap();
            models_2.push(Rc::new(RefCell::new(model)));
        }
        group.bench_with_input(BenchmarkId::new("Spheres", size), &size, |b, _size| {
            b.iter(|| {
                black_box(
                    drawer0
                        .borrow_mut()
                        .draw_in_3d(&models_0, Point::new(100.0, 100.0, 100.0)),
                );
            });
        });
        group.bench_with_input(BenchmarkId::new("Cones", size), &size, |b, _size| {
            b.iter(|| {
                black_box(
                    drawer1
                        .borrow_mut()
                        .draw_in_3d(&models_1, Point::new(100.0, 100.0, 100.0)),
                );
            });
        });
        group.bench_with_input(BenchmarkId::new("Cylinders", size), &size, |b, _size| {
            b.iter(|| {
                black_box(
                    drawer2
                        .borrow_mut()
                        .draw_in_3d(&models_2, Point::new(100.0, 100.0, 100.0)),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(benches, figures_bench, constSubdiv_bench);
criterion_main!(benches);
