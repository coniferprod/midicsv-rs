use std::{fs, io};
use std::error::Error;
use std::str;
use midly::{Smf, Format, Header, Timing, Track, TrackEvent, TrackEventKind, MidiMessage, MetaMessage};
use csv::{Writer, WriterBuilder, QuoteStyle};

fn main() {
    let data = fs::read("closeenc.mid").unwrap();
    let smf = Smf::parse(&data);
    match smf {
        Ok(smf) => {
            /*
            println!("Format {:?} MIDI file. {} tracks, timing = {:?}",
                smf.header.format,
                smf.tracks.len(),
                smf.header.timing
            );
            */

            let mut writer = csv::WriterBuilder::new()
                .flexible(true)   // MIDICSV records have a variable number of fields
                .quote_style(QuoteStyle::Necessary)
                .from_writer(io::stdout());

            let track_count = smf.tracks.len();

            write_header(&mut writer, &smf.header, track_count).unwrap();

            for (index, track) in smf.tracks.iter().enumerate() {
                let track_num = index + 1;
                writer.write_record(&[&track_num.to_string(), "0", "Start_track"]).unwrap();
                write_track(&mut writer, track_num, &track).unwrap();
            }

            write_eof(&mut writer).unwrap();
            writer.flush().unwrap();  // unnecessary?

        },
        Err(err) => {
            println!("error reading MIDI file: {}", err);
        }
    }
}

fn write_eof(writer: &mut Writer<io::Stdout>) -> Result<(), Box<dyn Error>> {
    writer.write_record(&["0", "0", "End_of_file"]).unwrap();

    Ok(())
}

