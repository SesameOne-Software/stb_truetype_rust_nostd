// Generated by Hebron at 1/3/2022 8:59:56 AM

use crate::*;
use c_runtime;
use core;

#[derive(Debug, Copy, Clone)]
pub struct stbtt__active_edge {
    pub next: *mut stbtt__active_edge,
    pub fx: f32,
    pub fdx: f32,
    pub fdy: f32,
    pub direction: f32,
    pub sy: f32,
    pub ey: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct stbtt__bitmap {
    pub w: i32,
    pub h: i32,
    pub stride: i32,
    pub pixels: *mut u8,
}

#[derive(Debug, Copy, Clone)]
pub struct stbtt__edge {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub invert: i32,
}

impl core::default::Default for stbtt__active_edge {
    fn default() -> Self {
        stbtt__active_edge {
            next: core::ptr::null_mut(),
            fx: 0.0f32,
            fdx: 0.0f32,
            fdy: 0.0f32,
            direction: 0.0f32,
            sy: 0.0f32,
            ey: 0.0f32,
        }
    }
}

impl core::default::Default for stbtt__bitmap {
    fn default() -> Self {
        stbtt__bitmap {
            w: 0,
            h: 0,
            stride: 0,
            pixels: core::ptr::null_mut(),
        }
    }
}

impl core::default::Default for stbtt__edge {
    fn default() -> Self {
        stbtt__edge {
            x0: 0.0f32,
            y0: 0.0f32,
            x1: 0.0f32,
            y1: 0.0f32,
            invert: 0,
        }
    }
}

pub unsafe fn stbtt__fill_active_edges_new(
    scanline: *mut f32,
    scanline_fill: *mut f32,
    len: i32,
    mut e: *mut stbtt__active_edge,
    y_top: f32,
) {
    let y_bottom: f32 = y_top + ((1) as f32);
    while (e) != core::ptr::null_mut() {
        if (*e).fdx == ((0) as f32) {
            let x0: f32 = (*e).fx;
            if x0 < ((len) as f32) {
                if x0 >= ((0) as f32) {
                    stbtt__handle_clipped_edge(scanline, (x0) as i32, e, x0, y_top, x0, y_bottom);
                    stbtt__handle_clipped_edge(
                        (scanline_fill).offset(-((1) as isize)),
                        ((x0) as i32) + 1,
                        e,
                        x0,
                        y_top,
                        x0,
                        y_bottom,
                    );
                } else {
                    stbtt__handle_clipped_edge(
                        (scanline_fill).offset(-((1) as isize)),
                        0,
                        e,
                        x0,
                        y_top,
                        x0,
                        y_bottom,
                    );
                }
            }
        } else {
            let mut x0: f32 = (*e).fx;
            let mut dx: f32 = (*e).fdx;
            let mut xb: f32 = x0 + dx;
            let mut x_top: f32 = 0.0f32;
            let mut x_bottom: f32 = 0.0f32;
            let mut sy0: f32 = 0.0f32;
            let mut sy1: f32 = 0.0f32;
            let mut dy: f32 = (*e).fdy;
            if (*e).sy > y_top {
                x_top = (x0 + dx * ((*e).sy - y_top)) as f32;
                sy0 = ((*e).sy) as f32;
            } else {
                x_top = (x0) as f32;
                sy0 = (y_top) as f32;
            }
            if (*e).ey < y_bottom {
                x_bottom = (x0 + dx * ((*e).ey - y_top)) as f32;
                sy1 = ((*e).ey) as f32;
            } else {
                x_bottom = (xb) as f32;
                sy1 = (y_bottom) as f32;
            }
            if x_top >= ((0) as f32)
                && x_bottom >= ((0) as f32)
                && x_top < ((len) as f32)
                && x_bottom < ((len) as f32)
            {
                if ((x_top) as i32) == ((x_bottom) as i32) {
                    let mut height: f32 = 0.0f32;
                    let x: i32 = (x_top) as i32;
                    height = ((sy1 - sy0) * (*e).direction) as f32;
                    *scanline.offset((x) as isize) += stbtt__position_trapezoid_area(
                        height,
                        x_top,
                        ((x) as f32) + 1.0f32,
                        x_bottom,
                        ((x) as f32) + 1.0f32,
                    );
                    *scanline_fill.offset((x) as isize) += (height) as f32;
                } else {
                    let mut x: i32 = 0;
                    let mut x1: i32 = 0;
                    let mut x2: i32 = 0;
                    let mut y_crossing: f32 = 0.0f32;
                    let mut y_final: f32 = 0.0f32;
                    let mut step: f32 = 0.0f32;
                    let mut sign: f32 = 0.0f32;
                    let mut area: f32 = 0.0f32;
                    if x_top > x_bottom {
                        let mut t: f32 = 0.0f32;
                        sy0 = (y_bottom - (sy0 - y_top)) as f32;
                        sy1 = (y_bottom - (sy1 - y_top)) as f32;
                        t = (sy0) as f32;
                        sy0 = (sy1) as f32;
                        sy1 = (t) as f32;
                        t = (x_bottom) as f32;
                        x_bottom = (x_top) as f32;
                        x_top = (t) as f32;
                        dx = (-dx) as f32;
                        dy = (-dy) as f32;
                        t = (x0) as f32;
                        x0 = (xb) as f32;
                        xb = (t) as f32;
                    }
                    x1 = (x_top) as i32;
                    x2 = (x_bottom) as i32;
                    y_crossing = y_top + dy * (((x1 + 1) as f32) - x0);
                    y_final = y_top + dy * (((x2) as f32) - x0);
                    if y_crossing > y_bottom {
                        y_crossing = (y_bottom) as f32;
                    }
                    sign = ((*e).direction) as f32;
                    area = (sign * (y_crossing - sy0)) as f32;
                    *scanline.offset((x1) as isize) +=
                        stbtt__sized_triangle_area(area, ((x1 + 1) as f32) - x_top);
                    if y_final > y_bottom {
                        y_final = (y_bottom) as f32;
                        dy = (y_final - y_crossing) / ((x2 - (x1 + 1)) as f32);
                    }
                    step = sign * dy * ((1) as f32);
                    x = (x1 + 1) as i32;
                    while x < x2 {
                        *scanline.offset((x) as isize) += area + step / ((2) as f32);
                        area += (step) as f32;
                        c_runtime::preInc(&mut x);
                    }
                    *scanline.offset((x2) as isize) += area
                        + sign
                            * stbtt__position_trapezoid_area(
                                sy1 - y_final,
                                (x2) as f32,
                                ((x2) as f32) + 1.0f32,
                                x_bottom,
                                ((x2) as f32) + 1.0f32,
                            );
                    *scanline_fill.offset((x2) as isize) += (sign * (sy1 - sy0)) as f32;
                }
            } else {
                let mut x: i32 = 0;
                x = (0) as i32;
                while x < len {
                    let y0: f32 = y_top;
                    let x1: f32 = (x) as f32;
                    let x2: f32 = (x + 1) as f32;
                    let x3: f32 = xb;
                    let y3: f32 = y_bottom;
                    let y1: f32 = (((x) as f32) - x0) / dx + y_top;
                    let y2: f32 = (((x + 1) as f32) - x0) / dx + y_top;
                    if x0 < x1 && x3 > x2 {
                        stbtt__handle_clipped_edge(scanline, x, e, x0, y0, x1, y1);
                        stbtt__handle_clipped_edge(scanline, x, e, x1, y1, x2, y2);
                        stbtt__handle_clipped_edge(scanline, x, e, x2, y2, x3, y3);
                    } else {
                        if x3 < x1 && x0 > x2 {
                            stbtt__handle_clipped_edge(scanline, x, e, x0, y0, x2, y2);
                            stbtt__handle_clipped_edge(scanline, x, e, x2, y2, x1, y1);
                            stbtt__handle_clipped_edge(scanline, x, e, x1, y1, x3, y3);
                        } else {
                            if x0 < x1 && x3 > x1 {
                                stbtt__handle_clipped_edge(scanline, x, e, x0, y0, x1, y1);
                                stbtt__handle_clipped_edge(scanline, x, e, x1, y1, x3, y3);
                            } else {
                                if x3 < x1 && x0 > x1 {
                                    stbtt__handle_clipped_edge(scanline, x, e, x0, y0, x1, y1);
                                    stbtt__handle_clipped_edge(scanline, x, e, x1, y1, x3, y3);
                                } else {
                                    if x0 < x2 && x3 > x2 {
                                        stbtt__handle_clipped_edge(scanline, x, e, x0, y0, x2, y2);
                                        stbtt__handle_clipped_edge(scanline, x, e, x2, y2, x3, y3);
                                    } else {
                                        if x3 < x2 && x0 > x2 {
                                            stbtt__handle_clipped_edge(
                                                scanline, x, e, x0, y0, x2, y2,
                                            );
                                            stbtt__handle_clipped_edge(
                                                scanline, x, e, x2, y2, x3, y3,
                                            );
                                        } else {
                                            stbtt__handle_clipped_edge(
                                                scanline, x, e, x0, y0, x3, y3,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c_runtime::preInc(&mut x);
                }
            }
        }
        e = (*e).next;
    }
}

pub unsafe fn stbtt__handle_clipped_edge(
    scanline: *mut f32,
    x: i32,
    e: *mut stbtt__active_edge,
    mut x0: f32,
    mut y0: f32,
    mut x1: f32,
    mut y1: f32,
) {
    if y0 == y1 {
        return;
    }

    if y0 > (*e).ey {
        return;
    }
    if y1 < (*e).sy {
        return;
    }
    if y0 < (*e).sy {
        x0 += ((x1 - x0) * ((*e).sy - y0) / (y1 - y0)) as f32;
        y0 = ((*e).sy) as f32;
    }
    if y1 > (*e).ey {
        x1 += ((x1 - x0) * ((*e).ey - y1) / (y1 - y0)) as f32;
        y1 = ((*e).ey) as f32;
    }
    if x0 <= ((x) as f32) && x1 <= ((x) as f32) {
        *scanline.offset((x) as isize) += ((*e).direction * (y1 - y0)) as f32;
    } else {
        if x0 >= ((x + 1) as f32) && x1 >= ((x + 1) as f32) {
        } else {
            *scanline.offset((x) as isize) += (*e).direction
                * (y1 - y0)
                * (((1) as f32) - ((x0 - ((x) as f32)) + (x1 - ((x) as f32))) / ((2) as f32));
        }
    }
}

pub unsafe fn stbtt__rasterize(
    result: *mut stbtt__bitmap,
    pts: *mut stbtt__point,
    wcount: *mut i32,
    windings: i32,
    scale_x: f32,
    scale_y: f32,
    shift_x: f32,
    shift_y: f32,
    off_x: i32,
    off_y: i32,
    invert: i32,
    userdata: *const u8,
) {
    let y_scale_inv: f32 = if (invert) != 0 { -scale_y } else { scale_y };
    let mut e: *mut stbtt__edge = core::ptr::null_mut();
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let vsubsample: i32 = 1;
    n = (0) as i32;
    i = (0) as i32;
    while i < windings {
        n += (*wcount.offset((i) as isize)) as i32;
        c_runtime::preInc(&mut i);
    }
    e = (c_runtime::malloc(core::mem::size_of::<stbtt__edge>() as u64 * ((n + 1) as u64)))
        as *mut stbtt__edge;
    if e == core::ptr::null_mut() {
        return;
    }
    n = (0) as i32;
    m = (0) as i32;
    i = (0) as i32;
    while i < windings {
        let p: *mut stbtt__point = (pts).offset((m) as isize);
        m += (*wcount.offset((i) as isize)) as i32;
        j = (*wcount.offset((i) as isize) - 1) as i32;
        k = (0) as i32;
        while k < *wcount.offset((i) as isize) {
            let mut a: i32 = k;
            let mut b: i32 = j;
            if (*p.offset((j) as isize)).y == (*p.offset((k) as isize)).y {
                j = (c_runtime::postInc(&mut k)) as i32;
                continue;
            }
            (*e.offset((n) as isize)).invert = (0) as i32;
            if (invert != 0 && (*p.offset((j) as isize)).y > (*p.offset((k) as isize)).y)
                || (invert == 0 && (*p.offset((j) as isize)).y < (*p.offset((k) as isize)).y)
            {
                (*e.offset((n) as isize)).invert = (1) as i32;
                a = (j) as i32;
                b = (k) as i32;
            }

            (*e.offset((n) as isize)).x0 = ((*p.offset((a) as isize)).x * scale_x + shift_x) as f32;
            (*e.offset((n) as isize)).y0 =
                ((*p.offset((a) as isize)).y * y_scale_inv + shift_y) * ((vsubsample) as f32);
            (*e.offset((n) as isize)).x1 = ((*p.offset((b) as isize)).x * scale_x + shift_x) as f32;
            (*e.offset((n) as isize)).y1 =
                ((*p.offset((b) as isize)).y * y_scale_inv + shift_y) * ((vsubsample) as f32);
            c_runtime::preInc(&mut n);
            j = (c_runtime::postInc(&mut k)) as i32;
        }
        c_runtime::preInc(&mut i);
    }
    stbtt__sort_edges(e, n);
    stbtt__rasterize_sorted_edges(result, e, n, vsubsample, off_x, off_y, userdata);
    c_runtime::free((e) as *mut u8);
}

pub unsafe fn stbtt__rasterize_sorted_edges(
    result: *mut stbtt__bitmap,
    mut e: *mut stbtt__edge,
    n: i32,
    _vsubsample: i32,
    off_x: i32,
    off_y: i32,
    userdata: *const u8,
) {
    let mut hh: stbtt__hheap = Default::default();
    let mut active: *mut stbtt__active_edge = core::ptr::null_mut();
    let mut y: i32 = 0;
    let mut j: i32 = 0;
    let mut i: i32 = 0;
    let mut scanline_data: [f32; 129] = [0.0f32; 129];
    let mut scanline: *mut f32 = core::ptr::null_mut();
    let mut scanline2: *mut f32 = core::ptr::null_mut();

    if (*result).w > 64 {
        scanline = (c_runtime::malloc(
            (((*result).w * 2 + 1) as u64) * core::mem::size_of::<f32>() as u64,
        )) as *mut f32;
    } else {
        scanline = scanline_data.as_mut_ptr();
    }
    scanline2 = (scanline).offset(((*result).w) as isize);
    y = (off_y) as i32;
    (*e.offset((n) as isize)).y0 = ((off_y + (*result).h) as f32) + ((1) as f32);
    while j < (*result).h {
        let scan_y_top: f32 = ((y) as f32) + 0.0f32;
        let scan_y_bottom: f32 = ((y) as f32) + 1.0f32;
        let mut step: *mut *mut stbtt__active_edge = &mut active;
        c_runtime::memset(
            (scanline) as *mut u8,
            0,
            (((*result).w) as u64) * core::mem::size_of::<f32>() as u64,
        );
        c_runtime::memset(
            (scanline2) as *mut u8,
            0,
            (((*result).w + 1) as u64) * core::mem::size_of::<f32>() as u64,
        );
        while (*step) != core::ptr::null_mut() {
            let z: *mut stbtt__active_edge = *step;
            if (*z).ey <= scan_y_top {
                *step = (*z).next;
                (*z).direction = (0) as f32;
                stbtt__hheap_free((&mut hh) as *mut stbtt__hheap, (z) as *mut u8);
            } else {
                step = &mut ((*(*step)).next);
            }
        }
        while (*e).y0 <= scan_y_bottom {
            if (*e).y0 != (*e).y1 {
                let z: *mut stbtt__active_edge = stbtt__new_active(
                    (&mut hh) as *mut stbtt__hheap,
                    e,
                    off_x,
                    scan_y_top,
                    userdata,
                );
                if z != core::ptr::null_mut() {
                    if j == 0 && off_y != 0 {
                        if (*z).ey < scan_y_top {
                            (*z).ey = (scan_y_top) as f32;
                        }
                    }
                    (*z).next = active;
                    active = z;
                }
            }
            c_runtime::preIncPtr(&mut e);
        }
        if (active) != core::ptr::null_mut() {
            stbtt__fill_active_edges_new(
                scanline,
                (scanline2).offset((1) as isize),
                (*result).w,
                active,
                scan_y_top,
            );
        }
        {
            let mut sum: f32 = (0) as f32;
            i = (0) as i32;
            while i < (*result).w {
                let mut k: f32 = 0.0f32;
                let mut m: i32 = 0;
                sum += (*scanline2.offset((i) as isize)) as f32;
                k = (*scanline.offset((i) as isize) + sum) as f32;
                k = c_runtime::fabs(k) * (255 as f32) + 0.5f32;
                m = (k) as i32;
                if m > 255 {
                    m = (255) as i32;
                }
                *(*result).pixels.offset((j * (*result).stride + i) as isize) = (m) as u8;
                c_runtime::preInc(&mut i);
            }
        }
        step = &mut active;
        while (*step) != core::ptr::null_mut() {
            let z: *mut stbtt__active_edge = *step;
            (*z).fx += ((*z).fdx) as f32;
            step = &mut ((*(*step)).next);
        }
        c_runtime::preInc(&mut y);
        c_runtime::preInc(&mut j);
    }
    stbtt__hheap_cleanup((&mut hh) as *mut stbtt__hheap, userdata);
    if scanline != scanline_data.as_mut_ptr() {
        c_runtime::free((scanline) as *mut u8);
    }
}

pub unsafe fn stbtt__sort_edges(p: *mut stbtt__edge, n: i32) {
    stbtt__sort_edges_quicksort(p, n);
    stbtt__sort_edges_ins_sort(p, n);
}

pub unsafe fn stbtt__sort_edges_ins_sort(p: *mut stbtt__edge, n: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = (1) as i32;
    while i < n {
        let mut t: stbtt__edge = *p.offset((i) as isize);
        let a: *mut stbtt__edge = &mut t;
        j = (i) as i32;
        while j > 0 {
            let b: *mut stbtt__edge = &mut *p.offset((j - 1) as isize);
            let c: i32 = if (*(a)).y0 < (*(b)).y0 { 1 } else { 0 };
            if c == 0 {
                break;
            }
            *p.offset((j) as isize) = (*p.offset((j - 1) as isize)) as stbtt__edge;
            c_runtime::preDec(&mut j);
        }
        if i != j {
            *p.offset((j) as isize) = (t) as stbtt__edge;
        }
        c_runtime::preInc(&mut i);
    }
}

pub unsafe fn stbtt__sort_edges_quicksort(mut p: *mut stbtt__edge, mut n: i32) {
    while n > 12 {
        let mut t: stbtt__edge = stbtt__edge::default();
        let mut c01: i32 = 0;
        let mut c12: i32 = 0;
        let mut c: i32 = 0;
        let mut m: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        m = (n >> 1) as i32;
        c01 = (if (&mut *p.offset((0) as isize)).y0 < (&mut *p.offset((m) as isize)).y0 {
            1
        } else {
            0
        }) as i32;
        c12 = (if (&mut *p.offset((m) as isize)).y0 < (&mut *p.offset((n - 1) as isize)).y0 {
            1
        } else {
            0
        }) as i32;
        if c01 != c12 {
            let mut z: i32 = 0;
            c = (if (&mut *p.offset((0) as isize)).y0 < (&mut *p.offset((n - 1) as isize)).y0 {
                1
            } else {
                0
            }) as i32;
            z = (if c == c12 { 0 } else { n - 1 }) as i32;
            t = (*p.offset((z) as isize)) as stbtt__edge;
            *p.offset((z) as isize) = (*p.offset((m) as isize)) as stbtt__edge;
            *p.offset((m) as isize) = (t) as stbtt__edge;
        }
        t = (*p.offset((0) as isize)) as stbtt__edge;
        *p.offset((0) as isize) = (*p.offset((m) as isize)) as stbtt__edge;
        *p.offset((m) as isize) = (t) as stbtt__edge;
        i = (1) as i32;
        j = (n - 1) as i32;
        loop {
            loop {
                if !((&mut *p.offset((i) as isize)).y0 < (&mut *p.offset((0) as isize)).y0) {
                    break;
                }
                c_runtime::preInc(&mut i);
            }
            loop {
                if !((&mut *p.offset((0) as isize)).y0 < (&mut *p.offset((j) as isize)).y0) {
                    break;
                }
                c_runtime::preDec(&mut j);
            }
            if i >= j {
                break;
            }
            t = (*p.offset((i) as isize)) as stbtt__edge;
            *p.offset((i) as isize) = (*p.offset((j) as isize)) as stbtt__edge;
            *p.offset((j) as isize) = (t) as stbtt__edge;
            c_runtime::preInc(&mut i);
            c_runtime::preDec(&mut j);
        }
        if j < (n - i) {
            stbtt__sort_edges_quicksort(p, j);
            p = (p).offset((i) as isize);
            n = (n - i) as i32;
        } else {
            stbtt__sort_edges_quicksort((p).offset((i) as isize), n - i);
            n = (j) as i32;
        }
    }
}

pub unsafe fn stbtt_Rasterize(
    result: *mut stbtt__bitmap,
    flatness_in_pixels: f32,
    vertices: *mut stbtt_vertex,
    num_verts: i32,
    scale_x: f32,
    scale_y: f32,
    shift_x: f32,
    shift_y: f32,
    x_off: i32,
    y_off: i32,
    invert: i32,
    userdata: *const u8,
) {
    let scale: f32 = if scale_x > scale_y { scale_y } else { scale_x };
    let mut winding_count: i32 = 0;
    let mut winding_lengths: *mut i32 = core::ptr::null_mut();
    let windings: *mut stbtt__point = stbtt_FlattenCurves(
        vertices,
        num_verts,
        flatness_in_pixels / scale,
        (&mut winding_lengths) as *mut *mut i32,
        (&mut winding_count) as *mut i32,
        userdata,
    );
    if (windings) != core::ptr::null_mut() {
        stbtt__rasterize(
            result,
            windings,
            winding_lengths,
            winding_count,
            scale_x,
            scale_y,
            shift_x,
            shift_y,
            x_off,
            y_off,
            invert,
            userdata,
        );
        c_runtime::free((winding_lengths) as *mut u8);
        c_runtime::free((windings) as *mut u8);
    }
}
