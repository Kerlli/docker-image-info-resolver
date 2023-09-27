use docker_image_info_resolver::image::Image;

fn main() {
    let mut image = Image::try_from_tar("resources/docker-images/test.tar");
    if let Ok(docker_image) = &mut image {
        let json = docker_image.json().expect("serialize error");
        println!("Json: {}", json);
    } else {
        panic!("{:?}", image.err());
    }
}
