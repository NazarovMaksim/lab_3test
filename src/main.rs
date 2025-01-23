use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
struct Room {
    id: u32,
    name: String,
    available: bool,
    owner_info: OwnerInfo,
}

#[derive(Serialize, Deserialize, Clone)]
struct OwnerInfo {
    contact_name: String,
    email: String,
    phone_number: String,
}

#[derive(Serialize, Deserialize)]
struct BookingRequest {
    room_id: u32,
}

#[derive(Serialize, Deserialize)]
struct CancelBookingRequest {
    room_id: u32,
}

#[derive(Serialize, Deserialize)]
struct GetRoomRequest {
    room_id: u32,
}

#[derive(Serialize, Deserialize)]
struct UpdateRoomStatusRequest {
    room_id: u32,
    available: bool,
}

#[derive(Serialize, Deserialize)]
struct BookingResponse {
    success: bool,
    message: String,
}

type Rooms = Arc<Mutex<HashMap<u32, Room>>>;

async fn get_rooms(data: web::Data<Rooms>) -> impl Responder {
    let rooms = data.lock().unwrap();
    HttpResponse::Ok().json(&*rooms)
}

async fn book_room(data: web::Data<Rooms>, req: web::Json<BookingRequest>) -> impl Responder {
    let mut rooms = data.lock().unwrap();
    if let Some(room) = rooms.get_mut(&req.room_id) {
        if room.available {
            room.available = false;
            HttpResponse::Ok().json(BookingResponse {
                success: true,
                message: format!("Room {} booked successfully!", room.id),
            })
        } else {
            HttpResponse::Conflict().json(BookingResponse {
                success: false,
                message: format!("Room {} is not available.", room.id),
            })
        }
    } else {
        HttpResponse::NotFound().json(BookingResponse {
            success: false,
            message: format!("Room {} not found.", req.room_id),
        })
    }
}

async fn cancel_booking(data: web::Data<Rooms>, req: web::Json<CancelBookingRequest>) -> impl Responder {
    let mut rooms = data.lock().unwrap();
    if let Some(room) = rooms.get_mut(&req.room_id) {
        if !room.available {
            room.available = true;
            HttpResponse::Ok().json(BookingResponse {
                success: true,
                message: format!("Room {} booking canceled successfully!", room.id),
            })
        } else {
            HttpResponse::Conflict().json(BookingResponse {
                success: false,
                message: format!("Room {} is already available.", room.id),
            })
        }
    } else {
        HttpResponse::NotFound().json(BookingResponse {
            success: false,
            message: format!("Room {} not found.", req.room_id),
        })
    }
}

async fn get_room(data: web::Data<Rooms>, req: web::Json<GetRoomRequest>) -> impl Responder {
    let rooms = data.lock().unwrap();
    if let Some(room) = rooms.get(&req.room_id) {
        HttpResponse::Ok().json(room)
    } else {
        HttpResponse::NotFound().json(BookingResponse {
            success: false,
            message: format!("Room {} not found.", req.room_id),
        })
    }
}

async fn update_room_status(data: web::Data<Rooms>, req: web::Json<UpdateRoomStatusRequest>) -> impl Responder {
    let mut rooms = data.lock().unwrap();
    if let Some(room) = rooms.get_mut(&req.room_id) {
        room.available = req.available;
        HttpResponse::Ok().json(BookingResponse {
            success: true,
            message: format!("Room {} status updated successfully!", room.id),
        })
    } else {
        HttpResponse::NotFound().json(BookingResponse {
            success: false,
            message: format!("Room {} not found.", req.room_id),
        })
    }
}

async fn get_owner_info(data: web::Data<Rooms>, req: web::Json<GetRoomRequest>) -> impl Responder {
    let rooms = data.lock().unwrap();
    if let Some(room) = rooms.get(&req.room_id) {
        HttpResponse::Ok().json(room.owner_info.clone())
    } else {
        HttpResponse::NotFound().json(BookingResponse {
            success: false,
            message: format!("Room {} not found.", req.room_id),
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rooms = Arc::new(Mutex::new(HashMap::new()));

    // Инициализация данных помещений
    let room_data = vec![
        Room {
            id: 1,
            name: "Room 1".to_string(),
            available: true,
            owner_info: OwnerInfo {
                contact_name: "Alexey Vladimirovich".to_string(),
                email: "Alex.vlad@mail.ru".to_string(),
                phone_number: "+8-800-5555".to_string(),
            },
        },
        Room {
            id: 2,
            name: "Room 2".to_string(),
            available: true,
            owner_info: OwnerInfo {
                contact_name: "Ivan Vasilyevich".to_string(),
                email: "ivan.vas@mail.ru".to_string(),
                phone_number: "+8-464-2345".to_string(),
            },
        },
        Room {
            id: 3,
            name: "Room 3".to_string(),
            available: true,
            owner_info: OwnerInfo {
                contact_name: "Dima Fedorovich".to_string(),
                email: "Dima.dvfu@gmail.com".to_string(),
                phone_number: "+8-565-5212".to_string(),
            },
        },
    ];

    for room in room_data {
        rooms.lock().unwrap().insert(room.id, room);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(rooms.clone()))
            .route("/rooms", web::get().to(get_rooms))
            .route("/book", web::post().to(book_room))
            .route("/cancel_book", web::post().to(cancel_booking))
            .route("/room", web::post().to(get_room))
            .route("/update_status", web::post().to(update_room_status))
            .route("/owner_info", web::post().to(get_owner_info))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
