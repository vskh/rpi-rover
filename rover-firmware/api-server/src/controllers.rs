use actix_web::HttpRequest;

pub fn move_forward(_req: &HttpRequest) -> &'static str {
    "Moving with speed"
}

pub fn move_backward(_req: &HttpRequest) -> &'static str {
    "Moving with speed"
}

pub fn spin_left(_req: &HttpRequest) -> &'static str {
    "Spinning in direction"
}

pub fn spin_right(_req: &HttpRequest) -> &'static str {
    "Spinning in direction"
}

pub fn look(_req: &HttpRequest) -> &'static str {
    "Looking at coordinates"
}

pub fn get_distance(_req: &HttpRequest) -> &'static str {
    "Distance to obstacle"
}

pub fn get_obstacles(_req: &HttpRequest) -> &'static str {
    "L R obstacles"
}

pub fn get_lines(_req: &HttpRequest) -> &'static str {
    "L R line"
}