fn write_track(writer: &mut Writer<io::Stdout>, track_num: usize, track: &Track) -> Result<(), Box<dyn Error>> {
    let mut abs_time = 0;

    for event in track {
        abs_time += u32::from(event.delta);

        let mut fields = vec![track_num.to_string()];

        match event.kind {
            TrackEventKind::Meta(message) => {
                match message {
                    // For SMPTE offset and sequence/track number, the time is always zero.
                    // Others use the running time counter.
                    MetaMessage::SmpteOffset(_) => {
                        fields.push(0.to_string());
                    },
                    MetaMessage::TrackNumber(_) => {
                        fields.push(0.to_string());
                    },
                    _ => {
                        fields.push(abs_time.to_string());
                    }
                }

                match message {
                    MetaMessage::TrackName(text) => {
                        fields.push("Title_t".to_string());
                        fields.push(str::from_utf8(text).unwrap().to_string());
                    },
                    MetaMessage::Copyright(text) => {
                        fields.push("Copyright_t".to_string());
                        fields.push(str::from_utf8(text).unwrap().to_string());
                    },
                    MetaMessage::InstrumentName(name) => {
                        fields.push("Instrument_name_t".to_string());
                        fields.push(str::from_utf8(name).unwrap().to_string());
                    },
                    MetaMessage::Marker(text) => {
                        fields.push("Marker_t".to_string());
                        fields.push(str::from_utf8(text).unwrap().to_string());
                    },
                    MetaMessage::CuePoint(text) => {
                        fields.push("Cue_point_t".to_string());
                        fields.push(str::from_utf8(text).unwrap().to_string());
                    },
                    MetaMessage::Lyric(text) => {
                        fields.push("Lyric_t".to_string());
                        fields.push(str::from_utf8(text).unwrap().to_string());
                    }
                    MetaMessage::Text(text) => {
                        fields.push("Text_t".to_string());
                        fields.push(str::from_utf8(text).unwrap().to_string());
                    },
                    MetaMessage::TrackNumber(number) => {
                        fields.push("Sequence_number".to_string());
                        match number {
                            Some(n) => fields.push(n.to_string()),
                            None => fields.push(0.to_string()),
                        };
                    }
                    MetaMessage::MidiPort(number) => {
                        fields.push("Text_t".to_string());
                        fields.push(number.as_int().to_string());
                    },
                    MetaMessage::TimeSignature(t1, t2, t3, t4) => {
                        fields.extend(vec![
                            "Time_signature_t".to_string(),
                            t1.to_string(),
                            t2.to_string(),
                            t3.to_string(),
                            t4.to_string(),
                        ]);
                    },
                    MetaMessage::KeySignature(k1, k2) => {
                        fields.extend(vec![
                            "Key_signature".to_string(),
                            k1.to_string(),
                            k2.to_string(),
                        ]);
                    },
                    MetaMessage::Tempo(tempo) => {
                        fields.extend(vec!["Tempo_t".to_string(), tempo.to_string()]);
                    },
                    MetaMessage::SmpteOffset(time) => {
                        fields.extend(vec![
                            "SMPTE_offset".to_string(),
                            time.hour().to_string(),
                            time.minute().to_string(),
                            time.second().to_string(),
                            time.frame().to_string(),
                            time.subframe().to_string(),
                        ])
                    },
                    MetaMessage::SequencerSpecific(data) => {
                        fields.push("Sequencer_specific".to_string());
                        fields.push(data.len().to_string());
                        for b in data {
                            fields.push(b.to_string());
                        }
                    },
                    _ => {},
                }
            },

            TrackEventKind::Midi { channel, message } => {
                fields.push(abs_time.to_string());

                match message {
                    MidiMessage::NoteOff { key, vel } => {
                        fields.push("Note_off_c".to_string());
                        fields.push(channel.to_string());
                        fields.push(key.to_string());
                        fields.push(vel.to_string());
                    },

                    MidiMessage::NoteOn { key, vel } => {
                        fields.push("Note_on_c".to_string());
                        fields.push(channel.to_string());
                        fields.push(key.to_string());
                        fields.push(vel.to_string());
                    },

                    MidiMessage::ChannelAftertouch { vel } => {
                        fields.push("Channel_aftertouch_c".to_string());
                        fields.push(channel.to_string());
                        fields.push(vel.to_string());
                    },

                    MidiMessage::Aftertouch { key, vel } => {
                        fields.push("Poly_aftertouch_c".to_string());
                        fields.push(channel.to_string());
                        fields.push(key.to_string());
                        fields.push(vel.to_string());
                    },

                    MidiMessage::Controller { controller, value } => {
                        fields.push("Control_c".to_string());
                        fields.push(channel.to_string());
                        fields.push(controller.to_string());
                        fields.push(value.to_string());
                    },

                    MidiMessage::ProgramChange { program } => {
                        fields.push("Program_c".to_string());
                        fields.push(channel.to_string());
                        fields.push(program.to_string());
                    },

                    MidiMessage::PitchBend { bend } => {
                        fields.push("Pitch_bend_c".to_string());
                        fields.push(channel.to_string());
                        fields.push(bend.as_int().to_string());
                    },
                }
            },

            TrackEventKind::SysEx(data) => {
                fields.push("System_exclusive".to_string());
                fields.push(data.len().to_string());
                for b in data {
                    fields.push(b.to_string());
                }
            },

            _ => { },
        };

        // Record must have at least three fields to be valid MIDICSV
        if fields.len() >= 3 {
            writer.write_record(fields)?;
        }
    }

    writer.write_record(&[&track_num.to_string(), &abs_time.to_string(), "End_track"]).unwrap();

    Ok(())
}

fn write_header(writer: &mut Writer<io::Stdout>, header: &Header, track_count: usize) -> Result<(), Box<dyn Error>> {
    // The MIDICSV file does not have a regular header,
    // but the first line must be a file-specific header:

    let midi_format = match header.format {
        Format::SingleTrack => "0",
        Format::Parallel => "1",
        Format::Sequential => "2"
    };

    let midi_timing = match header.timing {
        Timing::Metrical(ticks) => ticks.to_string(),
        _ => "unknown".to_string(),
    };

    writer.write_record(&["0", "0", "Header", midi_format, &track_count.to_string(), &midi_timing])?;

    Ok(())
}
