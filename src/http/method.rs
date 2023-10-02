mod method {
    //Rust enums each member can have different type 
    pub enum Method {
        // GET(String), //= 0
        // DELETE(u64), //=1
        GET,
        DELETE,
        POST, //=2  
        PUT,//=7 rest would increment from here 8-9-10...
        HEAD,
        CONNECT,
        OPTIONS,
        TRACE,
        PATCH
    }
    /*GET /user?id=10 GTTP/1.1\r\n
    HEADERS \r\n
    BODY
    */
}