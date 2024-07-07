use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

fn main() {
    // let email = Message::builder()
    //     .from("Cork <g1024536444@gmail.com>".parse().unwrap()) // 你的Gmail账户作为发件人
    //     .reply_to("Cork <g1024536444@gmail.com>".parse().unwrap()) // 你的Gmail账户作为回复地址
    //     .to("Recipient <1669387225@qq.com>".parse().unwrap()) // 目标邮箱作为收件人
    //     .subject("Test Email") // 邮件标题
    //     .header(ContentType::TEXT_PLAIN) // 邮件内容类型为纯文本
    //     .body(String::from("Hello World!")) // 邮件正文
    //     .unwrap();

    let email = Message::builder()
        .from("1024536444@qq.com".parse().unwrap()) // 你的Gmail账户作为发件人
        .reply_to("1024536444@qq.com".parse().unwrap()) // 你的Gmail账户作为回复地址
        .to("1669387225@qq.com".parse().unwrap()) // 目标邮箱作为收件人
        .subject("Test Email") // 邮件标题
        .header(ContentType::TEXT_PLAIN) // 邮件内容类型为纯文本
        .body(String::from("Hello World!")) // 邮件正文
        .unwrap();

    // let creds = Credentials::new("".to_owned(), "".to_owned()); // 使用你的Gmail账户名和密码
    let creds = Credentials::new("".to_owned(), "".to_owned());

    // // 使用Gmail的SMTP服务器
    // let mailer = SmtpTransport::relay("smtp.gmail.com")
    //     .unwrap()
    //     .credentials(creds)
    //     .build();
    // let mailer = SmtpTransport::unencrypted_localhost();\

    let mailer = SmtpTransport::relay("smtp.qq.com")
            .unwrap()
            .credentials(creds)
            .build();

    // 发送邮件
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {}", e),
    }
}
