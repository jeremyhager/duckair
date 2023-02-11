fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let kcle_lat_degrees: f64 = 41.4075;
    let kcle_long_degrees: f64 = -81.851111;

    let kslc_lat_degrees: f64 = 40.7861;
    let kslc_long_degrees: f64 = -111.9822;

    let kcle_lat_radians = kcle_lat_degrees.to_radians();
    let kslc_lat_radians = kslc_lat_degrees.to_radians();

    let delta_lat = (kcle_lat_degrees - kslc_lat_degrees).to_radians();
    let delta_long = (kcle_long_degrees - kslc_long_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_lat/ 2.0).sin(), 2)
        + kcle_lat_radians.cos()
        * kslc_lat_radians.cos()
        * f64::powi((delta_long/ 2.0).sin(),2);
    let central_angle = 2.0 * inner_central_angle.sqrt().asin();


    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    println!("The distance between the two points is {:.1} kilometers", distance);
}
