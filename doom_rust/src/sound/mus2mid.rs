//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
// Copyright(C) 2006 Ben Ryves 2006
//
// mus2mid - Convert MUS file to single track, type 0 MIDI file.
//
// Original: mus2mid.h / mus2mid.c

use std::io::{Read, Seek, SeekFrom, Write};

use super::memio::{MemFileRead, MemFileWrite};

const NUM_CHANNELS: usize = 16;
const MIDI_PERCUSSION_CHAN: u8 = 9;
const MUS_PERCUSSION_CHAN: u8 = 15;

#[repr(u8)]
enum MusEvent {
    ReleaseKey = 0x00,
    PressKey = 0x10,
    PitchWheel = 0x20,
    SystemEvent = 0x30,
    ChangeController = 0x40,
    ScoreEnd = 0x60,
}

#[repr(u8)]
enum MidiEvent {
    ReleaseKey = 0x80,
    PressKey = 0x90,
    ChangeController = 0xB0,
    ChangePatch = 0xC0,
    PitchWheel = 0xE0,
}

#[repr(C)]
struct MusHeader {
    id: [u8; 4],
    scorelength: u16,
    scorestart: u16,
    primarychannels: u16,
    secondarychannels: u16,
    instrumentcount: u16,
}

static CONTROLLER_MAP: [u8; 15] = [
    0x00, 0x20, 0x01, 0x07, 0x0A, 0x0B, 0x5B, 0x5D, 0x40, 0x43, 0x78, 0x7B, 0x7E, 0x7F, 0x79,
];

static MIDI_HEADER: [u8; 22] = [
    b'M', b'T', b'h', b'd', 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x46, b'M',
    b'T', b'r', b'k', 0x00, 0x00, 0x00, 0x00,
];

