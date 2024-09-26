use actix_web::Responder;

pub async fn demo_login() -> impl Responder {
    r#"{
    "code": 0,
    "data": {
        "id": 0,
        "password": "123456",
        "realName": "Vben",
        "roles": [
            "super"
        ],
        "username": "vben",
        "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6MCwicGFzc3dvcmQiOiIxMjM0NTYiLCJyZWFsTmFtZSI6IlZiZW4iLCJyb2xlcyI6WyJzdXBlciJdLCJ1c2VybmFtZSI6InZiZW4iLCJpYXQiOjE3MjczMzI4NTIsImV4cCI6MTcyNzkzNzY1Mn0.qwLILs9_v6QQKZrRzOeq_YayocYw7NVs-aUkcvM9HOo"
    },
    "error": null,
    "message": "ok"
}"#
}

pub async fn demo_user_info() -> impl Responder {
    r#"
    {
    "code": 0,
    "data": {
        "id": 0,
        "realName": "Vben",
        "roles": [
            "super"
        ],
        "username": "vben"
    },
    "error": null,
    "message": "ok"
}"#
}

pub async fn demo_code() -> impl Responder {
    r#"{
    "code": 0,
    "data": [
        "AC_100100",
        "AC_100110",
        "AC_100120",
        "AC_100010"
    ],
    "error": null,
    "message": "ok"
}"#
}
