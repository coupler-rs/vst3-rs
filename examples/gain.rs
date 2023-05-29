#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::cell::Cell;
use std::ffi::{c_char, c_void, CString};
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::{ptr, slice};

use vst3_bindgen::{uid, ComPtr, ComRef, Steinberg::Vst::*, Steinberg::*};

macro_rules! offset_of {
    ($struct:ty, $field:ident) => {{
        let dummy = std::mem::MaybeUninit::<$struct>::uninit();
        let base = dummy.as_ptr();
        let field = std::ptr::addr_of!((*base).$field);

        (field as *const c_void).offset_from(base as *const c_void)
    }};
}

fn copy_cstring(src: &str, dst: &mut [c_char]) {
    let c_string = CString::new(src).unwrap_or_else(|_| CString::default());
    let bytes = c_string.as_bytes_with_nul();

    for (src, dst) in bytes.iter().zip(dst.iter_mut()) {
        *dst = *src as c_char;
    }

    if bytes.len() > dst.len() {
        if let Some(last) = dst.last_mut() {
            *last = 0;
        }
    }
}

fn copy_wstring(src: &str, dst: &mut [TChar]) {
    let mut len = 0;
    for (src, dst) in src.encode_utf16().zip(dst.iter_mut()) {
        *dst = src as TChar;
        len += 1;
    }

    if len < dst.len() {
        dst[len] = 0;
    } else if let Some(last) = dst.last_mut() {
        *last = 0;
    }
}

unsafe fn len_wstring(string: *const TChar) -> usize {
    let mut len = 0;

    while *string.offset(len) != 0 {
        len += 1;
    }

    len as usize
}

const PLUGIN_NAME: &'static str = "Gain (vst3-bindgen example plugin)";

#[repr(C)]
struct GainProcessor {
    component: IComponent,
    audio_processor: IAudioProcessor,
    process_context_requirements: IProcessContextRequirements,
    gain: AtomicU64,
}

impl GainProcessor {
    const CID: TUID = uid(0x6E332252, 0x54224A00, 0xAA69301A, 0xF318797D);

    fn createInstance() -> *const GainProcessor {
        Arc::into_raw(Arc::new(GainProcessor {
            component: IComponent {
                vtbl: &IComponentVtbl {
                    base: IPluginBaseVtbl {
                        base: FUnknownVtbl {
                            queryInterface: GainProcessor::component_queryInterface,
                            addRef: GainProcessor::component_addRef,
                            release: GainProcessor::component_release,
                        },
                        initialize: GainProcessor::component_initialize,
                        terminate: GainProcessor::component_terminate,
                    },
                    getControllerClassId: GainProcessor::getControllerClassId,
                    setIoMode: GainProcessor::setIoMode,
                    getBusCount: GainProcessor::getBusCount,
                    getBusInfo: GainProcessor::getBusInfo,
                    getRoutingInfo: GainProcessor::getRoutingInfo,
                    activateBus: GainProcessor::activateBus,
                    setActive: GainProcessor::setActive,
                    setState: GainProcessor::setState,
                    getState: GainProcessor::getState,
                },
            },
            audio_processor: IAudioProcessor {
                vtbl: &IAudioProcessorVtbl {
                    base: FUnknownVtbl {
                        queryInterface: GainProcessor::audio_processor_queryInterface,
                        addRef: GainProcessor::audio_processor_addRef,
                        release: GainProcessor::audio_processor_release,
                    },
                    setBusArrangements: GainProcessor::setBusArrangements,
                    getBusArrangement: GainProcessor::getBusArrangement,
                    canProcessSampleSize: GainProcessor::canProcessSampleSize,
                    getLatencySamples: GainProcessor::getLatencySamples,
                    setupProcessing: GainProcessor::setupProcessing,
                    setProcessing: GainProcessor::setProcessing,
                    process: GainProcessor::process,
                    getTailSamples: GainProcessor::getTailSamples,
                },
            },
            process_context_requirements: IProcessContextRequirements {
                vtbl: &IProcessContextRequirementsVtbl {
                    base: FUnknownVtbl {
                        queryInterface: GainProcessor::process_context_requirements_queryInterface,
                        addRef: GainProcessor::process_context_requirements_addRef,
                        release: GainProcessor::process_context_requirements_release,
                    },
                    getProcessContextRequirements: GainProcessor::getProcessContextRequirements,
                },
            },
            gain: AtomicU64::new(1.0f64.to_bits()),
        }))
    }