/// Convert MUS bytes to MIDI bytes. Returns Ok(Vec<u8>) on success, Err on failure.
pub fn mus2mid(mus_input: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut mus = MemFileRead::new(mus_input);
    let mut midi = MemFileWrite::new();

    let mut channel_map: [i32; NUM_CHANNELS] = [-1; NUM_CHANNELS];
    let mut channel_velocities: [u8; NUM_CHANNELS] = [127; NUM_CHANNELS];
    let mut queued_time: u32 = 0;
    let mut track_size: u32 = 0;

    let header = read_mus_header(&mut mus)?;

    if header.id[0] != b'M' || header.id[1] != b'U' || header.id[2] != b'S' || header.id[3] != 0x1A {
        return Err("Invalid MUS header");
    }

    mus.seek(SeekFrom::Start(header.scorestart as u64))
        .map_err(|_| "Seek failed")?;

    midi.write_all(&MIDI_HEADER).map_err(|_| "Write failed")?;

    let mut hit_score_end = false;

    while !hit_score_end {
        loop {
            let mut event_descriptor = [0u8; 1];
            mus.read_exact(&mut event_descriptor).map_err(|_| "Read failed")?;

            let channel = (event_descriptor[0] & 0x0F) as usize;
            let midi_channel = get_midi_channel(
                channel as u8,
                &mut channel_map,
                &mut midi,
                &mut track_size,
                &mut queued_time,
            )?;
            let event = event_descriptor[0] & 0x70;

            match event {
                0x00 => {
                    let mut key = [0u8; 1];
                    mus.read_exact(&mut key).map_err(|_| "Read failed")?;
                    write_release_key(midi_channel, key[0], &mut midi, &mut track_size, &mut queued_time)?;
                }
                0x10 => {
                    let mut key = [0u8; 1];
                    mus.read_exact(&mut key).map_err(|_| "Read failed")?;
                    if (key[0] & 0x80) != 0 {
                        let mut vel = [0u8; 1];
                        mus.read_exact(&mut vel).map_err(|_| "Read failed")?;
                        channel_velocities[midi_channel as usize] = vel[0] & 0x7F;
                    }
                    write_press_key(
                        midi_channel,
                        key[0],
                        channel_velocities[midi_channel as usize],
                        &mut midi,
                        &mut track_size,
                        &mut queued_time,
                    )?;
                }
                0x20 => {
                    let mut key = [0u8; 1];
                    mus.read_exact(&mut key).map_err(|_| "Read failed")?;
                    write_pitch_wheel(midi_channel, (key[0] as i16) * 64, &mut midi, &mut track_size, &mut queued_time)?;
                }
                0x30 => {
                    let mut ctrl = [0u8; 1];
                    mus.read_exact(&mut ctrl).map_err(|_| "Read failed")?;
                    let cn = ctrl[0];
                    if cn < 10 || cn > 14 {
                        return Err("Invalid controller");
                    }
                    write_change_controller_valueless(
                        midi_channel,
                        CONTROLLER_MAP[cn as usize],
                        &mut midi,
                        &mut track_size,
                        &mut queued_time,
                    )?;
                }
                0x40 => {
                    let mut ctrl = [0u8; 1];
                    let mut val = [0u8; 1];
                    mus.read_exact(&mut ctrl).map_err(|_| "Read failed")?;
                    mus.read_exact(&mut val).map_err(|_| "Read failed")?;
                    if ctrl[0] == 0 {
                        write_change_patch(midi_channel, val[0], &mut midi, &mut track_size, &mut queued_time)?;
                    } else if ctrl[0] >= 1 && ctrl[0] <= 9 {
                        write_change_controller_valued(
                            midi_channel,
                            CONTROLLER_MAP[ctrl[0] as usize],
                            val[0],
                            &mut midi,
                            &mut track_size,
                            &mut queued_time,
                        )?;
                    } else {
                        return Err("Invalid controller");
                    }
                }
                0x60 => hit_score_end = true,
                _ => return Err("Unknown MUS event"),
            }

            if (event_descriptor[0] & 0x80) != 0 {
                break;
            }
        }

        if !hit_score_end {
            let mut timedelay: u32 = 0;
            loop {
                let mut working = [0u8; 1];
                mus.read_exact(&mut working).map_err(|_| "Read failed")?;
                timedelay = timedelay * 128 + (working[0] & 0x7F) as u32;
                if (working[0] & 0x80) == 0 {
                    break;
                }
            }
            queued_time += timedelay;
        }
    }

    write_end_track(&mut midi, &mut track_size, &mut queued_time)?;

    let mut buf = midi.into_buf();
    let pos = 18;
    if buf.len() >= pos + 4 {
        buf[pos] = (track_size >> 24) as u8;
        buf[pos + 1] = (track_size >> 16) as u8;
        buf[pos + 2] = (track_size >> 8) as u8;
        buf[pos + 3] = track_size as u8;
    }

    Ok(buf)
}

fn read_mus_header<R: Read>(r: &mut R) -> Result<MusHeader, &'static str> {
    let mut id = [0u8; 4];
    r.read_exact(&mut id).map_err(|_| "Read header failed")?;

    let mut scorelength = [0u8; 2];
    r.read_exact(&mut scorelength).map_err(|_| "Read header failed")?;
    let scorelength = u16::from_le_bytes(scorelength);

    let mut scorestart = [0u8; 2];
    r.read_exact(&mut scorestart).map_err(|_| "Read header failed")?;
    let scorestart = u16::from_le_bytes(scorestart);

    let mut primarychannels = [0u8; 2];
    r.read_exact(&mut primarychannels).map_err(|_| "Read header failed")?;
    let primarychannels = u16::from_le_bytes(primarychannels);

    let mut secondarychannels = [0u8; 2];
    r.read_exact(&mut secondarychannels).map_err(|_| "Read header failed")?;
    let secondarychannels = u16::from_le_bytes(secondarychannels);

    let mut instrumentcount = [0u8; 2];
    r.read_exact(&mut instrumentcount).map_err(|_| "Read header failed")?;
    let instrumentcount = u16::from_le_bytes(instrumentcount);

    Ok(MusHeader {
        id,
        scorelength,
        scorestart,
        primarychannels,
        secondarychannels,
        instrumentcount,
    })
}

