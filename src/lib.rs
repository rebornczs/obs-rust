use chrono::{DateTime, Utc};
use hmacsha1::hmac_sha1;
use reqwest::header::HeaderMap;

pub struct ObsClient {
    ak: String,
    sk: String,
    endpoint: String,
    bucket_name: String
}

impl ObsClient {
    fn get_time() -> String {
        let datetime: DateTime<Utc> = std::time::SystemTime::now().into();
        datetime.to_rfc2822()
    }

    fn get_signature(&self, str: String) -> String {
        base64::encode(hmac_sha1(self.sk.as_bytes(), str.as_bytes()))
    }

    pub fn upload(&self, source_path: String, target_path: String) {
        let client = reqwest::blocking::Client::new();

        let content_md5 = String::from("");
        let content_type = String::from("");
        let canonicalized_headers = String::from("");
        let canonicalized_resource = format!("/{}/{}", self.bucket_name, target_path);
        let request_time = Self::get_time();

        let url = format!("http://{}.{}/{}", self.bucket_name, self.endpoint, target_path);
        println!("url: {}", url);

        let canonical_string = format!("PUT\n{}\n{}\n{}\n{}{}", content_md5, content_type, request_time, canonicalized_headers, canonicalized_resource);
        println!("cano: \n{}", &canonical_string);

        let mut headers = HeaderMap::new();
        headers.insert("Date", request_time.parse().unwrap());

        let signature = Self::get_signature(&self, canonical_string);
        headers.insert("Authorization", format!("OBS {}:{}", self.ak, signature).to_string().parse().unwrap());
        println!("{:?}", headers);

        let upload_file = std::fs::File::open(source_path).unwrap();
        let response = client.put(url).headers(headers).body("str").send().unwrap().status();
        println!("{}", response);
    }
}

#[cfg(test)]
mod tests {
    use crate::ObsClient;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn put() {
        let obs_client = ObsClient {
            ak: String::from("IKLRILARMLSEO9WCTNTM"),
            sk: String::from("j3lDgcQvbd7gtOnI3ZK74s2vtg9dFy6ocj7StZ1Z"),
            endpoint: String::from("obs.cn-north-4.myhuaweicloud.com"),
            bucket_name: String::from("octopus-czs")
        };
        let source_str = String::from("test.txt");
        let target_str = String::from("test.txt");
        obs_client.upload(source_str, target_str);
    }
}
