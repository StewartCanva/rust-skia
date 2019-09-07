use crate::prelude::*;
use crate::{
    scalar, BlendMode, Color, ColorChannel, ColorFilter, FilterQuality, IPoint, IRect, ISize,
    Image, ImageFilter, Matrix, Paint, Picture, Point3, Rect, Region, TileMode, Vector,
};
use skia_bindings as sb;
use skia_bindings::SkImageFilter;

pub fn alpha_threshold<'a>(
    region: &Region,
    inner_min: scalar,
    outer_max: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_AlphaThreshold(
            region.native(),
            inner_min,
            outer_max,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn arithmetic<'a>(
    k1: scalar,
    k2: scalar,
    k3: scalar,
    k4: scalar,
    enforce_pm_color: bool,
    background: ImageFilter,
    foreground: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Arithmetic(
            k1,
            k2,
            k3,
            k4,
            enforce_pm_color,
            background.into_ptr(),
            foreground.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn blur<'a>(
    (sigma_x, sigma_y): (scalar, scalar),
    tile_mode: impl Into<Option<TileMode>>,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Blur(
            sigma_x,
            sigma_y,
            tile_mode.into().unwrap_or(TileMode::Decal).into_native(),
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn color_filter<'a>(
    cf: ColorFilter,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_ColorFilter(
            cf.into_ptr(),
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn compose(outer: ImageFilter, inner: ImageFilter) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Compose(outer.into_ptr(), inner.into_ptr())
    })
}

pub fn displacement_map<'a>(
    (x_channel_selector, y_channel_selector): (ColorChannel, ColorChannel),
    scale: scalar,
    displacement: ImageFilter,
    color: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DisplacementMap(
            x_channel_selector.into_native(),
            y_channel_selector.into_native(),
            scale,
            displacement.into_ptr(),
            color.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn drop_shadow<'a>(
    delta: impl Into<Vector>,
    (sigma_x, sigma_y): (scalar, scalar),
    color: impl Into<Color>,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    let color = color.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DropShadow(
            delta.x,
            delta.y,
            sigma_x,
            sigma_y,
            color.into_native(),
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn drop_shadow_only<'a>(
    delta: impl Into<Vector>,
    (sigma_x, sigma_y): (scalar, scalar),
    color: impl Into<Color>,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    let color = color.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DropShadowOnly(
            delta.x,
            delta.y,
            sigma_x,
            sigma_y,
            color.into_native(),
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn image<'a>(
    image: Image,
    src_rect: impl Into<Option<&'a Rect>>,
    dst_rect: impl Into<Option<&'a Rect>>,
    filter_quality: impl Into<Option<FilterQuality>>,
) -> Option<ImageFilter> {
    let image_rect = Rect::from_iwh(image.width(), image.height());
    let src_rect = src_rect.into().unwrap_or(&image_rect);
    let dst_rect = dst_rect.into().unwrap_or(&image_rect);
    let filter_quality = filter_quality.into().unwrap_or(FilterQuality::High);

    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Image(
            image.into_ptr(),
            src_rect.as_ref().native(),
            dst_rect.as_ref().native(),
            filter_quality.into_native(),
        )
    })
}

pub fn magnifier<'a>(
    src_rect: impl AsRef<Rect>,
    inset: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Magnifier(
            src_rect.as_ref().native(),
            inset,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn matrix_convolution<'a>(
    kernel_size: impl Into<ISize>,
    kernel: &[scalar],
    gain: scalar,
    bias: scalar,
    kernel_offset: impl Into<IPoint>,
    tile_mode: TileMode,
    convolve_alpha: bool,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    let kernel_size = kernel_size.into();
    assert_eq!(
        (kernel_size.width * kernel_size.height) as usize,
        kernel.len()
    );
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_MatrixConvolution(
            kernel_size.native(),
            kernel.as_ptr(),
            gain,
            bias,
            kernel_offset.into().native(),
            tile_mode.into_native(),
            convolve_alpha,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn matrix_transform(
    matrix: &Matrix,
    filter_quality: FilterQuality,
    input: ImageFilter,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_MatrixTransform(
            matrix.native(),
            filter_quality.into_native(),
            input.into_ptr(),
        )
    })
}

#[allow(clippy::new_ret_no_self)]
pub fn merge<'a>(
    filters: impl IntoIterator<Item = ImageFilter>,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    let filter_ptrs: Vec<*mut SkImageFilter> = filters.into_iter().map(|f| f.into_ptr()).collect();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Merge(
            filter_ptrs.as_ptr(),
            filter_ptrs.len().try_into().unwrap(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn offset<'a>(
    delta: impl Into<Vector>,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    let delta = delta.into();
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Offset(
            delta.x,
            delta.y,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn paint<'a>(paint: &Paint, crop_rect: impl Into<Option<&'a IRect>>) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Paint(paint.native(), crop_rect.into().native_ptr_or_null())
    })
}

pub fn picture<'a>(
    picture: Picture,
    target_rect: impl Into<Option<&'a Rect>>,
) -> Option<ImageFilter> {
    let picture_rect = picture.cull_rect();
    let target_rect = target_rect.into().unwrap_or(&picture_rect);

    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Picture(picture.into_ptr(), target_rect.native())
    })
}

pub fn tile(
    src: impl AsRef<Rect>,
    dst: impl AsRef<Rect>,
    input: ImageFilter,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Tile(
            src.as_ref().native(),
            dst.as_ref().native(),
            input.into_ptr(),
        )
    })
}

pub fn xfermode<'a>(
    blend_mode: BlendMode,
    background: ImageFilter,
    foreground: impl Into<Option<ImageFilter>>,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Xfermode(
            blend_mode.into_native(),
            background.into_ptr(),
            foreground.into().into_ptr_or_null(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn dilate<'a>(
    (radius_x, radius_y): (i32, i32),
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Dilate(
            radius_x,
            radius_y,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn erode<'a>(
    (radius_x, radius_y): (i32, i32),
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_Erode(
            radius_x,
            radius_y,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn distant_lit_diffuse<'a>(
    direction: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_DistantLitDiffuse(
            direction.into().native(),
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn point_lit_diffuse<'a>(
    location: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_PointLitDiffuse(
            location.into().native(),
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn spot_lit_diffuse<'a>(
    location: impl Into<Point3>,
    target: impl Into<Point3>,
    specular_exponent: scalar,
    cutoff_angle: scalar,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    kd: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_SpotLitDiffuse(
            location.into().native(),
            target.into().native(),
            specular_exponent,
            cutoff_angle,
            light_color.into().into_native(),
            surface_scale,
            kd,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn distant_lit_specular<'a>(
    direction: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_ImageFilters_DistantLitSpecular(
            direction.into().native(),
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

pub fn point_lit_specular<'a>(
    location: impl Into<Point3>,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_PointLitSpecular(
            location.into().native(),
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
pub fn spot_lit_specular<'a>(
    location: impl Into<Point3>,
    target: impl Into<Point3>,
    specular_exponent: scalar,
    cutoff_angle: scalar,
    light_color: impl Into<Color>,
    surface_scale: scalar,
    ks: scalar,
    shininess: scalar,
    input: ImageFilter,
    crop_rect: impl Into<Option<&'a IRect>>,
) -> Option<ImageFilter> {
    ImageFilter::from_ptr(unsafe {
        sb::C_SkImageFilters_SpotLitSpecular(
            location.into().native(),
            target.into().native(),
            specular_exponent,
            cutoff_angle,
            light_color.into().into_native(),
            surface_scale,
            ks,
            shininess,
            input.into_ptr(),
            crop_rect.into().native_ptr_or_null(),
        )
    })
}
