use utoipa::openapi::{
    path::Operation, Content, HttpMethod, Info, License, OpenApiBuilder, PathItem, Paths, Response,
};

fn main() {
    let open_api = OpenApiBuilder::new()
        .info(
            Info::builder()
                .license(Some(License::new("GPL 2")))
                .title("This is my API")
                .version("1.0"),
        )
        .paths(
            Paths::builder().path(
                "/hello",
                PathItem::builder()
                    .summary(Some("My Hello endpoint"))
                    .operation(
                        HttpMethod::Get,
                        Operation::builder()
                            .response(
                                "200",
                                Response::builder()
                                    .description("'Hello' string")
                                    .content("text/plain", Content::builder().build()),
                            )
                            .build(),
                    )
                    .build(),
            ),
        )
        .build();
    println!("{}", open_api.to_pretty_json().unwrap());
}
