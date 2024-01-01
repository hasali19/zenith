use std::fs::File;
use std::io::{Read, Write};

use xml::reader::XmlEvent;
use xml::EventReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nuget_pkg = "D:/dev/projects/windows-app-sdk-sample/Microsoft.WindowsAppSDK.1.3.230502000";

    println!("cargo:rustc-link-search={nuget_pkg}/lib/win10-x64");
    println!("cargo:rustc-link-lib=Microsoft.WindowsAppRuntime");
    println!("cargo:rustc-link-lib=Microsoft.WindowsAppRuntime.Bootstrap");

    let version_info = File::open(format!("{nuget_pkg}/WindowsAppSDK-VersionInfo.xml"))?;
    let mut parser = EventReader::new(version_info);
    let mut stack = TagStack(vec![]);

    let mut major_minor = None;
    let mut tag = None;
    let mut version = None;

    while let Ok(e) = parser.next() {
        match e {
            XmlEvent::StartElement { name, .. } => {
                stack.push(name.local_name);

                if stack.matches("WindowsAppSDK/Release/MajorMinor/UInt32") {
                    major_minor = Some(parse_element_text(&mut parser, &mut stack)?);
                } else if stack.matches("WindowsAppSDK/Release/Tag") {
                    tag = Some(parse_element_text(&mut parser, &mut stack)?);
                } else if stack.matches("WindowsAppSDK/Runtime/Version/UInt16") {
                    version = Some(parse_element_text(&mut parser, &mut stack)?);
                }
            }
            XmlEvent::EndElement { name } => stack.pop(&name.local_name),
            XmlEvent::EndDocument => break,
            _ => {}
        }
    }

    let major_minor = major_minor.unwrap();
    let tag = tag.unwrap();
    let version = version.unwrap();

    let mut file = File::create("src/version_info.rs").unwrap();

    writeln!(
        file,
        "pub const WINDOWSAPPSDK_RELEASE_MAJORMINOR: u32 = {major_minor};"
    )?;

    writeln!(
        file,
        "pub const WINDOWSAPPSDK_RELEASE_VERSION_TAG_W: windows::core::PCWSTR = windows::w!(\"{tag}\");"
    )?;

    writeln!(
        file,
        "pub const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64: u64 = {version};"
    )?;

    Ok(())
}

struct TagStack(Vec<String>);

impl TagStack {
    fn push(&mut self, tag: String) {
        self.0.push(tag);
    }

    fn pop(&mut self, tag: &str) {
        if !matches!(self.0.pop(), Some(v) if v == tag) {
            panic!("invalid closing tag: {tag}")
        }
    }

    fn matches(&mut self, path: &str) -> bool {
        path.split('/')
            .zip(
                self.0
                    .iter()
                    .map(Some)
                    .chain(std::iter::from_fn(|| Some(None))),
            )
            .all(|(a, b)| matches!(b, Some(b) if a == b))
    }
}

fn parse_element_text(
    parser: &mut EventReader<impl Read>,
    stack: &mut TagStack,
) -> Result<String, xml::reader::Error> {
    match parser.next()? {
        XmlEvent::Characters(value) => Ok(value),
        XmlEvent::EndElement { name } => {
            stack.pop(&name.local_name);
            Ok(String::new())
        }
        _ => panic!("invalid value"),
    }
}