    unsafe fn queryInterface(
        this: *mut GainProcessor,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> tresult {
        let iid = *iid;

        if iid == FUnknown_iid || iid == IPluginBase_iid || iid == IComponent_iid {
            Self::addRef(this);
            *obj = (this as *mut c_void).offset(offset_of!(Self, component)) as *mut c_void;
            return kResultOk;
        }

        if iid == IAudioProcessor_iid {
            Self::addRef(this);
            *obj = (this as *mut c_void).offset(offset_of!(Self, audio_processor)) as *mut c_void;
            return kResultOk;
        }

        if iid == IProcessContextRequirements_iid {
            Self::addRef(this);
            *obj = (this as *mut c_void).offset(offset_of!(Self, process_context_requirements))
                as *mut c_void;
            return kResultOk;
        }

        kNoInterface
    }

    unsafe fn addRef(this: *mut GainProcessor) -> u32 {
        let arc = Arc::from_raw(this);
        let result = Arc::strong_count(&arc) + 1;
        let _ = Arc::into_raw(arc);

        Arc::increment_strong_count(this);

        result as u32
    }

    unsafe fn release(this: *mut GainProcessor) -> u32 {
        let arc = Arc::from_raw(this);
        let result = Arc::strong_count(&arc) - 1;
        let _ = Arc::into_raw(arc);

        Arc::decrement_strong_count(this);

        result as u32
    }

    unsafe extern "system" fn component_queryInterface(
        this: *mut FUnknown,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> tresult {
        Self::queryInterface(
            (this as *mut c_void).offset(-offset_of!(Self, component)) as *mut GainProcessor,
            iid,
            obj,
        )
    }

    unsafe extern "system" fn component_addRef(this: *mut FUnknown) -> u32 {
        Self::addRef(
            (this as *mut c_void).offset(-offset_of!(Self, component)) as *mut GainProcessor
        )
    }

    unsafe extern "system" fn component_release(this: *mut FUnknown) -> u32 {
        Self::release(
            (this as *mut c_void).offset(-offset_of!(Self, component)) as *mut GainProcessor
        )
    }

    unsafe extern "system" fn component_initialize(
        _this: *mut IPluginBase,
        _context: *mut FUnknown,
    ) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn component_terminate(_this: *mut IPluginBase) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn getControllerClassId(
        _this: *mut IComponent,
        class_id: *mut TUID,
    ) -> tresult {
        *class_id = GainController::CID;
        kResultOk
    }

    unsafe extern "system" fn setIoMode(_this: *mut IComponent, _mode: IoMode) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn getBusCount(
        _this: *mut IComponent,
        mediaType: MediaType,
        dir: BusDirection,
    ) -> i32 {
        match mediaType as BusDirections {
            MediaTypes_::kAudio => match dir as BusDirections {
                BusDirections_::kInput => 1,
                BusDirections_::kOutput => 1,
                _ => 0,
            },
            MediaTypes_::kEvent => 0,
            _ => 0,
        }
    }

    unsafe extern "system" fn getBusInfo(
        _this: *mut IComponent,
        mediaType: MediaType,
        dir: BusDirection,
        index: i32,
        bus: *mut BusInfo,
    ) -> tresult {
        match mediaType as MediaTypes {
            MediaTypes_::kAudio => match dir as BusDirections {
                BusDirections_::kInput => match index {
                    0 => {
                        let bus = &mut *bus;

                        bus.mediaType = MediaTypes_::kAudio as MediaType;
                        bus.direction = BusDirections_::kInput as BusDirection;
                        bus.channelCount = 2;
                        copy_wstring("Input", &mut bus.name);
                        bus.busType = BusTypes_::kMain as BusType;
                        bus.flags = BusInfo_::BusFlags_::kDefaultActive as u32;

                        kResultOk
                    }
                    _ => kInvalidArgument,
                },
                BusDirections_::kOutput => match index {
                    0 => {
                        let bus = &mut *bus;

                        bus.mediaType = MediaTypes_::kAudio as MediaType;
                        bus.direction = BusDirections_::kOutput as BusDirection;
                        bus.channelCount = 2;
                        copy_wstring("Output", &mut bus.name);
                        bus.busType = BusTypes_::kMain as BusType;
                        bus.flags = BusInfo_::BusFlags_::kDefaultActive as u32 as uint32;

                        kResultOk
                    }
                    _ => kInvalidArgument,
                },
                _ => kInvalidArgument,
            },
            MediaTypes_::kEvent => kInvalidArgument,
            _ => kInvalidArgument,
        }
    }

    unsafe extern "system" fn getRoutingInfo(
        _this: *mut IComponent,
        _in_info: *mut RoutingInfo,
        _out_info: *mut RoutingInfo,
    ) -> tresult {
        kNotImplemented
    }

    unsafe extern "system" fn activateBus(
        _this: *mut IComponent,
        _media_type: MediaType,
        _dir: BusDirection,
        _index: i32,
        _state: TBool,
    ) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn setActive(_this: *mut IComponent, _state: TBool) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn setState(_this: *mut IComponent, _state: *mut IBStream) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn getState(_this: *mut IComponent, _state: *mut IBStream) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn audio_processor_queryInterface(
        this: *mut FUnknown,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> tresult {
        Self::queryInterface(
            (this as *mut c_void).offset(-offset_of!(Self, audio_processor)) as *mut GainProcessor,
            iid,
            obj,
        )
    }

    unsafe extern "system" fn audio_processor_addRef(this: *mut FUnknown) -> u32 {
        Self::addRef(
            (this as *mut c_void).offset(-offset_of!(Self, audio_processor)) as *mut GainProcessor,
        )
    }

    unsafe extern "system" fn audio_processor_release(this: *mut FUnknown) -> u32 {
        Self::release(
            (this as *mut c_void).offset(-offset_of!(Self, audio_processor)) as *mut GainProcessor,
        )
    }

    unsafe extern "system" fn setBusArrangements(
        _this: *mut IAudioProcessor,
        inputs: *mut SpeakerArrangement,
        num_ins: i32,
        outputs: *mut SpeakerArrangement,
        num_outs: i32,
    ) -> tresult {
        if num_ins != 1 || num_outs != 1 {
            return kResultFalse;
        }

        if *inputs != SpeakerArr::kStereo || *outputs != SpeakerArr::kStereo {
            return kResultFalse;
        }

        kResultTrue
    }

    unsafe extern "system" fn getBusArrangement(
        _this: *mut IAudioProcessor,
        dir: BusDirection,
        index: i32,
        arr: *mut SpeakerArrangement,
    ) -> tresult {
        match dir as BusDirections {
            BusDirections_::kInput => {
                if index == 0 {
                    *arr = SpeakerArr::kStereo;
                    kResultOk
                } else {
                    kInvalidArgument
                }
            }
            BusDirections_::kOutput => {
                if index == 0 {
                    *arr = SpeakerArr::kStereo;
                    kResultOk
                } else {
                    kInvalidArgument
                }
            }
            _ => kInvalidArgument,
        }
    }

    unsafe extern "system" fn canProcessSampleSize(
        _this: *mut IAudioProcessor,
        symbolic_sample_size: i32,
    ) -> tresult {
        match symbolic_sample_size as SymbolicSampleSizes {
            SymbolicSampleSizes_::kSample32 => kResultOk as i32,
            SymbolicSampleSizes_::kSample64 => kNotImplemented as i32,
            _ => kInvalidArgument,
        }
    }

    unsafe extern "system" fn getLatencySamples(_this: *mut IAudioProcessor) -> u32 {
        0
    }

    unsafe extern "system" fn setupProcessing(
        _this: *mut IAudioProcessor,
        _setup: *mut ProcessSetup,
    ) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn setProcessing(_this: *mut IAudioProcessor, _state: TBool) -> tresult {
        kResultOk
    }

    unsafe extern "system" fn process(
        this: *mut IAudioProcessor,
        data: *mut ProcessData,
    ) -> tresult {
        let processor = &mut *((this as *mut c_void).offset(-offset_of!(Self, audio_processor))
            as *mut GainProcessor);

        let process_data = &*data;

        if let Some(param_changes) = ComRef::from_raw(process_data.inputParameterChanges) {
            let param_count = param_changes.getParameterCount();
            for param_index in 0..param_count {
                if let Some(param_queue) =
                    ComRef::from_raw(param_changes.getParameterData(param_index))
                {
                    let param_id = param_queue.getParameterId();
                    let point_count = param_queue.getPointCount();

                    match param_id {
                        0 => {
                            let mut sample_offset = 0;
                            let mut value = 0.0;
                            let result = param_queue.getPoint(
                                point_count - 1,
                                &mut sample_offset,
                                &mut value,
                            );

                            if result == kResultTrue {
                                processor.gain.store(value.to_bits(), Ordering::Relaxed);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let gain = f64::from_bits(processor.gain.load(Ordering::Relaxed)) as f32;

        let num_samples = process_data.numSamples as usize;

        if process_data.numInputs != 1 || process_data.numOutputs != 1 {
            return kResultOk;
        }

        let input_buses =
            slice::from_raw_parts(process_data.inputs, process_data.numInputs as usize);
        let output_buses =
            slice::from_raw_parts(process_data.outputs, process_data.numOutputs as usize);

        if input_buses[0].numChannels != 2 || output_buses[0].numChannels != 2 {
            return kResultOk;
        }

        let input_channels = slice::from_raw_parts(
            input_buses[0].__field0.channelBuffers32,
            input_buses[0].numChannels as usize,
        );
        let output_channels = slice::from_raw_parts(
            output_buses[0].__field0.channelBuffers32,
            output_buses[0].numChannels as usize,
        );

        let input_l = slice::from_raw_parts(input_channels[0], num_samples);
        let input_r = slice::from_raw_parts(input_channels[1], num_samples);
        let output_l = slice::from_raw_parts_mut(output_channels[0], num_samples);
        let output_r = slice::from_raw_parts_mut(output_channels[1], num_samples);

        for i in 0..num_samples {
            output_l[i] = gain * input_l[i];
            output_r[i] = gain * input_r[i];
        }

        kResultOk
    }

    unsafe extern "system" fn getTailSamples(_this: *mut IAudioProcessor) -> u32 {
        0
    }

    pub unsafe extern "system" fn process_context_requirements_queryInterface(
        this: *mut FUnknown,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> tresult {
        Self::queryInterface(
            (this as *mut c_void).offset(-offset_of!(Self, process_context_requirements))
                as *mut GainProcessor,
            iid,
            obj,
        )
    }

    pub unsafe extern "system" fn process_context_requirements_addRef(this: *mut FUnknown) -> u32 {
        Self::addRef(
            (this as *mut c_void).offset(-offset_of!(Self, process_context_requirements))
                as *mut GainProcessor,
        )
    }

    pub unsafe extern "system" fn process_context_requirements_release(this: *mut FUnknown) -> u32 {
        Self::release(
            (this as *mut c_void).offset(-offset_of!(Self, process_context_requirements))
                as *mut GainProcessor,
        )
    }

    pub unsafe extern "system" fn getProcessContextRequirements(
        _this: *mut IProcessContextRequirements,
    ) -> u32 {
        0
    }
}

#[repr(C)]
struct GainController {
    edit_controller: IEditController,
    gain: Cell<f64>,
}

impl GainController {
    const CID: TUID = uid(0x1BA8A477, 0xEE0A4A2D, 0x80F50D14, 0x13D2EAA0);

    fn createInstance() -> *const GainController {
        Arc::into_raw(Arc::new(GainController {
            edit_controller: IEditController {
                vtbl: &IEditControllerVtbl {
                    base: IPluginBaseVtbl {
                        base: FUnknownVtbl {
                            queryInterface: GainController::edit_controller_queryInterface,
                            addRef: GainController::edit_controller_addRef,
                            release: GainController::edit_controller_release,
                        },
                        initialize: GainController::edit_controller_initialize,
                        terminate: GainController::edit_controller_terminate,
                    },
                    setComponentState: GainController::setComponentState,
                    setState: GainController::edit_controller_setState,
                    getState: GainController::edit_controller_getState,
                    getParameterCount: GainController::getParameterCount,
                    getParameterInfo: GainController::getParameterInfo,
                    getParamStringByValue: GainController::getParamStringByValue,
                    getParamValueByString: GainController::getParamValueByString,
                    normalizedParamToPlain: GainController::normalizedParamToPlain,
                    plainParamToNormalized: GainController::plainParamToNormalized,
                    getParamNormalized: GainController::getParamNormalized,
                    setParamNormalized: GainController::setParamNormalized,
                    setComponentHandler: GainController::setComponentHandler,
                    createView: GainController::createView,
                },
            },
            gain: Cell::new(1.0),
        }))
    }

    unsafe fn queryInterface(
        this: *mut GainController,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> tresult {
        let iid = *iid;

        if iid == FUnknown_iid || iid == IEditController_iid {
            Self::addRef(this);
            *obj = (this as *mut c_void).offset(offset_of!(Self, edit_controller)) as *mut c_void;
            return kResultOk;
        }

        kNoInterface
    }

    unsafe fn addRef(this: *mut GainController) -> u32 {
        let arc = Arc::from_raw(this as *const GainController);
        let result = Arc::strong_count(&arc) + 1;
        let _ = Arc::into_raw(arc);

        Arc::increment_strong_count(this as *const GainController);

        result as u32
    }

    unsafe fn release(this: *mut GainController) -> u32 {
        let arc = Arc::from_raw(this as *const GainController);
        let result = Arc::strong_count(&arc) - 1;
        let _ = Arc::into_raw(arc);

        Arc::decrement_strong_count(this as *const GainController);

        result as u32
    }

    pub unsafe extern "system" fn edit_controller_queryInterface(
        this: *mut FUnknown,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> tresult {
        Self::queryInterface(
            (this as *mut c_void).offset(-offset_of!(Self, edit_controller)) as *mut GainController,
            iid,
            obj,
        )
    }

    pub unsafe extern "system" fn edit_controller_addRef(this: *mut FUnknown) -> u32 {
        Self::addRef(
            (this as *mut c_void).offset(-offset_of!(Self, edit_controller)) as *mut GainController,
        )
    }

    pub unsafe extern "system" fn edit_controller_release(this: *mut FUnknown) -> u32 {
        Self::release(
            (this as *mut c_void).offset(-offset_of!(Self, edit_controller)) as *mut GainController,
        )
    }

    pub unsafe extern "system" fn edit_controller_initialize(
        _this: *mut IPluginBase,
        _context: *mut FUnknown,
    ) -> tresult {
        kResultOk
    }

    pub unsafe extern "system" fn edit_controller_terminate(_this: *mut IPluginBase) -> tresult {
        kResultOk
    }

    pub unsafe extern "system" fn setComponentState(
        _this: *mut IEditController,
        _state: *mut IBStream,
    ) -> tresult {
        kNotImplemented
    }

    pub unsafe extern "system" fn edit_controller_setState(
        _this: *mut IEditController,
        _state: *mut IBStream,
    ) -> tresult {
        kResultOk
    }

    pub unsafe extern "system" fn edit_controller_getState(
        _this: *mut IEditController,
        _state: *mut IBStream,
    ) -> tresult {
        kResultOk
    }

    pub unsafe extern "system" fn getParameterCount(_this: *mut IEditController) -> i32 {
        1
    }

    pub unsafe extern "system" fn getParameterInfo(
        _this: *mut IEditController,
        param_index: i32,
        info: *mut ParameterInfo,
    ) -> tresult {
        match param_index {
            0 => {
                let info = &mut *info;

                info.id = 0;
                copy_wstring("Gain", &mut info.title);
                copy_wstring("Gain", &mut info.shortTitle);
                copy_wstring("", &mut info.units);
                info.stepCount = 0;
                info.defaultNormalizedValue = 1.0;
                info.unitId = 0;
                info.flags = ParameterInfo_::ParameterFlags_::kCanAutomate as i32;

                kResultOk
            }
            _ => kInvalidArgument,
        }
    }

    pub unsafe extern "system" fn getParamStringByValue(
        _this: *mut IEditController,
        id: u32,
        value_normalized: f64,
        string: *mut String128,
    ) -> tresult {
        let slice = unsafe { &mut *string };

        match id {
            0 => {
                let display = value_normalized.to_string();
                copy_wstring(&display, slice);
                kResultOk
            }
            _ => kInvalidArgument,
        }
    }

    pub unsafe extern "system" fn getParamValueByString(
        _this: *mut IEditController,
        id: u32,
        string: *mut TChar,
        value_normalized: *mut f64,
    ) -> tresult {
        match id {
            0 => {
                let len = len_wstring(string as *const TChar);
                if let Ok(string) =
                    String::from_utf16(slice::from_raw_parts(string as *const u16, len))
                {
                    if let Ok(value) = f64::from_str(&string) {
                        *value_normalized = value;
                        return kResultOk;
                    }
                }
                kInvalidArgument
            }
            _ => kInvalidArgument,
        }
    }

    pub unsafe extern "system" fn normalizedParamToPlain(
        _this: *mut IEditController,
        id: u32,
        value_normalized: f64,
    ) -> f64 {
        match id {
            0 => value_normalized,
            _ => 0.0,
        }
    }

    pub unsafe extern "system" fn plainParamToNormalized(
        _this: *mut IEditController,
        id: u32,
        plain_value: f64,
    ) -> f64 {
        match id {
            0 => plain_value,
            _ => 0.0,
        }
    }

    pub unsafe extern "system" fn getParamNormalized(this: *mut IEditController, id: u32) -> f64 {
        let controller = &*((this as *mut c_void).offset(-offset_of!(Self, edit_controller))
            as *const GainController);
        match id {
            0 => controller.gain.get(),
            _ => 0.0,
        }
    }

    pub unsafe extern "system" fn setParamNormalized(
        this: *mut IEditController,
        id: u32,
        value: f64,
    ) -> tresult {
        let controller = &mut *((this as *mut c_void).offset(-offset_of!(Self, edit_controller))
            as *mut GainController);
        match id {
            0 => {
                controller.gain.set(value);
                kResultOk
            }
            _ => kInvalidArgument,
        }
    }

    pub unsafe extern "system" fn setComponentHandler(
        _this: *mut IEditController,
        _handler: *mut IComponentHandler,
    ) -> tresult {
        kResultOk
    }

    pub unsafe extern "system" fn createView(
        _this: *mut IEditController,
        _name: *const c_char,
    ) -> *mut IPlugView {
        ptr::null_mut()
    }
}

#[repr(C)]
struct Factory {
    vtbl: *const IPluginFactoryVtbl,
}

unsafe impl Sync for Factory {}

impl Factory {
    unsafe extern "system" fn queryInterface(
        this: *mut FUnknown,
        iid: *const TUID,
        obj: *mut *mut c_void,
    ) -> tresult {
        let iid = *iid;

        if iid == FUnknown_iid || iid == IPluginFactory_iid {
            *obj = this as *mut c_void;
            return kResultOk;
        }

        kNoInterface
    }

    unsafe extern "system" fn addRef(_this: *mut FUnknown) -> u32 {
        1
    }

    unsafe extern "system" fn release(_this: *mut FUnknown) -> u32 {
        1
    }

    unsafe extern "system" fn getFactoryInfo(
        _this: *mut IPluginFactory,
        info: *mut PFactoryInfo,
    ) -> tresult {
        let info = &mut *info;

        copy_cstring("Vendor", &mut info.vendor);
        copy_cstring("https://example.com", &mut info.url);
        copy_cstring("someone@example.com", &mut info.email);
        info.flags = PFactoryInfo_::FactoryFlags_::kUnicode as int32;

        kResultOk
    }

    unsafe extern "system" fn countClasses(_this: *mut IPluginFactory) -> i32 {
        2
    }

    unsafe extern "system" fn getClassInfo(
        _this: *mut IPluginFactory,
        index: i32,
        info: *mut PClassInfo,
    ) -> tresult {
        match index {
            0 => {
                let info = &mut *info;
                info.cid = GainProcessor::CID;
                info.cardinality = PClassInfo_::ClassCardinality_::kManyInstances as int32;
                copy_cstring("Audio Module Class", &mut info.category);
                copy_cstring(PLUGIN_NAME, &mut info.name);

                kResultOk
            }
            1 => {
                let info = &mut *info;
                info.cid = GainController::CID;
                info.cardinality = PClassInfo_::ClassCardinality_::kManyInstances as int32;
                copy_cstring("Component Controller Class", &mut info.category);
                copy_cstring(PLUGIN_NAME, &mut info.name);

                kResultOk
            }
            _ => kInvalidArgument,
        }
    }

    unsafe extern "system" fn createInstance(
        _this: *mut IPluginFactory,
        cid: FIDString,
        iid: FIDString,
        obj: *mut *mut c_void,
    ) -> tresult {
        let instance = match *(cid as *const TUID) {
            GainProcessor::CID => Some(GainProcessor::createInstance() as *mut FUnknown),
            GainController::CID => Some(GainController::createInstance() as *mut FUnknown),
            _ => None,
        };

        if let Some(instance) = instance {
            let instance = ComPtr::from_raw_unchecked(instance);
            instance.queryInterface(iid as *mut TUID, obj)
        } else {
            kInvalidArgument
        }
    }
}

static PLUGIN_FACTORY: Factory = Factory {
    vtbl: &IPluginFactoryVtbl {
        base: FUnknownVtbl {
            queryInterface: Factory::queryInterface,
            addRef: Factory::addRef,
            release: Factory::release,
        },
        getFactoryInfo: Factory::getFactoryInfo,
        countClasses: Factory::countClasses,
        getClassInfo: Factory::getClassInfo,
        createInstance: Factory::createInstance,
    },
};

#[cfg(target_os = "windows")]
#[no_mangle]
extern "system" fn InitDll() -> bool {
    true
}

#[cfg(target_os = "windows")]
#[no_mangle]
extern "system" fn ExitDll() -> bool {
    true
}

#[cfg(target_os = "macos")]
#[no_mangle]
extern "system" fn BundleEntry(_bundle_ref: *mut c_void) -> bool {
    true
}

#[cfg(target_os = "macos")]
#[no_mangle]
extern "system" fn BundleExit() -> bool {
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
extern "system" fn ModuleEntry(_library_handle: *mut c_void) -> bool {
    true
}

#[cfg(target_os = "linux")]
#[no_mangle]
extern "system" fn ModuleExit() -> bool {
    true
}

#[no_mangle]
extern "system" fn GetPluginFactory() -> *mut IPluginFactory {
    &PLUGIN_FACTORY as *const Factory as *mut IPluginFactory
}
