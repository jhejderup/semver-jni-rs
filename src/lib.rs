extern crate jni;
extern crate semver;

use semver::Version;
use semver::VersionReq;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jstring};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_Crates_resolve(env: JNIEnv,
                                             _class: JClass,
                                             req: JString,
                                             versions: JString)
                                             -> jstring {

    let req: String =
        env.get_string(req).expect("Couldn't get java string!").into();
    let verstr: String =
        env.get_string(versions).expect("Couldn't get java string!").into();

    let r = VersionReq::parse(&req).unwrap();

    let resolved_result = verstr.split(",")
                          .map(|v| Version::parse(v))
                          .filter(|ver| ver.is_ok()) //only consider valid versions
                          .map(|ver| ver.unwrap()) //only unwrap valid versions
                          .filter(|ver| r.matches(ver)) //does the version fit the constraint?
                          .max(); //get the latest version

    let resolved = if let Some(value) = resolved_result { format!("{}", value) } else {format!("null") };
    // null is returned when the versions are not valid (e.g., "abc") or there are no valid matches
    
    let output = env.new_string(resolved)
        .expect("Couldn't create java string!");
    
    output.into_inner()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
