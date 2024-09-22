use std::cell::RefCell;
use std::cmp::PartialEq;
use std::ffi::{c_int, c_void};
use std::rc::Rc;
use nanovg_sys::{nvgBeginPath, nvgCreateImage, nvgCreateImageMem, nvgDeleteImage, nvgFill, nvgFillColor, nvgFillPaint, nvgImagePattern, nvgIntersectScissor, NVGpaint, nvgRect, nvgRestore, nvgSave};
use yoga_sys::YGNodeSetContext;
use crate::core::attribute::register_string_xml_attribute;
use crate::core::frame_context::{frame_context, FrameContext};
use crate::core::resource::read_to_bytes;
use crate::core::theme::nvg_rgb;
use crate::core::view_base::{View, ViewBase, ViewData};
use crate::core::view_box::{Axis, BoxTrait};
use crate::core::view_drawer::{ViewDrawer, ViewTrait};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;
use crate::core::views::add_view;

/// This dictates what to do with the image if there is not
/// enough room for the view to grow and display the whole image,
/// or if the view is bigger than the image
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum ImageScalingType {
    /// The image is scaled to fit the view boundaries, aspect ratio is conserved
    Fit,
    /// The image is stretched to fit the view boundaries (aspect ratio is not conserved). The original image dimensions are entirely ignored in the layout process.
    Stretch,
    /// The image is either cropped (not enough space) or untouched (too much space)
    Crop,
}

/// This dictates what interpolation to use when down / up scaling the image
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum ImageInterpolation {
    Linear,
    Nearest,
}

/// Alignment of the image inside the view for FIT and CROP scaling types
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum ImageAlignment {
    Top,
    Right,
    Bottom,
    Left,
    Center,
}

pub struct ImageData {
    scaling_type: ImageScalingType,
    align: ImageAlignment,
    interpolation: ImageInterpolation,
    texture: c_int,
    paint: NVGpaint,
    original_image_width: f32,
    original_image_height: f32,
    image_x: f32,
    image_y: f32,
    image_height: f32,
    image_width: f32,
    image_bytes: Vec<u8>,
}

impl Default for ImageData {
    fn default() -> Self {
        Self {
            scaling_type: ImageScalingType::Fit,
            align: ImageAlignment::Center,
            interpolation: ImageInterpolation::Linear,
            texture: 0,
            paint: NVGpaint{
                xform: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                extent: [0.0, 0.0],
                radius: 0.0,
                feather: 0.0,
                innerColor: nvg_rgb(0, 0, 0),
                outerColor: nvg_rgb(0, 0, 0),
                image: 0,
            },
            original_image_width: 0.0,
            original_image_height: 0.0,
            image_x: 0.0,
            image_y: 0.0,
            image_height: 0.0,
            image_width: 0.0,
            image_bytes: vec![],
        }
    }
}

/// An image. The view will try to grow as much
/// as possible to fit the image. The scaling type dictates
/// what to do with the image if there is not enough or too much space
/// for the view compared to the image inside.
/// Supported formats are: JPG, PNG, TGA, BMP and GIF (not animated).
pub struct Image {
    view_data: Rc<RefCell<ViewData>>,
    image_data: Rc<RefCell<ImageData>>,
}

impl Default for Image {
    fn default() -> Self {
        let mut s = Self {
            view_data: Default::default(),
            image_data: Default::default(),
        };

        register_string_xml_attribute("image", Box::new(|view,value| {
            let view =  &mut *view.borrow_mut();
            match view {
                View::Image(v) => v.set_image_from_file(value),
                _ => {}
            }
        }));

        s
    }
}

impl Image {
    pub fn create() -> Rc<RefCell<View>> {
        let s = Image::default();
        let v= Rc::new(RefCell::new(View::Image(s)));
        add_view(&v.borrow().id(), v.clone());
        let boxed_id = Box::new(String::from(v.borrow().id()));
        unsafe {
            YGNodeSetContext(v.borrow().view_data().borrow().yg_node, Box::into_raw(boxed_id) as *mut c_void);
        }
        v
    }
}

pub trait ImageTrait: ViewTrait {

    fn image_data(&self) -> &Rc<RefCell<ImageData>>;

    fn get_image_flags(&self) -> c_int {
        if self.image_data().borrow().interpolation == ImageInterpolation::Nearest {
            return nanovg_sys::NVGimageFlags::NVG_IMAGE_NEAREST.bits();
        }
        return 0;
    }

