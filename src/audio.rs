use log::{debug,trace};
use alsa::{Direction, ValueOr, Result};
use alsa::pcm::{HwParams, PCM};

fn init_playback_device(device_name: &str) -> Result<()> {
    trace!("init_playback_device()");
    let period_size = 32;
    let pcm = PCM::new(device_name, Direction::Playback, false)?;
    let hwp = HwParams::any(&pcm)?;
    hwp.set_channels(2)?;
    hwp.set_rate_resample(false)?;
    hwp.set_rate(44100, ValueOr::Nearest)?;
    hwp.set_format(alsa::pcm::Format::S32LE)?;
    hwp.set_access(alsa::pcm::Access::RWInterleaved)?;
    hwp.set_periods(16, ValueOr::Nearest)?;
    hwp.set_period_size_near(period_size, ValueOr::Nearest)?;
    pcm.hw_params(&hwp)?;
    let swp = pcm.sw_params_current()?;
    swp.set_start_threshold(0x7fffffff)?;
    swp.set_avail_min(period_size)?;
    swp.set_tstamp_mode(true)?;
    swp.set_tstamp_type(alsa::pcm::TstampType::MonotonicRaw)?;
    pcm.sw_params(&swp)?;

    let mut output = alsa::Output::buffer_open()?;
    pcm.dump(&mut output)?;
    debug!("{}", output);
    Ok(())
}

fn init_capture_device(device_name: &str) -> Result<()> {
    trace!("init_capture_device()");
    let period_size = 32;
    let pcm = PCM::new(device_name, Direction::Capture, false)?;
    let hwp = HwParams::any(&pcm)?;
    hwp.set_channels(2)?;
    hwp.set_rate_resample(false)?;
    hwp.set_rate(44100, ValueOr::Nearest)?;
    hwp.set_format(alsa::pcm::Format::S32LE)?;
    hwp.set_access(alsa::pcm::Access::RWInterleaved)?;
    hwp.set_periods(16, ValueOr::Nearest)?;     // ここだけplaybackと違う
    hwp.set_period_size_near(period_size, ValueOr::Nearest)?;
    pcm.hw_params(&hwp)?;
    let swp = pcm.sw_params_current()?;
    swp.set_start_threshold(0x7fffffff)?;
    swp.set_avail_min(period_size)?;
    swp.set_tstamp_mode(true)?;
    swp.set_tstamp_type(alsa::pcm::TstampType::MonotonicRaw)?;
    pcm.sw_params(&swp)?;

    let mut output = alsa::Output::buffer_open()?;
    pcm.dump(&mut output)?;
    debug!("{}", output);
    Ok(())
}

pub fn init() -> Result<()> {
    trace!("init()");
    //let device_name = "hw:CARD=U22,DEV=0";
    let device_name = "default";
    init_playback_device(device_name)?;
    init_capture_device(device_name)?;
    
    Ok(())
}