fn write_time<W: Write>(
    time: u32,
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    let mut buffer = time & 0x7F;
    let mut t = time >> 7;
    while t != 0 {
        buffer = (buffer << 8) | ((t & 0x7F) | 0x80);
        t >>= 7;
    }

    loop {
        let writeval = (buffer & 0xFF) as u8;
        out.write_all(&[writeval]).map_err(|_| "Write failed")?;
        *track_size += 1;
        if (buffer & 0x80) != 0 {
            buffer >>= 8;
        } else {
            *queued_time = 0;
            break;
        }
    }
    Ok(())
}

fn write_end_track<W: Write>(
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    write_time(*queued_time, out, track_size, queued_time)?;
    out.write_all(&[0xFF, 0x2F, 0x00]).map_err(|_| "Write failed")?;
    *track_size += 3;
    Ok(())
}

fn write_press_key<W: Write>(
    channel: u8,
    key: u8,
    velocity: u8,
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    write_time(*queued_time, out, track_size, queued_time)?;
    out.write_all(&[MidiEvent::PressKey as u8 | channel, key & 0x7F, velocity & 0x7F])
        .map_err(|_| "Write failed")?;
    *track_size += 3;
    Ok(())
}

fn write_release_key<W: Write>(
    channel: u8,
    key: u8,
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    write_time(*queued_time, out, track_size, queued_time)?;
    out.write_all(&[MidiEvent::ReleaseKey as u8 | channel, key & 0x7F, 0])
        .map_err(|_| "Write failed")?;
    *track_size += 3;
    Ok(())
}

fn write_pitch_wheel<W: Write>(
    channel: u8,
    wheel: i16,
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    write_time(*queued_time, out, track_size, queued_time)?;
    out.write_all(&[
        MidiEvent::PitchWheel as u8 | channel,
        (wheel & 0x7F) as u8,
        ((wheel >> 7) & 0x7F) as u8,
    ])
    .map_err(|_| "Write failed")?;
    *track_size += 3;
    Ok(())
}

fn write_change_patch<W: Write>(
    channel: u8,
    patch: u8,
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    write_time(*queued_time, out, track_size, queued_time)?;
    out.write_all(&[MidiEvent::ChangePatch as u8 | channel, patch & 0x7F])
        .map_err(|_| "Write failed")?;
    *track_size += 2;
    Ok(())
}

fn write_change_controller_valued<W: Write>(
    channel: u8,
    control: u8,
    value: u8,
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    write_time(*queued_time, out, track_size, queued_time)?;
    let val = if (value & 0x80) != 0 { 0x7F } else { value };
    out.write_all(&[
        MidiEvent::ChangeController as u8 | channel,
        control & 0x7F,
        val,
    ])
    .map_err(|_| "Write failed")?;
    *track_size += 3;
    Ok(())
}

fn write_change_controller_valueless<W: Write>(
    channel: u8,
    control: u8,
    out: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<(), &'static str> {
    write_change_controller_valued(channel, control, 0, out, track_size, queued_time)
}

fn allocate_midi_channel(channel_map: &[i32; NUM_CHANNELS]) -> i32 {
    let max = channel_map.iter().copied().max().unwrap_or(-1);
    let mut result = max + 1;
    if result == MIDI_PERCUSSION_CHAN as i32 {
        result += 1;
    }
    result
}

fn get_midi_channel<W: Write>(
    mus_channel: u8,
    channel_map: &mut [i32; NUM_CHANNELS],
    midi_output: &mut W,
    track_size: &mut u32,
    queued_time: &mut u32,
) -> Result<u8, &'static str> {
    if mus_channel == MUS_PERCUSSION_CHAN {
        return Ok(MIDI_PERCUSSION_CHAN);
    }
    let idx = mus_channel as usize;
    if channel_map[idx] == -1 {
        channel_map[idx] = allocate_midi_channel(channel_map);
        write_change_controller_valueless(
            channel_map[idx] as u8,
            0x7b,
            midi_output,
            track_size,
            queued_time,
        )?;
    }
    Ok(channel_map[idx] as u8)
}