    fn set_image_from_file(&self, path: &str) {
        let vg = frame_context();

        // Free the old texture if necessary
        if self.image_data().borrow().texture != 0 {
            unsafe {
                nvgDeleteImage(vg, self.image_data().borrow().texture);
            }
        }

        // Load the new texture
        let flags = self.get_image_flags();
        self.image_data().borrow_mut().image_bytes = read_to_bytes(path).unwrap();
        let image_bytes_len = self.image_data().borrow_mut().image_bytes.len() as c_int;
        let texture = unsafe {
            nvgCreateImageMem(vg, flags, self.image_data().borrow_mut().image_bytes.as_mut_ptr(), image_bytes_len)
        };
        self.image_data().borrow_mut().texture = texture;
    }

    fn invalidate_image_bounds(&self) {
        if self.image_data().borrow().texture == 0 {
            return;
        }

        let width = self.width();
        let height = self.height();

        let view_aspect_ratio = width / height;
        let image_aspect_ratio = self.image_data().borrow().original_image_width / self.image_data().borrow().original_image_height;

        let scaling_type = self.image_data().borrow().scaling_type;

        match scaling_type {
            ImageScalingType::Fit => {
                if view_aspect_ratio >= image_aspect_ratio {
                    let mut this = self.image_data().borrow_mut();
                    this.image_height = height;
                    this.image_width = this.image_height * image_aspect_ratio;
                    this.image_x = (width - this.image_width) / 2.0;
                    this.image_y = 0.0;
                } else {
                    let mut this = self.image_data().borrow_mut();
                    this.image_width = width;
                    this.image_height = this.image_width * image_aspect_ratio;
                    this.image_y = (height - this.image_height) / 2.0;
                    this.image_x = 0.0;
                }
            }
            ImageScalingType::Stretch => {
                let mut this = self.image_data().borrow_mut();
                this.image_x = 0.0;
                this.image_y = 0.0;
                this.image_width = width;
                this.image_height = height;
            }
            ImageScalingType::Crop => {
                if view_aspect_ratio < image_aspect_ratio {
                    let mut this = self.image_data().borrow_mut();
                    this.image_height = this.original_image_height;
                    this.image_width = this.image_height * image_aspect_ratio;
                    this.image_x = (width - this.image_width) / 2.0;
                    this.image_y = 0.0;
                } else {
                    let mut this = self.image_data().borrow_mut();
                    this.image_width = this.original_image_width;
                    this.image_height = this.image_width * image_aspect_ratio;
                    this.image_y = (height - this.image_height) / 2.0;
                    this.image_x = 0.0;
                }
            }
        }

        // Create the paint - actual X and Y positions are updated every frame in draw() to apply translation (scrolling...)
        let vg = frame_context();
        let paint = unsafe {
            let this = self.image_data().borrow();
            nvgImagePattern(vg, 0.0, 0.0, this.image_width, this.image_height, 0.0, this.texture, 1.0)
        };
        let mut this = self.image_data().borrow_mut();
        this.paint = paint;
    }
}

impl ViewDrawer for Image {
    fn draw(&self, ctx: &FrameContext, x: f32, y: f32, width: f32, height: f32) {
        let height = 30.0;
        if self.image_data.borrow().texture == 0 {
            return;
        }

        if self.image_data.borrow().scaling_type == ImageScalingType::Crop {
            unsafe {
                nvgSave(ctx.context);
                nvgIntersectScissor(ctx.context, x, y, width, height);
            }
        }

        let coord_x = x + self.image_data.borrow().image_x;
        let coord_y = y + self.image_data.borrow().image_y;

        self.image_data.borrow_mut().paint.xform[4] = coord_x;
        self.image_data.borrow_mut().paint.xform[5] = coord_y;
        trace!("image draw {} {} {} {}", coord_x, coord_y, self.image_data.borrow().image_width, self.image_data.borrow().image_height);
        unsafe {
            nvgBeginPath(ctx.context);
            nvgFillColor(ctx.context, nvg_rgb(255, 0,0 ));
            nvgRect(ctx.context, coord_x, coord_y, self.image_data.borrow().image_width, self.image_data.borrow().image_height);
            nvgFillPaint(ctx.context, self.a_paint(self.image_data.borrow().paint));
            nvgFill(ctx.context);

            if self.image_data.borrow().scaling_type == ImageScalingType::Crop {
                unsafe {
                    nvgRestore(ctx.context);
                }
            }
        }
    }
}

impl ViewLayout for Image {}

impl ViewStyle for Image {}

impl ViewBase for Image {
    fn view_data(&self) -> &Rc<RefCell<ViewData>> {
        &self.view_data
    }
}

impl ViewTrait for Image {}

impl ImageTrait for Image {
    fn image_data(&self) -> &Rc<RefCell<ImageData>> {
        &self.image_data
    }
}
