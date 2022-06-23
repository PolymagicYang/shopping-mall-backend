use actix_web::{web::{self, Data}, post, Responder, Result, get, error, HttpResponse};
use mongodb::{Database, bson::{doc, Document, self}};
use serde::{Serialize, Deserialize};
use shopping_mall_backend::{connection, model::{user_model::User, good_model::{Good, GoodId, Cart}, order_model::Order}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    let db_client = connection::get_connection().await.unwrap();
    let database = db_client.database("shopping");

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(database.clone()))
        .service(login)
        .service(register)
        .service(get_list)
        .service(goods)
        .service(get_cart)
        .service(add2cart)
        .service(search)
        .service(orders)
        .service(save)
        .service(delete)
        .service(deletecart)
        .service(health_check))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

#[get("/health_check")]
async fn health_check() -> Result<impl Responder> {
    Ok(HttpResponse::Ok()) 
}

#[post("/add2cart")]
async fn add2cart(db: web::Data<Database>, form: web::Form<GoodId>) -> Result<impl Responder> {
    let id = &form.goodId;
    let collection = db.get_ref().collection::<Good>("Goods");
    
    let r = collection.find_one(doc! {
        "goodId": id 
    }, None).await;

    let r = match r {
        Ok(r) => r,
        Err(e) => return Err(error::ErrorBadRequest(e)),
    };
    let good = match r {
        Some(u) => u,
        None => return Err(error::ErrorBadRequest("No such a good.")),
    };
    
    // check existence first.
    let cart_coll = db.get_ref() .collection::<Cart>("Cart");
    let cart_id = good.goodId;
    let find = cart_coll.find_one(
        doc! {
            "goodId": cart_id.clone(),
        },
        None,
    ).await;
   
    let find_result = match find {
        Ok(r) => r,
        Err(_) => return Err(error::ErrorBadRequest("No such a result.")),
    };
    let cart = match find_result {
        Some(r) => r,
        None => {
            match cart_coll.insert_one(
                Cart {
                    goodId: cart_id,
                    goodImage: good.goodImage,
                    goodName: good.goodName,
                    goodValue: good.goodValue,
                    goodNumber: String::from("1"),
                    isSelected: false,
                },
                None
            ).await {
                Ok(_) => return Ok(web::Json(LoginStruct { flag: true })),
                Err(_) => return Err(error::ErrorExpectationFailed("Failed to insert the new good.")),
            } 
        },
    };
    let mut current_num = str::parse::<u64>(&cart.goodNumber).unwrap();
    current_num += 1;
    let cart = doc! {
        "$set": { "goodNumber": format!("{}", current_num) }
    };
    
    let result = cart_coll.update_one(doc! {
        "goodId": cart_id,
    }, cart, None).await;
    
    match result {
        Ok(_) => {
            Ok(web::Json(LoginStruct { flag: true }))
        },
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}

#[post("/cart")]
async fn get_cart(db: web::Data<Database>) -> Result<impl Responder> {
    let collection = db.get_ref().collection::<Cart>("Cart");
    let r = collection.find(None, None).await;
    let mut cursor = match r {
        Ok(r) => r,
        Err(_) => return Err(error::ErrorBadRequest("Failed to connect to the database.")),
    };
    let mut cart = vec![];
    while cursor.advance().await.unwrap() {
        let doc = cursor.current();

        cart.push(Cart {
            goodId: String::from(doc.get_str("goodId").unwrap()),
            goodName: String::from(doc.get_str("goodName").unwrap()),
            goodImage: String::from(doc.get_str("goodImage").unwrap()),
            goodValue: String::from(doc.get_str("goodValue").unwrap()),
            goodNumber: String::from(doc.get_str("goodNumber").unwrap()),
            isSelected: doc.get_bool("isSelected").unwrap(),
        });
    }
    let json = web::Json(cart);
    Ok(json)
    
}

#[derive(Serialize, Deserialize, Debug)]
struct deleteForm {
    orderId: String, 
}
#[derive(Serialize, Deserialize, Debug)]
struct cartForm {
    goodId: String, 
}

#[post("/deletecart")]
async fn deletecart(db: web::Data<Database>, form: web::Form<cartForm>) -> Result<impl Responder> {
    let collection = db.get_ref().collection::<Cart>("Cart");
    let r = collection.delete_one(doc! {
        "goodId": form.goodId.clone()
    }, None).await;
    
    match r {
        Ok(_) => return Ok(web::Json(LoginStruct { flag: true })),
        Err(e) => return Err(error::ErrorBadRequest(e))
    }
}


#[post("/delete")]
async fn delete(db: web::Data<Database>, form: web::Form<deleteForm>) -> Result<impl Responder> {
    let collection = db.get_ref().collection::<Order>("Orders");
    let r = collection.delete_one(doc! {
        "orderId": form.orderId.clone()
    }, None).await;
    
    match r {
        Ok(_) => return Ok(web::Json(LoginStruct { flag: true })),
        Err(e) => return Err(error::ErrorBadRequest(e))
    }
}

#[post("/orders")]
async fn orders(db: web::Data<Database>) -> Result<impl Responder> {
    let collection = db.get_ref().collection::<Order>("Orders");
    let r = collection.find(None, None).await;
    let mut cursor = match r {
        Ok(r) => r,
        Err(_) => return Err(error::ErrorBadRequest("Failed to connect to the database.")),
    };
    let mut orders = vec![];
    while cursor.advance().await.unwrap() {
        let doc = cursor.current();

        orders.push(Order {
            orderId: Some(String::from(doc.get_str("orderId").unwrap())),
            orderMoney: String::from(doc.get_str("orderMoney").unwrap()),
            orderState: String::from(doc.get_str("orderState").unwrap()),
            address: String::from(doc.get_str("address").unwrap()),
            phone: String::from(doc.get_str("phone").unwrap()),
            receiver: String::from(doc.get_str("receiver").unwrap()),
        });
    }
    let json = web::Json(orders);
    Ok(json)
}

#[post("/goods")]
async fn goods(db: web::Data<Database>) -> Result<impl Responder> {
    let collection = db.get_ref().collection::<Good>("Goods");
    let r = collection.find(None, None).await;
    let mut cursor = match r {
        Ok(r) => r,
        Err(_) => return Err(error::ErrorBadRequest("Failed to connect to the database.")),
    };
    let mut goods = vec![];
    while cursor.advance().await.unwrap() {
        let doc = cursor.current();

        goods.push(Good {
            goodId: String::from(doc.get_str("goodId").unwrap()),
            goodName: String::from(doc.get_str("goodName").unwrap()),
            goodIntroduction: String::from(doc.get_str("goodIntroduction").unwrap()),
            goodImage: String::from(doc.get_str("goodImage").unwrap()),
            goodValue: String::from(doc.get_str("goodValue").unwrap()),
        });
    }
    let json = web::Json(goods);
    Ok(json)
}

#[post("/login")]
async fn login(db: web::Data<Database>, user: web::Form<User>) -> Result<impl Responder> {
    let mut login_result = LoginStruct { flag: false };
    let conn = db.get_ref();
    let users = conn.collection::<User>("User");
    let username = &user.name;
    let password = &user.password;
    let r = users.find_one(doc! {
        "name": username
    }, None).await;
    let r = match r {
        Ok(r) => r,
        Err(_) => return Err(error::ErrorBadRequest("Failed to connect to the database.")),
    };
    let user = match r {
        Some(u) => u,
        None => return Ok(web::Json(login_result)),
    };
    if user.password.eq(password) {
        login_result = LoginStruct { flag: true };
    }
    Ok(web::Json(login_result))
}

#[post("/register")]
async fn register(db: web::Data<Database>, user: web::Form<User>) -> Result<impl Responder> {
    let users = db.collection::<User>("User");
    let user = User {
        name: user.name.clone(),
        password: user.password.clone(),
    };
   
    let username = &user.name;
    let r = users.find_one(doc! {
        "name": username
    }, None).await;
    let r = match r {
        Ok(r) => r,
        Err(_) => return Err(error::ErrorBadRequest("Failed to connect to the database.")),
    };
    match r {
        Some(_) => { 
            let reg_result = LoginStruct { flag: false };
            return Ok(web::Json(reg_result))
        },
        None => (),
    };

    let users_vec = vec![user];
    match users.insert_many(users_vec, None).await {
        Ok(user) => { user },
        Err(_) =>  { return Err(error::ErrorBadRequest("can not register right now.")) }
    };
    let reg_result = LoginStruct { flag: true };

    Ok(web::Json(reg_result))
}

#[get("/list_collections")]
async fn get_list(db: web::Data<Database>) -> Result<impl Responder> {
    let mut list = vec![];
    for collection_name in db.list_collection_names(None).await.unwrap() {
        list.push(collection_name)
    }
    Ok(web::Json(list))
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderForm {
    orderId: String,
    receiver: String, 
    address: String,
    phone: String,
    price: String
}

#[post("/saveorder")]
async fn save(db: web::Data<Database>, form: web::Form<OrderForm>) -> Result<impl Responder> {
    let db = db.get_ref();
    let collection = db.collection::<Order>("Orders");
    let order = Order {
        orderId: Some(form.orderId.clone()),
        receiver: form.receiver.clone(),
        address: form.address.clone(),
        phone: form.phone.clone(),
        orderMoney: form.price.clone(),
        orderState: String::from("test")
    };
    let r = collection.insert_one(order, None).await;
    match r {
        Ok(_) => Ok(web::Json(LoginStruct { flag: true } )),
        Err(_) => Err(error::ErrorBadRequest("Failed to add order.")),
    }
}

#[post("/search")]
async fn search(db: web::Data<Database>, form: web::Form<SearchStruct>) -> Result<impl Responder> {
    let keyword = form.inputSome.clone();
    
    let re = mongodb::bson::Regex {
        pattern: String::from(keyword),
        options: String::new(),
    };
    
    let collection = db.get_ref().collection::<Good>("Goods");
    let filter = doc! {
        "goodName": re
    };

    let r = collection.find(filter, None).await;
    let mut cursor = match r {
        Ok(r) => r,
        Err(_) => return Err(error::ErrorBadRequest("Failed to connect to the database.")),
    };

    let mut result = vec![];
    while cursor.advance().await.unwrap() {
        let doc = cursor.current();

        result.push(Good {
            goodId: String::from(doc.get_str("goodId").unwrap()),
            goodName: String::from(doc.get_str("goodName").unwrap()),
            goodIntroduction: String::from(doc.get_str("goodIntroduction").unwrap()),
            goodImage: String::from(doc.get_str("goodImage").unwrap()),
            goodValue: String::from(doc.get_str("goodValue").unwrap()),
        });
    }
    let json = web::Json(result);
    Ok(json)
}

#[derive(Serialize)]
struct LoginStruct {
    flag: bool
}

#[derive(Serialize, Deserialize)]
struct SearchStruct {
    inputSome: String, 
}