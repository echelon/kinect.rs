#![allow(unused)]

use k4a_sys_temp as k4a_sys;

#[derive(Clone)]
pub struct Calibration(pub k4a_sys::k4a_calibration_t);

impl Calibration {
    pub fn default() -> Self {
        let extrinsics = k4a_sys::_k4a_calibration_extrinsics_t {
            rotation: [0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
            translation: [0.0f32, 0.0f32, 0.0f32],
        };

        let extrinsics_4 = [
            extrinsics.clone(),
            extrinsics.clone(),
            extrinsics.clone(),
            extrinsics.clone(),
        ];

        let extrinsics_4_4 = [
            extrinsics_4.clone(),
            extrinsics_4.clone(),
            extrinsics_4.clone(),
            extrinsics_4.clone(),
        ];

        let camera_calibration = k4a_sys::_k4a_calibration_camera_t {
            resolution_width: 0,
            resolution_height: 0,
            metric_radius: 0.0f32,
            extrinsics: extrinsics.clone(),
            intrinsics: k4a_sys::_k4a_calibration_intrinsics_t {
                type_: 0,
                parameter_count: 0,
                parameters: k4a_sys::k4a_calibration_intrinsic_parameters_t {
                    param: k4a_sys::k4a_calibration_intrinsic_parameters_t__param {
                        cx: 0.0f32,
                        cy: 0.0f32,
                        fx: 0.0f32,
                        fy: 0.0f32,
                        k1: 0.0f32,
                        k2: 0.0f32,
                        k3: 0.0f32,
                        k4: 0.0f32,
                        k5: 0.0f32,
                        k6: 0.0f32,
                        codx: 0.0f32,
                        cody: 0.0f32,
                        p2: 0.0f32,
                        p1: 0.0f32,
                        metric_radius: 0.0f32,
                    },
                },
            },
        };

        Self(k4a_sys::k4a_calibration_t {
            color_camera_calibration: camera_calibration.clone(),
            depth_camera_calibration: camera_calibration.clone(),
            color_resolution: 0,
            depth_mode: 0,
            extrinsics: extrinsics_4_4,
        })
    }

    /// Return the Calibration's color camera resolution width.
    pub fn color_camera_resolution_width(&self) -> i32 {
        self.0.color_camera_calibration.resolution_width
    }

    /// Return the Calibration's color camera resolution height.
    pub fn color_camera_resolution_height(&self) -> i32 {
        self.0.color_camera_calibration.resolution_height
    }
    /// Return the Calibration's depth camera resolution width.
    pub fn depth_camera_resolution_width(&self) -> i32 {
        self.0.depth_camera_calibration.resolution_width
    }

    /// Return the Calibration's depth camera resolution height.
    pub fn depth_camera_resolution_height(&self) -> i32 {
        self.0.depth_camera_calibration.resolution_height
    }

    // TODO: Make this the `Debug` trait output instead.
    pub fn debug_print(&self) {
        println!("===== CALIBRATION =====");
        println!("\t Color resolution: {}", self.0.color_resolution);
        println!("\t Depth mode: {}", self.0.depth_mode);
        println!("\t Extrinsics: {:?}", self.0.extrinsics);

        println!("\t depth.resolution_width: {}", self.0.depth_camera_calibration.resolution_width);
        println!("\t depth.resolution_height: {}", self.0.depth_camera_calibration.resolution_height);
        println!("\t depth.metric_radius: {}", self.0.depth_camera_calibration.metric_radius);
        println!("\t depth.extrinsics: {:?}", self.0.depth_camera_calibration.extrinsics);
        println!("\t depth.intrinsics.type: {}", self.0.depth_camera_calibration.intrinsics.type_);
        println!("\t depth.intrinsics.parameter_count: {}", self.0.depth_camera_calibration.intrinsics.parameter_count);
        unsafe {
            // NB: This is a union field, so we have to use unsafe access
            println!("\t depth.intrinsics.parameters.param.cx: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.cx);
            println!("\t depth.intrinsics.parameters.param.cy: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.cy);
            println!("\t depth.intrinsics.parameters.param.fx: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.fx);
            println!("\t depth.intrinsics.parameters.param.fy: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.fy);
            println!("\t depth.intrinsics.parameters.param.k1: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k1);
            println!("\t depth.intrinsics.parameters.param.k2: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k2);
            println!("\t depth.intrinsics.parameters.param.k3: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k3);
            println!("\t depth.intrinsics.parameters.param.k4: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k4);
            println!("\t depth.intrinsics.parameters.param.k5: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k5);
            println!("\t depth.intrinsics.parameters.param.k6: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k6);
            println!("\t depth.intrinsics.parameters.param.codx: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.codx);
            println!("\t depth.intrinsics.parameters.param.cody: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.cody);
            println!("\t depth.intrinsics.parameters.param.p2: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.p2);
            println!("\t depth.intrinsics.parameters.param.p1: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.p1);
            println!("\t depth.intrinsics.parameters.param.metric_radius: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.metric_radius);
        }
        println!("\t color.resolution_width: {}", self.0.color_camera_calibration.resolution_width);
        println!("\t color.resolution_height: {}", self.0.color_camera_calibration.resolution_height);
        println!("\t color.metric_radius: {}", self.0.color_camera_calibration.metric_radius);
        println!("\t color.extrinsics: {:?}", self.0.color_camera_calibration.extrinsics);
        println!("\t color.intrinsics.type: {}", self.0.color_camera_calibration.intrinsics.type_);
        println!("\t color.intrinsics.parameter_count: {}", self.0.color_camera_calibration.intrinsics.parameter_count);
        unsafe {
            // NB: This is a union field, so we have to use unsafe access
            println!("\t color.intrinsics.parameters.param.cx: {}", self.0.color_camera_calibration.intrinsics.parameters.param.cx);
            println!("\t color.intrinsics.parameters.param.cy: {}", self.0.color_camera_calibration.intrinsics.parameters.param.cy);
            println!("\t color.intrinsics.parameters.param.fx: {}", self.0.color_camera_calibration.intrinsics.parameters.param.fx);
            println!("\t color.intrinsics.parameters.param.fy: {}", self.0.color_camera_calibration.intrinsics.parameters.param.fy);
            println!("\t color.intrinsics.parameters.param.k1: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k1);
            println!("\t color.intrinsics.parameters.param.k2: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k2);
            println!("\t color.intrinsics.parameters.param.k3: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k3);
            println!("\t color.intrinsics.parameters.param.k4: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k4);
            println!("\t color.intrinsics.parameters.param.k5: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k5);
            println!("\t color.intrinsics.parameters.param.k6: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k6);
            println!("\t color.intrinsics.parameters.param.codx: {}", self.0.color_camera_calibration.intrinsics.parameters.param.codx);
            println!("\t color.intrinsics.parameters.param.cody: {}", self.0.color_camera_calibration.intrinsics.parameters.param.cody);
            println!("\t color.intrinsics.parameters.param.p2: {}", self.0.color_camera_calibration.intrinsics.parameters.param.p2);
            println!("\t color.intrinsics.parameters.param.p1: {}", self.0.color_camera_calibration.intrinsics.parameters.param.p1);
            println!("\t color.intrinsics.parameters.param.metric_radius: {}", self.0.color_camera_calibration.intrinsics.parameters.param.metric_radius);
        }
        println!("==========");
    }
}
