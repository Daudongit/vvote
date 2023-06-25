/// A generic method for rendering an error to present to the browser.
/// This should only be called in non-production settings.
#[cfg(not(feature = "production"))]
pub(crate) fn render<E: std::fmt::Debug>(e: E) -> String {
    format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no, maximum-scale=1.0">
            <title>Jelly: An Error Occurred</title>
            <style>
                html, body {{
                    margin: 0;
                    padding: 0;
                    background: #F0DEE0;
                    color: #111;
                    font-family: -apple-system, "Helvetica Neue", Helvetica, "Segoe UI", Ubuntu, arial, sans-serif;
                }}
                
                h1 {{ margin: 0; background: #F05758; border-bottom: 1px solid #C7484A; padding: 20px; font-size: 30px; font-weight: 600; line-height: 40px; }}
                
                code {{
                    display: block;
                    font-family: "Anonymous Pro", Consolas, Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace, serif; 
                    font-size: 16px;
                    line-height: 20px;
                    padding: 20px;
                }}
            </style>
        </head>
        <body>
            <h1>Error</h1>
            <code>{:#?}<code>
        </body>
        </html>
    "#,
        e
    )
}

/// A generic method for rendering an error to present to the browser.
#[cfg(feature = "production")]
pub(crate) fn render<E: std::fmt::Debug>(_e: E) -> String {
    format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <!-- The above 3 meta tags *must* come first in the head; any other head content must come *after* these tags -->
        
                <title> 500 </title>
        
                <!-- Google font -->
                <link href="https://fonts.googleapis.com/css?family=Josefin+Sans:400,700" rel="stylesheet"> 
        
                <!-- Custom stlylesheet -->
                <style>
                    * {{
                        -webkit-box-sizing: border-box;
                                box-sizing: border-box;
                    }}
                    
                    body {{
                        padding: 0;
                        margin: 0;
                    }}
                    
                    #notfound {{
                        position: relative;
                        height: 100vh;
                        background-color: #222;
                    }}
                    
                    #notfound .notfound {{
                        position: absolute;
                        left: 50%;
                        top: 50%;
                        -webkit-transform: translate(-50%, -50%);
                            -ms-transform: translate(-50%, -50%);
                                transform: translate(-50%, -50%);
                    }}
                    
                    .notfound {{
                        max-width: 460px;
                        width: 100%;
                        text-align: center;
                        line-height: 1.4;
                    }}
                    
                    .notfound .notfound-404 {{
                        height: 158px;
                        line-height: 153px;
                    }}
                    
                    .notfound .notfound-404 h1 {{
                        font-family: 'Josefin Sans', sans-serif;
                        color: #222;
                        font-size: 220px;
                        letter-spacing: 10px;
                        margin: 0px;
                        font-weight: 700;
                        text-shadow: 2px 2px 0px #c9c9c9, -2px -2px 0px #c9c9c9;
                    }}
                    
                    .notfound .notfound-404 h1>span {{
                        text-shadow: 2px 2px 0px #ffab00, -2px -2px 0px #ffab00, 0px 0px 8px #ff8700;
                    }}
                    
                    .notfound p {{
                        font-family: 'Josefin Sans', sans-serif;
                        color: #c9c9c9;
                        font-size: 16px;
                        font-weight: 400;
                        margin-top: 0px;
                        margin-bottom: 15px;
                    }}
                    
                    .notfound a {{
                        font-family: 'Josefin Sans', sans-serif;
                        font-size: 14px;
                        text-decoration: none;
                        text-transform: uppercase;
                        background: transparent;
                        color: #c9c9c9;
                        border: 2px solid #c9c9c9;
                        display: inline-block;
                        padding: 10px 25px;
                        font-weight: 700;
                        -webkit-transition: 0.2s all;
                        transition: 0.2s all;
                    }}
                    
                    .notfound a:hover {{
                        color: #ffab00;
                        border-color: #ffab00;
                    }}
                    
                    @media only screen and (max-width: 480px) {{
                        .notfound .notfound-404 {{
                            height: 122px;
                            line-height: 122px;
                        }}
                    
                        .notfound .notfound-404 h1 {{
                            font-size: 122px;
                        }}
                    }}
                </style>
            </head>
            <body>
                
            <div id="notfound">
                <div class="notfound">
                    <div class="notfound-404">
                        <h1>5<span>0</span>0</h1>
                    </div>
                    <p>We are very sorry about this please contact the admin for more infomation</p>
                    <a href="/">home page</a>
                </div>
            </div>	
            </body><!-- This templates was made by Colorlib (https://colorlib.com) -->
        </html>
        "#
    )
}
