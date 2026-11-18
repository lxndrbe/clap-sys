#![no_std]
#![allow(non_camel_case_types)]

pub mod ext;
pub mod factory;

pub mod audio_buffer;
pub mod color;
pub mod entry;
pub mod events;
pub mod fixedpoint;
pub mod host;
pub mod id;
pub mod plugin;
pub mod plugin_features;
pub mod process;
pub mod stream;
pub mod string_sizes;
pub mod timestamp;
pub mod universal_plugin_id;
pub mod version;

#[cfg(test)]
mod layout_tests {
    use core::mem::{align_of, size_of};

    use crate::audio_buffer::clap_audio_buffer;
    use crate::entry::clap_plugin_entry;
    use crate::events::{
        clap_event_header, clap_event_midi, clap_event_note, clap_event_param_value,
        clap_event_transport,
    };
    use crate::ext::audio_ports::clap_audio_port_info;
    use crate::ext::audio_ports_config::clap_audio_ports_config;
    use crate::ext::gui::{clap_gui_resize_hints, clap_window};
    use crate::ext::params::clap_param_info;
    use crate::host::clap_host;
    use crate::plugin::{clap_plugin, clap_plugin_descriptor};
    use crate::process::clap_process;
    use crate::version::clap_version;

    #[test]
    fn core_struct_sizes() {
        assert_eq!(size_of::<clap_version>(), 12);
        assert_eq!(align_of::<clap_version>(), 4);

        assert_eq!(size_of::<clap_event_header>(), 16);
        assert_eq!(align_of::<clap_event_header>(), 4);

        // header(16) + note_id(4) + 3xi16(6) + pad(6) + velocity(8)
        assert_eq!(size_of::<clap_event_note>(), 40);
        assert_eq!(align_of::<clap_event_note>(), 8);

        assert_eq!(size_of::<clap_event_midi>(), 24);
        assert_eq!(align_of::<clap_event_midi>(), 4);

        assert_eq!(size_of::<clap_audio_buffer>(), 32);
        assert_eq!(align_of::<clap_audio_buffer>(), 8);

        assert_eq!(size_of::<clap_gui_resize_hints>(), 12);
        assert_eq!(align_of::<clap_gui_resize_hints>(), 4);
    }

    #[test]
    fn pointer_heavy_struct_alignments() {
        assert_eq!(align_of::<clap_plugin_entry>(), align_of::<*const ()>());
        assert_eq!(align_of::<clap_plugin>(), align_of::<*const ()>());
        assert_eq!(
            align_of::<clap_plugin_descriptor>(),
            align_of::<*const ()>()
        );
        assert_eq!(align_of::<clap_host>(), align_of::<*const ()>());
        assert_eq!(align_of::<clap_process>(), align_of::<*const ()>());
        assert_eq!(align_of::<clap_window>(), align_of::<*const ()>());
        assert_eq!(
            align_of::<clap_event_param_value>(),
            align_of::<*const ()>()
        );
        assert_eq!(align_of::<clap_event_transport>(), 8);
        assert_eq!(align_of::<clap_audio_port_info>(), align_of::<*const ()>());
        assert_eq!(
            align_of::<clap_audio_ports_config>(),
            align_of::<*const ()>()
        );
        assert_eq!(align_of::<clap_param_info>(), align_of::<*const ()>());
    }
}
