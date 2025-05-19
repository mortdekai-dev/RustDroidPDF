use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jint, jstring};
use lopdf::{Document, Result as LopdResult};
use std::path::path;

struct PdfDocument {
    doc: Document
}

fn jstring_to_rstring(env: &JNIEnv, java_string: JString) -> String {
    env.get_string(java_string)
    .expect("Could not get java string")
    .into()
}

 #[no_mangle]
pub extern "system" fn Java_com_mortdekai_rustdroidpdf_PdfManager_openPdfDocument(
    env: JNIEnv,
    _class: JClass,
    file_path_jstring: JString
) -> jlong {

    let file_path_rstring: String = jstring_to_rstring(&env, file_path)
    let path = Path::new(&file_path_rstring)  

    match Document::load(path) {
    Ok(doc) => {
        let handle = Box::new(PdfDocument {doc})
        Box::into_raw(handle) as jlong
        }
    Err(e) => {
        eprintln!("Failed to load pdf: {:?}", e)
        0
        }
    } 
}


#[no_mangle]
pub extern "system" fn Java_com_mortdekai_rustdroidpdf_PdfManager_getPageCount (
    _env: JNIEnv,
    _class: JClass,
    doc_pointer: jlong
) -> jint {

    if doc_pointer == 0 {
    return -1
    }

    let handle = unsafe { &*(doc_pointer as &const PdfDocument)};
    handle.doc.get_pages().len() as jint
}


