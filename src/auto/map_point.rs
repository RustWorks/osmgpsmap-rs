// This file was generated by gir (https://github.com/gtk-rs/gir @ 45f834b+)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use glib::translate::*;
use osm_gps_map_sys;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MapPoint(Boxed<osm_gps_map_sys::OsmGpsMapPoint>);

    match fn {
        copy => |ptr| osm_gps_map_sys::osm_gps_map_point_copy(mut_override(ptr)),
        free => |ptr| osm_gps_map_sys::osm_gps_map_point_free(ptr),
        get_type => || osm_gps_map_sys::osm_gps_map_point_get_type(),
    }
}

impl MapPoint {
    pub fn new_degrees(lat: f32, lon: f32) -> MapPoint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(osm_gps_map_sys::osm_gps_map_point_new_degrees(lat, lon))
        }
    }

    pub fn new_radians(rlat: f32, rlon: f32) -> MapPoint {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(osm_gps_map_sys::osm_gps_map_point_new_radians(rlat, rlon))
        }
    }

    pub fn get_degrees(&mut self) -> (f32, f32) {
        unsafe {
            let mut lat = mem::MaybeUninit::uninit();
            let mut lon = mem::MaybeUninit::uninit();
            osm_gps_map_sys::osm_gps_map_point_get_degrees(self.to_glib_none_mut().0, lat.as_mut_ptr(), lon.as_mut_ptr());
            let lat = lat.assume_init();
            let lon = lon.assume_init();
            (lat, lon)
        }
    }

    pub fn get_radians(&mut self) -> (f32, f32) {
        unsafe {
            let mut rlat = mem::MaybeUninit::uninit();
            let mut rlon = mem::MaybeUninit::uninit();
            osm_gps_map_sys::osm_gps_map_point_get_radians(self.to_glib_none_mut().0, rlat.as_mut_ptr(), rlon.as_mut_ptr());
            let rlat = rlat.assume_init();
            let rlon = rlon.assume_init();
            (rlat, rlon)
        }
    }

    pub fn set_degrees(&mut self, lat: f32, lon: f32) {
        unsafe {
            osm_gps_map_sys::osm_gps_map_point_set_degrees(self.to_glib_none_mut().0, lat, lon);
        }
    }

    pub fn set_radians(&mut self, rlat: f32, rlon: f32) {
        unsafe {
            osm_gps_map_sys::osm_gps_map_point_set_radians(self.to_glib_none_mut().0, rlat, rlon);
        }
    }
}
