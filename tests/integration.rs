#[macro_use]
mod support;
use self::support::*;

t! {
    get_1,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            ;
    server:
        request:
            uri: "/",
            ;
        response:
}

t! {
    get_implicit_path,
    client:
        request:
            uri: "",
            ;
        response:
            status: 200,
            ;
    server:
        request:
            uri: "/",
            ;
        response:
}

t! {
    get_body,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            headers: {
                "content-length" => 11,
            },
            body: "hello world",
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            headers: {
                "content-length" => 11,
            },
            body: "hello world",
}

t! {
    get_body_chunked,
    client:
        request:
            uri: "/",
            ;
        response:
            status: 200,
            headers: {
                // h2 doesn't actually receive the transfer-encoding header
            },
            body: "hello world",
            ;
    server:
        request:
            uri: "/",
            ;
        response:
            headers: {
                // http2 should strip this header
                "transfer-encoding" => "chunked",
            },
            body: "hello world",
}

t! {
    post_chunked,
    client:
        request:
            method: "POST",
            uri: "/post_chunked",
            headers: {
                // http2 should strip this header
                "transfer-encoding" => "chunked",
            },
            body: "hello world",
            ;
        response:
            ;
    server:
        request:
            method: "POST",
            uri: "/post_chunked",
            body: "hello world",
            ;
        response:
}
