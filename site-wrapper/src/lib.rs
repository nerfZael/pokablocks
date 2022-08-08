mod wrap;
use wrap::{*, imported::ArgsGetFile};

pub fn index(_: ArgsIndex) -> Option<Response> {
    let index_html = WrapClientModule::get_file(&ArgsGetFile{
        uri: "wrap://ens/site.pokablocks.eth".to_string(),
        file_path: "site/index.html".to_string(),
    });

    match index_html {
        Ok(index_html) => {
            match index_html {
                Some(index_html) => {
                    Some(Response {
                        data: Some(index_html),
                        headers: Some(vec![
                            Header {
                                name: "Content-Type".to_string(),
                                value: "text/html".to_string(),
                            },
                        ]),
                    })
                }
                None => {
                    None
                }
            }
        }
        Err(_) => {
            None
        }
    }
}
