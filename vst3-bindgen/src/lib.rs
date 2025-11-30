//! A binding generator for the VST 3 API. `vst3-bindgen` can be used to generate Rust bindings for
//! the VST 3 API from the original C++ headers.

use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn find_headers<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>, io::Error> {
    fn find_headers_inner<P: AsRef<Path>>(dir: P, headers: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;

            let path = entry.path();
            if metadata.file_type().is_dir() {
                find_headers_inner(path, headers)?;
            } else {
                if path.extension().map(|ext| ext == "h").unwrap_or(false) {
                    headers.push(path);
                }
            }
        }

        Ok(())
    }

    let mut headers = Vec::new();
    find_headers_inner(dir, &mut headers)?;

    Ok(headers)
}

fn parse_iid(tokens: &[&str]) -> Option<String> {
    if let Some(first) = tokens.first() {
        if *first == "DECLARE_CLASS_IID" {
            return Some(format!(
                "crate::support::uid({}, {}, {}, {})",
                tokens[4], tokens[6], tokens[8], tokens[10]
            ));
        }
    }

    None
}

/// Generates Rust bindings given a path to the VST 3 SDK.
pub fn generate(
    sdk_dir: &Path,
    target: Option<&str>,
    mut sink: impl Write,
) -> Result<(), Box<dyn Error>> {
    let pluginterfaces_path = sdk_dir.join("pluginterfaces");
    let headers = find_headers(&pluginterfaces_path)?;

    let skip_headers = HashSet::from([
        Path::new("pluginterfaces/base/funknownimpl.h"),
        Path::new("pluginterfaces/base/ustring.h"),
        Path::new("pluginterfaces/test/itest.h"),
        Path::new("pluginterfaces/vst/ivsttestplugprovider.h"),
    ]);

    let mut source = String::new();
    for header in &headers {
        let relative = header.strip_prefix(sdk_dir).unwrap();
        if skip_headers.contains(relative) {
            continue;
        }

        let name = relative.to_str().unwrap();

        use std::fmt::Write;
        writeln!(source, "#include \"{}\"", name)?;
    }

    #[rustfmt::skip]
    let mut generator = com_scrape::Generator::default()
        .skip_types(&[
            "Steinberg::SKI::Detail::Adopt",
            "Steinberg::ConstStringTable",
            "Steinberg::FUID",
            "Steinberg::FReleaser",
            "Steinberg::LARGE_INT",
        ])
        .skip_interface_trait("Steinberg::FUnknown")
        .override_typedef_types(&[
            ("Steinberg::Direction", "crate::support::DefaultEnumType"),
            ("Steinberg::KeyModifier", "crate::support::DefaultEnumType"),
            ("Steinberg::Orientation", "crate::support::DefaultEnumType"),
            ("Steinberg::StandardColor", "crate::support::DefaultEnumType"),
            ("Steinberg::VirtualKeyCodes", "crate::support::DefaultEnumType"),
            ("Steinberg::IBStream::IStreamSeekMode", "crate::support::DefaultEnumType"),
            ("Steinberg::IDependent::ChangeMessage", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfo::ClassCardinality", "crate::support::DefaultEnumType"),
            ("Steinberg::PFactoryInfo::FactoryFlags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::BusDirections", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::BusTypes", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::ComponentFlags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::ControllerNumbers", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::IoModes", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::MediaTypes", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::PhysicalUITypeIDs", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::ProcessModes", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::SymbolicSampleSizes", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::ePrefetchableSupport", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::BusInfo::BusFlags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::Chord::Masks", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::DataEvent::DataTypes", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::Event::EventFlags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::Event::EventTypes", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::FrameRate::FrameRateFlags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::IContextMenuItem::Flags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::IProcessContextRequirements::Flags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::NoteExpressionTypeInfo::NoteExpressionTypeFlags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::ProcessContext::StatesAndFlags", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::ChannelContext::ChannelPluginLocation", "crate::support::DefaultEnumType"),
        ])
        .override_constant_types(&[
            ("Steinberg::FVariant::kEmpty", "crate::support::DefaultEnumType"),
            ("Steinberg::FVariant::kFloat", "crate::support::DefaultEnumType"),
            ("Steinberg::FVariant::kInteger", "crate::support::DefaultEnumType"),
            ("Steinberg::FVariant::kObject", "crate::support::DefaultEnumType"),
            ("Steinberg::FVariant::kOwner", "crate::support::DefaultEnumType"),
            ("Steinberg::FVariant::kString16", "crate::support::DefaultEnumType"),
            ("Steinberg::FVariant::kString8", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfo::kCategorySize", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfo::kNameSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfo2::kSubCategoriesSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfo2::kVendorSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfo2::kVersionSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfoW::kSubCategoriesSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfoW::kVendorSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PClassInfoW::kVersionSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PFactoryInfo::kEmailSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PFactoryInfo::kNameSize", "crate::support::DefaultEnumType"),
            ("Steinberg::PFactoryInfo::kURLSize", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::RepresentationInfo::kNameSize", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kDisplay", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kEndOfLayerType", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kFader", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kKnob", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kLED", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kLink", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kPressedKnob", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kSwitch", "crate::support::DefaultEnumType"),
            ("Steinberg::Vst::LayerType::kSwitchKnob", "crate::support::DefaultEnumType"),
        ])
        .override_constant_values(&[
            ("Steinberg::kNoInterface", "crate::support::kNoInterface"),
            ("Steinberg::kResultOk", "crate::support::kResultOk"),
            ("Steinberg::kResultTrue", "crate::support::kResultTrue"),
            ("Steinberg::kResultFalse", "crate::support::kResultFalse"),
            ("Steinberg::kInvalidArgument", "crate::support::kInvalidArgument"),
            ("Steinberg::kNotImplemented", "crate::support::kNotImplemented"),
            ("Steinberg::kInternalError", "crate::support::kInternalError"),
            ("Steinberg::kNotInitialized", "crate::support::kNotInitialized"),
            ("Steinberg::kOutOfMemory", "crate::support::kOutOfMemory"),
            ("Steinberg::Vst::PhysicalUITypeIDs::kInvalidPUITypeID", "0xFFFFFFFFu32 as crate::support::DefaultEnumType"),
        ])
        .constant_parser(parse_iid)
        .iid_generator(|name| format!("crate::support::tuid_as_guid({name}_iid)"))
        .query_interface_fn("crate::support::FUnknown_query_interface")
        .add_ref_fn("crate::support::FUnknown_add_ref")
        .release_fn("crate::support::FUnknown_release")
        .include_path(sdk_dir);

    if let Some(target) = target {
        generator = generator.target(target);
    }

    generator.generate(source, &mut sink)?;

    Ok(())
}
