use cli::pointcloud_conversion::convert_laz_to_ply;

fn main() {
    match convert_laz_to_ply("/Users/hectorcrean/Downloads/lidar_point_cloud-2015-NaN-TQ38sw/TQ3080_P_9983_20150206_20150206.laz", "/Users/hectorcrean/rust/crayon/app/assets/gcloud/output.ply") {
        Ok(_) => println!("Conversion successful!"),
        Err(e) => eprintln!("Error during conversion: {:?}", e),
    }
}