fn main() {
    unsafe {
        let handle = curl_sys::curl_easy_init();
        if handle.is_null() {
            panic!("curl_easy_init returned null");
        }
        curl_sys::curl_easy_cleanup(handle);
    }
}
