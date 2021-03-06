use crate::server::*;

#[derive(Clone, Debug)]
pub struct request {
    pub req_type: String,
    pub key: String,
    pub value: String,
    pub present_root_hash: String,
    pub deleted_root_hash: String,
}

#[derive(Clone, Debug)]
pub struct response {
    pub rsp_status: bool,
    pub req_type: String,
    pub data: String,
    pub error_info: String,
}

pub fn client_test(server: &mut server) {
    initial_data(server);
    get_data(server);
    modify_data(server);
}

pub fn initial_data(server: &mut server) {
    let req = request {
        req_type: "put".to_string(),
        key: "db".to_string(),
        value: "proxy".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("test{:?}", rsp);
}

pub fn get_data(server: &mut server) {
    //try to get data
    let req = request {
        req_type: "get".to_string(),
        key: "db".to_string(),
        value: "".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("test{:?}", rsp);
}

pub fn modify_data(server: &mut server) {
    //try to insert data failed
    let req = request {
        req_type: "insert".to_string(),
        key: "db".to_string(),
        value: "proxy1".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("{:?}", rsp);

    let req = request {
        req_type: "get".to_string(),
        key: "db".to_string(),
        value: "".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("test{:?}", rsp);

    //try to delete data
    let req = request {
        req_type: "delete".to_string(),
        key: "db".to_string(),
        value: "".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("test{:?}", rsp);
    //
    //try to get the deleted data
    let req = request {
        req_type: "get".to_string(),
        key: "db".to_string(),
        value: "".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("test{:?}", rsp);
    //
    //try to insert data ,success
    let req = request {
        req_type: "insert".to_string(),
        key: "db".to_string(),
        value: "proxy1".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("{:?}", rsp);
    //
    //try to get the inserted data
    let req = request {
        req_type: "get".to_string(),
        key: "db".to_string(),
        value: "".to_string(),
        present_root_hash: "".to_string(),
        deleted_root_hash: "".to_string(),
    };
    let rsp = server.handle_req(req);
    println!("{:?}", rsp);
}

pub fn send_req(server: &mut server, req: request) -> response {
    server.handle_req(req)
}
